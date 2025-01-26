use std::sync::Arc;
use std::time::Instant;
use yohsin::order_struct::DailyBlotterData;
use yohsin::serialize::{deserialize_from_file, deserialize_range_from_file, serialize_to_file};

// Use multi-threaded runtime
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data
    let original_data = DailyBlotterData::load_from_file("../data_baker/data/dummy_data_5.csv")?;
    let start = Instant::now();

    // Serialize data to file
    let file_path = Arc::new("dump.bin".to_string());
    let memo_file = Arc::new("memo.txt".to_string());
    serialize_to_file(original_data, file_path.clone(), memo_file.clone()).await?;

    // Calculate elapsed time
    println!("Time elapsed (serialize+dump) : {:?}", start.elapsed());

    // Deserialize data from file
    let retrieved_data = deserialize_from_file::<DailyBlotterData>(file_path.clone()).await?;
    let retrieved_range =
        deserialize_range_from_file::<DailyBlotterData>(file_path, 50..100).await?;

    // Write the retrieved data to a CSV file for verification
    DailyBlotterData::write_to_file("../data_baker/data/written_file_5.csv", &retrieved_data)?;
    DailyBlotterData::write_to_file("../data_baker/data/written_range_5.csv", &retrieved_range)?;

    Ok(())
}
