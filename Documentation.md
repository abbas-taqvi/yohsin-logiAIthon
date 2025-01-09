# Project: Daily Blotter Data Serialization and Deserialization

## Objective

To efficiently serialize `DailyBlotterData` records into binary format and deserialize them back while preserving data integrity. This ensures data storage and retrieval efficiency in a high-performance environment.

## Modules Overview

### 1. `main.rs`

#### Purpose:
- Acts as the **entry point**.
- Handles the **timing** and overall execution flow for data serialization and deserialization.

#### Key Steps:
1. Load `DailyBlotterData` from a CSV file.
2. Serialize the data into binary format, including a **lookup table** for quick access.
3. Calculate the byte offsets of object instances.
4. Write serialization data from byte computation to a file.
5. Deserialize the data from the binary file.
6. Validate the integrity of the deserialized data against the original.

---

### 2. `lib.rs`

#### Purpose:
- Houses **modular components** of the project.

#### Structure:
- Includes submodules:
  1. `order_generated`: Automatically generated code for FlatBuffer support.
  2. `order_struct`: Defines the `DailyBlotterData` structure and associated methods.

---

### 3. `order_struct.rs`

#### Purpose:
- Defines the `DailyBlotterData` structure.
- Implements functionality to load data from a CSV file.

#### Key Features:
- Supports parsing and validating fields such as `orderdate`, `price`, `action`, and `side`.
- Method: `load_from_file` to populate `Vec<DailyBlotterData>` from a CSV.

---

## Data Handling Workflow

1. **Load Data**:
   - `DailyBlotterData` records are read from a CSV file using `load_from_file` in `order_struct.rs`.

2. **Serialization**:
   - Binary serialization involves:
     - Converting the vector of `DailyBlotterData` into raw bytes.
     - Generating a **lookup table** containing byte offsets for each record.

3. **Storage**:
   - Both the serialized data and lookup table are written into a single binary file (`dump_with_lookup`).

4. **Deserialization**:
   - Reads back the binary file.
   - Reconstructs the original `DailyBlotterData` vector using the lookup table and raw bytes.

5. **Validation**:
   - Compares the original and deserialized vectors to ensure data integrity.

---

## Technology Stack

- **Language**: Rust
- **Libraries**:
  - `std::fs`: For file operations.
  - `std::io`: For reading and writing data.

---

## Key Strengths

- **Performance**:
  - Binary serialization is faster and more compact compared to traditional formats like JSON or XML.
- **Data Integrity**:
  - Ensures that serialized and deserialized data are identical.