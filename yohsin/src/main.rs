#![allow(unused_variables)]

use std::sync::Arc;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::Mutex;
use yohsin::order_struct::DailyBlotterData;

// Use multi-threaded runtime
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data
    let original_data = DailyBlotterData::load_from_file("../data_baker/data/dummy_data_5.csv")?;
    let start = Instant::now();

    let memo_file = "memo.txt";
    let memo_content = tokio::fs::read_to_string(memo_file)
        .await
        .unwrap_or_default();

    // Infer the last written index from the memo file
    let last_written_idx: usize = memo_content.trim().parse().unwrap_or(0); // Default to 0 if memo file is empty or invalid

    // Calculate the number of objects to process
    let n_objects = original_data.len() - last_written_idx;
    let object_size = std::mem::size_of::<DailyBlotterData>();
    let data_size = n_objects * object_size;

    // Calculate the offset for the remaining data
    let offset = last_written_idx * object_size;

    // Use Arc to share data without cloning (only for the remaining data)
    let data_bytes: Arc<[u8]> = Arc::from(unsafe {
        std::slice::from_raw_parts(
            original_data.as_ptr().add(last_written_idx) as *const u8,
            data_size,
        )
    });

    // Open a file for writing
    let file = File::create("dump").await?;
    let writer = BufWriter::new(file);
    let shared_writer = Arc::new(Mutex::new(writer));

    // Write the number of records to the file
    shared_writer
        .lock()
        .await
        .write_all(&(n_objects as u64).to_le_bytes())
        .await?;

    // Calculate the number of threads to use
    let num_threads = std::thread::available_parallelism()?.get();
    let chunk_size = (n_objects + num_threads - 1) / num_threads; // Divide work evenly

    // Spawn tasks for parallel writing
    let mut handles = Vec::new();
    for thread_id in 0..num_threads {
        let data_bytes = Arc::clone(&data_bytes);
        let shared_writer = Arc::clone(&shared_writer);

        // Calculate the start and end indices for this thread's chunk
        let start_idx = thread_id * chunk_size * object_size;
        let end_idx = std::cmp::min(start_idx + chunk_size * object_size, data_size);

        // Calculate the file offset for this thread's chunk
        let file_offset = 8 + offset + start_idx; // 8 bytes for the number of records

        // Spawn a Tokio task to write this chunk's data to the file
        let handle = tokio::spawn(async move {
            let data_slice = &data_bytes[start_idx..end_idx];

            // Open the file for writing
            let mut file = File::create("dump").await.unwrap();
            file.seek(std::io::SeekFrom::Start(file_offset as u64))
                .await
                .unwrap();

            // Write the data to the correct position
            file.write_all(data_slice).await.unwrap();

            // Update the memo file to mark this chunk as completed
            let mut memo_file = File::create("memo.txt").await.unwrap();
            memo_file
                .write_all(
                    (last_written_idx + (end_idx / object_size))
                        .to_string()
                        .as_bytes(),
                )
                .await
                .unwrap();
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }

    // Clear the memo file after all threads have completed
    tokio::fs::write(memo_file, "").await?;

    // Ensure all data is flushed to the file
    let mut writer = shared_writer.lock().await;
    writer.flush().await?;

    // Calculate elapsed time
    println!("Time elapsed (serialize+dump) : {:?}", start.elapsed());

    // --- Retrieve:
    // Open the file for reading
    let file = File::open("dump").await?;
    let mut file = BufReader::new(file);

    // Read the number of records
    let mut num_records_bytes = [0u8; 8];
    file.read_exact(&mut num_records_bytes).await?;
    let num_records = u64::from_le_bytes(num_records_bytes) as usize;

    // Read the binary data into a buffer
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data).await?;

    // Debug: Print lengths
    println!("Number of records: {}", num_records);

    // Reconstruct the Vec<DailyBlotterData>
    let mut retrieved_data = Vec::with_capacity(num_records);
    for i in 0..num_records {
        let offset = i * object_size;
        if offset + object_size > binary_data.len() {
            panic!(
                "Invalid offset: {} (binary data length: {})",
                offset,
                binary_data.len()
            );
        }

        let record_ptr =
            binary_data[offset..offset + object_size].as_ptr() as *const DailyBlotterData;
        let record = unsafe { &*record_ptr }; // Safe because we know the layout
        retrieved_data.push((*record).clone()); // Clone the record to get an owned value
    }

    // Write the retrieved data to a CSV file for verification
    DailyBlotterData::write_to_file("../data_baker/data/written_file_5.csv", &retrieved_data)?;

    // Compare the sorted original and retrieved data
    if original_data == retrieved_data {
        println!("The original and retrieved data are the same.");
    } else {
        println!("The original and retrieved data are NOT the same.");
    }

    Ok(())
}
