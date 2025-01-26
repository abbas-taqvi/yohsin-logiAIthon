use std::sync::Arc;
use yohsin::order_struct::DailyBlotterData;
use yohsin::serialize::{deserialize_range_from_file, serialize_to_file};

#[tokio::test]
async fn test_deserialize_range() -> Result<(), Box<dyn std::error::Error>> {
    // Load the dummy data
    let original_data = DailyBlotterData::load_from_file("../data_baker/data/dummy_data_5.csv")?;

    // Serialize the data to a file
    let file_path = Arc::new("test_dump.bin".to_string());
    let memo_file = Arc::new("test_memo.txt".to_string());
    serialize_to_file(
        Arc::clone(&original_data),
        file_path.clone(),
        memo_file.clone(),
    )
    .await?;

    // Define the range to deserialize
    let range = 50..100;

    // Deserialize the range from the file
    let retrieved_range_data =
        deserialize_range_from_file::<DailyBlotterData>(file_path.clone(), range.clone()).await?;

    // Convert the original data to a Vec and slice it
    let original_slice = original_data[range].to_vec();

    // Compare the sliced original data and the retrieved range data
    assert_eq!(
        original_slice,
        retrieved_range_data.to_vec(),
        "Original slice and retrieved range data do not match"
    );

    // Clean up test files
    tokio::fs::remove_file(&*file_path).await?;
    tokio::fs::remove_file(&*memo_file).await?;

    Ok(())
}
