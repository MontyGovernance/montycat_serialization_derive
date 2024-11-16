# Montycat Serialization Derive

`montycat_serialization_derive` is a procedural macro library designed to make binary serialization and deserialization in Rust seamless and efficient. Built on top of the `serde` and `rmp-serde` ecosystems, it provides the tools needed to serialize and deserialize structs into compact MessagePack (binary) formats.

## Features

- Automatically derive binary serialization and deserialization methods for your structs.
- Utilizes the robust `serde` framework for flexible data representation.
- Ensures compatibility with the lightweight and efficient MessagePack (`rmp-serde`).
- Simple integration with just a single macro.

## Requirements

This crate relies on the following dependencies:

- `serde` (with `derive` feature enabled)
- `rmp-serde` for MessagePack serialization/deserialization

Make sure to include these dependencies in your `Cargo.toml`.

## Installation

Add the crate to your project's dependencies in `Cargo.toml`:

```toml
[dependencies]
montycat_serialization_derive = "0.1.1"
serde = { version = "1.0", features = ["derive"] }
rmp-serde = "1.3.0"
```

## Usage

Import and setup your struct

```rust
use serde::{Serialize, Deserialize};
use montycat_serialization_derive::BinaryConvert;

#[derive(Serialize, Deserialize, BinaryConvert, Default)]
struct MyStruct {
    id: u32,
    name: String,
}
```

Use methods from the crate

```rust
fn main() {
    // Create an instance of the struct
    let original = MyStruct {
        id: 42,
        name: "Monty".to_string(),
    };

    // Serialize the struct to binary
    let bytes = original.convert_to_bytes();
    println!("Serialized bytes: {:?}", bytes);

    // Deserialize the binary back into the struct
    let deserialized = MyStruct::convert_from_bytes(&bytes);
    println!("Deserialized struct: id = {}, name = {}", deserialized.id, deserialized.name);

    // Verify correctness
    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.name, deserialized.name);
}
```

## Error Handling

Serialization (convert_to_bytes): If serialization fails, an empty Vec<u8> is returned.
Deserialization (convert_from_bytes): If deserialization fails, the method returns a default instance of the struct. For this to work, the struct must implement the Default trait.

## Notes

The library uses the MessagePack format, making it compact and efficient for binary data storage or transmission.
For additional control over serialization, you can use serde attributes (e.g., #[serde(rename = "...")], #[serde(skip)]).