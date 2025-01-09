use std::fs;
use std::io::Write;
use regex::{Regex, RegexBuilder};

fn main() -> std::io::Result<()> {
    // 1. Read a file containing Rust structs
    let input_file = "input.rs"; // Replace with your actual file path
    let input_data = fs::read_to_string(input_file)?;

    // 2. Build a regex to capture structs across multiple lines
    //    - group 1 = struct name
    //    - group 2 = struct body (the fields)
    let struct_regex = RegexBuilder::new(r"struct\s+(\w+)\s*\{\s*([^}]*)\s*\}")
        .dot_matches_new_line(true)
        .build()
        .expect("Failed to compile struct regex");

    // 3. Build a regex to capture fields (field_name: field_type).
    //    This is simplistic; it won't handle very complex field types.
    let field_regex = Regex::new(r"(\w+):\s*([\w<>\[\]]+),?")
        .expect("Failed to compile field regex");

    // 4. Create (if necessary) a directory for generated FlatBuffers files
    let output_folder = "createdFbs";
    fs::create_dir_all(output_folder)?;

    // 5. Iterate over each struct definition
    for captures in struct_regex.captures_iter(&input_data) {
        let struct_name = captures.get(1).unwrap().as_str();
        let struct_body = captures.get(2).unwrap().as_str();

        println!("Found struct: {}", struct_name);

        // 6. For each struct, parse out fields and map their types
        let mut fields = Vec::new();
        for field_caps in field_regex.captures_iter(struct_body) {
            let field_name = field_caps.get(1).unwrap().as_str();
            let rust_type  = field_caps.get(2).unwrap().as_str();
            let fbs_type   = map_rust_type_to_fbs(rust_type);
            fields.push((field_name.to_string(), fbs_type));
        }

        // 7. Generate the FlatBuffers schema
        let fbs_schema = generate_fbs_schema(struct_name, &fields);

        // 8. Write the schema to a file named "<struct_name>.fbs" inside createdFbs/
        let output_file = format!("{}/{}.fbs", output_folder, struct_name);
        let mut file = fs::File::create(&output_file)?;
        file.write_all(fbs_schema.as_bytes())?;

        println!("Wrote FlatBuffers schema for '{}' to '{}'.", struct_name, output_file);
    }

    Ok(())
}

// Make the function public so that it's accessible for testing
pub fn map_rust_type_to_fbs(rust_type: &str) -> String {
    match rust_type {
        "i32"    => "int32".to_string(),
        "i64"    => "int64".to_string(),
        "f32"    => "float".to_string(),
        "f64"    => "double".to_string(),
        "String" => "string".to_string(),
        _        => format!("Unknown_{}", rust_type), // fallback for unrecognized types
    }
}

// Generate a FlatBuffers schema for a single struct.
fn generate_fbs_schema(struct_name: &str, fields: &[(String, String)]) -> String {
    let mut schema = format!("table {} {{\n", struct_name);
    for (field_name, field_type) in fields {
        // In FlatBuffers, fields typically end with a semicolon
        schema.push_str(&format!("  {}: {};\n", field_name, field_type));
    }
    schema.push_str("}\n\n");
    schema.push_str(&format!("root_type {};\n", struct_name));
    schema
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for mapping Rust types to FlatBuffers types
    #[test]
    fn test_map_rust_type_to_fbs() {
        assert_eq!(map_rust_type_to_fbs("i32"), "int32");
        assert_eq!(map_rust_type_to_fbs("i64"), "int64");
        assert_eq!(map_rust_type_to_fbs("f32"), "float");
        assert_eq!(map_rust_type_to_fbs("f64"), "double");
        assert_eq!(map_rust_type_to_fbs("String"), "string");
        assert_eq!(map_rust_type_to_fbs("UnknownType"), "Unknown_UnknownType");
    }


    // Test case for generating a FlatBuffers schema
    #[test]
    fn test_generate_fbs_schema() {
        let fields = vec![
            ("id".to_string(), "int32".to_string()),
            ("name".to_string(), "string".to_string()),
            ("email".to_string(), "string".to_string()),
        ];

        let expected_schema = "table User {\n  id: int32;\n  name: string;\n  email: string;\n}\n\nroot_type User;\n";
        let schema = generate_fbs_schema("User", &fields);
        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_write_fbs_schema() {
        let output_folder = "createdFbs";
        fs::create_dir_all(output_folder).unwrap();

        let fields = vec![
            ("id".to_string(), "int32".to_string()),
            ("name".to_string(), "string".to_string()),
            ("email".to_string(), "string".to_string()),
        ];

        let fbs_schema = generate_fbs_schema("User", &fields);
        let output_file = format!("{}/{}.fbs", output_folder, "User");

        let mut file = fs::File::create(&output_file).unwrap();
        file.write_all(fbs_schema.as_bytes()).unwrap();

        // Verify file is created
        assert!(fs::metadata(output_file).is_ok());
    }

    // You can add more tests here as needed
}
