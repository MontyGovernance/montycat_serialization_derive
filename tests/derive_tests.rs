use montycat_serialization_derive::{BinaryConvert, RuntimeSchema};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Dummy trait
pub trait RuntimeSchema {
    fn pointer_and_timestamp_fields(&self) -> Vec<(&'static str, &'static str)>;
    fn field_names_and_types(&self) -> Vec<(&'static str, &'static str)>;
    fn schema_params() -> (std::collections::HashMap<&'static str, &'static str>, &'static str);
}

// Dummy structs to simulate types used in the macros
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Pointer(pub u64);

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Timestamp(pub u64);

/// Test struct for both derives
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, BinaryConvert, RuntimeSchema)]
pub struct MyStruct {
    pub id: Pointer,
    pub timestamp: Timestamp,
    pub name: String,
    pub value: u32,
}

#[test]
fn test_binary_convert_roundtrip() {
    let original = MyStruct {
        id: Pointer(123),
        timestamp: Timestamp(987),
        name: "MontyCat".into(),
        value: 42,
    };

    // Serialize to bytes
    let bytes = original.convert_to_bytes();
    assert!(!bytes.is_empty(), "Serialized bytes should not be empty");

    // Deserialize from bytes
    let decoded = MyStruct::convert_from_bytes(&bytes);
    assert_eq!(original, decoded, "Roundtrip BinaryConvert should preserve data");

    // Option-based serialization
    let opt_bytes = original.convert_to_bytes_populate_option();
    assert!(opt_bytes.is_some(), "Option-based serialization should succeed");

    // Option-based deserialization
    let opt_decoded = MyStruct::convert_from_bytes_populate_option(&bytes);
    assert!(opt_decoded.is_some(), "Option-based deserialization should succeed");
}

#[test]
fn test_runtime_schema_field_types() {
    let s = MyStruct::default();

    let pointer_ts_fields = s.pointer_and_timestamp_fields();
    assert_eq!(
        pointer_ts_fields,
        vec![("id", "Pointer"), ("timestamp", "Timestamp")]
    );

    let all_fields = s.field_names_and_types();
    assert_eq!(
        all_fields,
        vec![
            ("id", "Pointer"),
            ("timestamp", "Timestamp"),
            ("name", "String"),
            ("value", "u32")
        ]
    );
}

#[test]
fn test_runtime_schema_map_and_name() {
    let (schema_map, struct_name) = MyStruct::schema_params();
    assert_eq!(struct_name, "MyStruct");
    assert_eq!(schema_map.get("name"), Some(&"String"));
    assert_eq!(schema_map.get("value"), Some(&"u32"));
    assert_eq!(schema_map.get("id"), Some(&"Pointer"));
}
