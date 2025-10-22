# 🦀 Montycat Serialization Derive

`montycat_serialization_derive` is a procedural macro library for seamless and efficient binary serialization and runtime schema introspection in Rust.
Built on top of `serde` and `rmp-serde`, it provides automatic MessagePack-based serialization and deserialization, as well as reflection-like runtime type checking with `RuntimeSchema`.

## ✨ Features

- 🧩 RuntimeSchema — inspect struct fields and types at runtime.
- ⚡ BinaryConvert — derive compact, high-speed binary serialization methods automatically.
- 💪 Powered by serde for flexible data representation.
- 📦 Compatible with rmp-serde (MessagePack format) for compact and efficient binary encoding.
- 🔧 Simple integration — add a single derive macro to your structs.

## Requirements

This crate relies on the following dependencies:

- `serde` (with `derive` feature enabled)
- `rmp-serde` for MessagePack serialization/deserialization

Make sure to include these dependencies in your `Cargo.toml`.

## Installation

Add the crate to your project's dependencies in `Cargo.toml`:

```toml
[dependencies]
montycat_serialization_derive = "0.1.6"
serde = { version = "1.0", features = ["derive"] }
rmp-serde = "1"
```

## Usage BinaryConvert

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

Then serialize and deserialize easily:

```rust
fn main() {
    // Create an instance of the struct
    let original = MyStruct {
        id: 42,
        name: "User".to_string(),
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

    let new_struct = SomeNewStruct {
        id: 99,
        name: "User".to_string()
    };

    if let Some(bytes) = new_struct.convert_to_bytes_populate_option() {
        // operate with bytes in isolated scope
        if let Some(new_struct) = SomeNewStruct::convert_from_bytes_populate_option(&bytes) {
            // operate with struct in isolated scope
        } else {
            // None case
        }
    } else {
        // None case
    }

}
```

## Error Handling

Serialization (convert_to_bytes): If serialization fails, an empty Vec<u8> is returned.
Deserialization (convert_from_bytes): If deserialization fails, the method returns a default instance of the struct. For this to work, the struct must implement the Default trait.

Use convert_to_bytes_populate_option() and convert_from_bytes_populate_option() to populate Option<T>. Use all the
unwrapping Rust methods ( ?, unwrap(), is_none(), is_some(), and so on... ) to uncover value or run check.

## Notes

The library uses the MessagePack format, making it compact and efficient for binary data storage or transmission.
For additional control over serialization, you can use serde attributes (e.g., #[serde(rename = "...")], #[serde(skip)]).

## Usage RuntimeSchema Introspection

Use the RuntimeSchema macro to inspect struct metadata at runtime:

```rust
use montycat_serialization_derive::RuntimeSchema;

#[derive(RuntimeSchema, Default)]
struct User {
    id: String,
    created_at: u32,
    username: String,
}
```

Reflection-like utilities in action:

```rust

let user = User::default();

// 2. Get all field names and types
let all_fields = user.field_names_and_types();
// -> [("id", "String"), ("created_at", "u32"), ("username", "String")]

// 3. Get schema parameters (HashMap + Struct name)
let (schema_map, struct_name) = User::schema_params();
// -> ({"id": "String", "created_at": "u32", "username": "String"}, "User")

```

## 🧠 Design Philosophy

Montycat’s derive macros are built to:
- Maximize performance and compactness with binary serialization.
- Provide type-safe runtime reflection for schema-driven systems.
- Serve as the foundation for the MontyCat NoSQL database implementing data mesh infrastructure.

## See also:

Montycat Rust Client: 
Montycat Engine: Explore the full ecosystem and architecture at https://montygovernance.com


