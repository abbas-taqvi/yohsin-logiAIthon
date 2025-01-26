use std::sync::Arc;
use yohsin::order_struct::DailyBlotterData;
use yohsin::serialize::{deserialize_from_file, serialize_to_file};

#[tokio::test]
async fn test_serialize_deserialize() -> Result<(), Box<dyn std::error::Error>> {
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

    // Deserialize the data from the file
    let retrieved_data = deserialize_from_file::<DailyBlotterData>(file_path.clone()).await?;

    // Compare the original and retrieved data
    assert_eq!(
        *original_data, *retrieved_data,
        "Original and retrieved data do not match"
    );

    // Clean up test files
    tokio::fs::remove_file(&*file_path).await?;
    tokio::fs::remove_file(&*memo_file).await?;

    Ok(())
}
