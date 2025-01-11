use std::sync::Arc;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::Mutex;
use yohsin::order_struct::DailyBlotterData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_data = DailyBlotterData::load_from_file("../data_baker/data/dummy_data_6.csv")?;
    let start = Instant::now();

    let object_size = std::mem::size_of::<DailyBlotterData>();
    let n_objects = original_data.len();
    let data_size = n_objects * object_size;

    let data_bytes: Arc<[u8]> = Arc::from(
        unsafe { std::slice::from_raw_parts(original_data.as_ptr() as *const u8, data_size) }
            .to_vec(),
    );

    // Open a shared file for writing
    let file = File::create("dump").await?;
    let writer = BufWriter::new(file);
    let shared_writer: Arc<Mutex<BufWriter<File>>> = Arc::new(Mutex::new(writer));

    // Create a lookup table (offsets for each record)
    let mut lookup_table = Vec::new();
    let mut offset = 0;
    for _ in 0..n_objects {
        lookup_table.push(offset);
        offset += object_size;
    }

    // Write the number of records to the file
    shared_writer
        .lock()
        .await
        .write_all(&(n_objects as u64).to_le_bytes())
        .await?;

    // Write the lookup table to the file
    for &off in &lookup_table {
        shared_writer
            .lock()
            .await
            .write_all(&off.to_le_bytes())
            .await?;
    }

    // Spawn one task per object
    let mut handles = Vec::new();
    for i in 0..n_objects {
        let shared_writer = Arc::clone(&shared_writer);
        let data_bytes = Arc::clone(&data_bytes);

        // Spawn a task to write this object's data to the file
        let handle = tokio::spawn(async move {
            let start_idx = i * object_size;
            let end_idx = start_idx + object_size;
            let data_slice = &data_bytes[start_idx..end_idx];

            // Lock the shared writer and write the data
            let mut writer = shared_writer.lock().await;
            writer.write_all(data_slice).await?;
            Ok::<(), std::io::Error>(())
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await??;
    }

    // Ensure all data is flushed to the file
    let mut writer = shared_writer.lock().await;
    writer.flush().await?;

    // Calculate elapsed time
    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);

    println!("Binary data successfully written to 'dump' file.");

    // --- Retrieve:
    // Open the file for reading
    let file = File::open("dump").await?;
    let mut file = BufReader::new(file);

    // Read the number of records
    let mut num_records_bytes = [0u8; 8];
    file.read_exact(&mut num_records_bytes).await?;
    let num_records = u64::from_le_bytes(num_records_bytes) as usize;

    // Read the lookup table (offsets)
    let mut lookup_table = Vec::with_capacity(num_records);
    for _ in 0..num_records {
        let mut offset_bytes = [0u8; std::mem::size_of::<usize>()];
        file.read_exact(&mut offset_bytes).await?;
        let offset = usize::from_le_bytes(offset_bytes);
        lookup_table.push(offset);
    }

    // Read the binary data into a buffer
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data).await?;

    // Debug: Print lengths
    println!("Number of records: {}", num_records);

    // Reconstruct the Vec<DailyBlotterData>
    let mut retrieved_data = Vec::with_capacity(num_records);
    for &offset in &lookup_table {
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

    // Sort the retrieved data by a unique field (e.g., `id`)
    retrieved_data.sort_by_key(|record| record.id);

    // Sort the original data by the same unique field
    let mut sorted_original_data = original_data.clone();
    sorted_original_data.sort_by_key(|record| record.id);

    // Compare the sorted original and retrieved data
    if sorted_original_data == retrieved_data {
        println!("The original and retrieved data are the same.");
    } else {
        println!("The original and retrieved data are NOT the same.");
    }

    // Debug: Compare the first few sorted original and retrieved records
    for i in 0..10 {
        if sorted_original_data[i] != retrieved_data[i] {
            println!("Mismatch at index {}:", i);
            println!("Original: {:?}", sorted_original_data[i]);
            println!("Retrieved: {:?}", retrieved_data[i]);
        }
    }

    Ok(())
}

