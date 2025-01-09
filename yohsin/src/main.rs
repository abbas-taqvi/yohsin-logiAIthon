use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use std::slice;
use std::time::Instant;
use yohsin::order_struct::DailyBlotterData;

mod order_struct;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start timing
    let start = Instant::now();

    // Load data from a file into the struct
    let original_data = DailyBlotterData::load_from_file("../dataBaker/data/dummy_data_.csv")?;

    // Treat the Vec<DailyBlotterData> as binary data
    let data_ptr = original_data.as_ptr() as *const u8; // Pointer to the first element as raw bytes
    let data_size = original_data.len() * std::mem::size_of::<DailyBlotterData>(); // Total size in bytes

    // Create a slice of raw bytes from the Vec<DailyBlotterData>
    let data_bytes = unsafe { slice::from_raw_parts(data_ptr, data_size) };

    // Create a lookup table (e.g., offsets for each record)
    let mut lookup_table = Vec::new();
    let mut offset = 0;
    for _ in original_data.iter() {
        lookup_table.push(offset);
        offset += std::mem::size_of::<DailyBlotterData>();
    }

    // Write the binary data and lookup table to a file
    let mut file = BufWriter::new(File::create("dump_with_lookup")?);

    // Write the lookup table first (number of records + offsets)
    file.write_all(&(original_data.len() as u64).to_le_bytes())?; // Write number of records
    for &off in &lookup_table {
        file.write_all(&off.to_le_bytes())?; // Write each offset
    }

    // Write the binary data
    file.write_all(data_bytes)?;

    // Calculate elapsed time
    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);

    println!("Binary data and lookup table successfully written to 'dump_with_lookup' file.");

    // --- retrieve:
    // Open the file for reading
    let mut file = BufReader::new(File::open("dump_with_lookup")?);

    // Read the number of records
    let mut num_records_bytes = [0u8; 8];
    file.read_exact(&mut num_records_bytes)?;
    let num_records = u64::from_le_bytes(num_records_bytes) as usize;

    // Read the lookup table (offsets)
    let mut lookup_table = Vec::with_capacity(num_records);
    for _ in 0..num_records {
        let mut offset_bytes = [0u8; std::mem::size_of::<usize>()];
        file.read_exact(&mut offset_bytes)?;
        let offset = usize::from_le_bytes(offset_bytes);
        lookup_table.push(offset);
    }

    // Read the binary data into a buffer
    let mut binary_data = Vec::new();
    file.read_to_end(&mut binary_data)?;

    // Reconstruct the Vec<DailyBlotterData>
    let mut retrieved_data = Vec::with_capacity(num_records);
    for &offset in &lookup_table {
        let record_ptr = binary_data[offset..].as_ptr() as *const DailyBlotterData;
        let record = unsafe { &*record_ptr }; // Safe because we know the layout
        retrieved_data.push((*record).clone()); // Clone the record to get an owned value
    }

    println!("Successfully retrieved {} records.", retrieved_data.len());

    // Compare the original and retrieved data
    if original_data == retrieved_data {
        println!("The original and retrieved data are the same.");
    } else {
        println!("The original and retrieved data are NOT the same.");
    }

    Ok(())
}
