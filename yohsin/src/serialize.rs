#![allow(unused_variables)]

use std::ops::Range;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::Mutex;

pub async fn serialize_to_file<T>(
    data: Arc<[T]>,
    file_path: Arc<String>,
    memo_file: Arc<String>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Send + Sync + 'static,
{
    let memo_content = tokio::fs::read_to_string(&*memo_file)
        .await
        .unwrap_or_default();

    // Infer the last written index from the memo file
    let last_written_idx: usize = memo_content.trim().parse().unwrap_or(0); // Default to 0 if memo file is empty or invalid

    // Calculate the number of objects to process
    let n_objects = data.len() - last_written_idx;
    let object_size = std::mem::size_of::<T>();
    let data_size = n_objects * object_size;

    // Calculate the offset for the remaining data
    let offset = last_written_idx * object_size;

    // Use Arc to share data without cloning (only for the remaining data)
    let data_bytes: Arc<[u8]> = Arc::from(unsafe {
        std::slice::from_raw_parts(data.as_ptr().add(last_written_idx) as *const u8, data_size)
    });

    let file = File::create(&*file_path).await?;
    let writer = BufWriter::new(file);
    let shared_writer = Arc::new(Mutex::new(writer));

    // Write the number of records to the file
    shared_writer
        .lock()
        .await
        .write_all(&(n_objects as u64).to_le_bytes())
        .await?;

    let num_threads = std::thread::available_parallelism()?.get();
    let chunk_size = (n_objects + num_threads - 1) / num_threads;

    let mut handles = Vec::new();
    for thread_id in 0..num_threads {
        let data_bytes = Arc::clone(&data_bytes);
        let shared_writer = Arc::clone(&shared_writer);
        let file_path = Arc::clone(&file_path);
        let memo_file = Arc::clone(&memo_file);

        // Calculate this thread's start and end indices
        let start_idx = thread_id * chunk_size * object_size;
        let end_idx = std::cmp::min(start_idx + chunk_size * object_size, data_size);

        let file_offset = 8 + offset + start_idx;

        // Write to the file
        let handle = tokio::spawn(async move {
            let data_slice = &data_bytes[start_idx..end_idx];

            let mut file = File::create(&*file_path).await.unwrap();
            file.seek(std::io::SeekFrom::Start(file_offset as u64))
                .await
                .unwrap();

            file.write_all(data_slice).await.unwrap();

            // Mark this chunk as completed in memo
            let mut memo_file = File::create(&*memo_file).await.unwrap();
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

    for handle in handles {
        handle.await?;
    }

    // Empty the memo file as a sign of success
    tokio::fs::write(&*memo_file, "").await?;

    let mut writer = shared_writer.lock().await;
    writer.flush().await?;

    Ok(())
}

pub async fn deserialize_from_file<T>(
    file_path: Arc<String>,
) -> Result<Arc<[T]>, Box<dyn std::error::Error>>
where
    T: Clone + 'static,
{
    let file = File::open(&*file_path).await?;
    let mut file = BufReader::new(file);

    let mut num_records_bytes = [0u8; 8];
    file.read_exact(&mut num_records_bytes).await?;
    let num_records = u64::from_le_bytes(num_records_bytes) as usize;

    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data).await?;

    // Reconstruct the Arc<[T]>
    let object_size = std::mem::size_of::<T>();
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

        let record_ptr = binary_data[offset..offset + object_size].as_ptr() as *const T;
        let record = unsafe { &*record_ptr }; // Safe because we know the layout
        retrieved_data.push((*record).clone());
    }

    Ok(Arc::from(retrieved_data))
}

/// Deserializes a range of elements from a file into an Arc<[T]>.
pub async fn deserialize_range_from_file<T>(
    file_path: Arc<String>,
    range: Range<usize>,
) -> Result<Arc<[T]>, Box<dyn std::error::Error>>
where
    T: Clone + 'static,
{
    let file = File::open(&*file_path).await?;
    let mut file = BufReader::new(file);

    let mut num_records_bytes = [0u8; 8];
    file.read_exact(&mut num_records_bytes).await?;
    let num_records = u64::from_le_bytes(num_records_bytes) as usize;

    // Validate the range
    if range.end > num_records {
        return Err(format!(
            "Invalid range: {}..{} (file contains {} records)",
            range.start, range.end, num_records
        )
        .into());
    }

    // Calculate the seek position and scope
    let object_size = std::mem::size_of::<T>();
    let start_offset = 8 + range.start * object_size; // 8 bytes for the number of records
    let end_offset = 8 + range.end * object_size;

    file.seek(std::io::SeekFrom::Start(start_offset as u64))
        .await?;

    // Read the required data into a buffer
    let mut binary_data = vec![0u8; end_offset - start_offset];
    file.read_exact(&mut binary_data).await?;

    // Reconstruct the Arc<[T]> for the range
    let mut retrieved_data = Vec::with_capacity(range.len());
    for i in 0..range.len() {
        let offset = i * object_size;
        if offset + object_size > binary_data.len() {
            panic!(
                "Invalid offset: {} (binary data length: {})",
                offset,
                binary_data.len()
            );
        }

        let record_ptr = binary_data[offset..offset + object_size].as_ptr() as *const T;
        let record = unsafe { &*record_ptr }; // Safe because we know the layout
        retrieved_data.push((*record).clone()); // Clone the record to get an owned value
    }

    Ok(Arc::from(retrieved_data))
}
