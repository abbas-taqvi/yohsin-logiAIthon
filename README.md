# Project: High-Performance Data Serialization and Deserialization and Persistent Storage

## Objective

To efficiently serialize records of any type `T` into binary format and deserialize them back while preserving data integrity with functionality of range deserialization, fault-tolerance and zero-copy serialization. This ensures data storage and retrieval efficiency in a high-performance environment with no waste of resources.

## How to run and test

Install the cargo utility:
```rs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
restart the shell or source the config file (.bashrc/.zshrc etc.)

Now for running the project:
```rs
cd yohsin
cargo run --release
```

For running the tests
```rs
cd yohsin
cargo test
```

## Modules Overview

### 1. `main.rs`
- Acts as the **entry point**.
- Handles the **timing** and overall execution flow for data serialization and deserialization.

---

### 2. `serialize.rs`

- Contains the core logic for serialization and deserialization.
- Implements zero-copy serialization for efficient memory usage.
- Provides fault-tolerant serialization using a memo file to track progress.
- Supports range-based deserialization for efficient retrieval of specific data segments.

---

### 3. `lib.rs`
- Houses **modular components** of the project.

---

### 4. `order_struct.rs`

- Defines the `DailyBlotterData` structure, any other structure can also be
used with the with the two methods defined below.
- Implements functionality to load data from a CSV file; Method: `load_from_file`
- Provides a way to write the records to a CSV file; Method: `write_to_file`

## Data Handling Workflow

### Data Loading:
- Data is loaded from a CSV file into a structured format (DailyBlotterData or any other type T).

### Serialization:
- The data is serialized into a binary format and written to a file.
- A memo file tracks the progress of serialization for fault tolerance.

### Deserialization:
- The binary data is deserialized back into the original format.
- Supports full deserialization or range-based deserialization for specific segments.

### Verification:
- The deserialized data is compared with the original data to ensure integrity.
- Results are written to a CSV file for verification.

---

## Technology Stack

- **Language**: Rust
- **External Libraries**:
  - `tokio`: For providing async-runtime.
