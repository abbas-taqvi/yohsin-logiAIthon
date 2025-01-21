use crate::order_struct::DailyBlotterData;

pub fn serialize_to_file(
    data: Vec<DailyBlotterData>,
    filepath: String,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
