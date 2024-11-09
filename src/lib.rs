// Importing the necessary crates for procedural macros
extern crate proc_macro;
use proc_macro::TokenStream; // TokenStream is used for passing input/output to/from the macro
use quote::quote; // Quote is used to generate Rust code as tokens
use syn::{parse_macro_input, DeriveInput, Ident}; // Syn crate for parsing Rust syntax

/// This procedural macro derives a trait called `BinaryConvert` for the struct
/// it's applied to. The `BinaryConvert` trait provides methods to convert a
/// struct to and from its byte representation, useful for serializing and
/// deserializing to and from binary formats.
///
/// ## Overview:
/// The trait defines two methods:
/// - `convert_to_bytes(&self) -> Vec<u8>`: Converts the struct into a byte vector.
/// - `convert_from_bytes(bytes: &[u8]) -> Self`: Converts a byte slice back into the struct.
/// 
/// The implementation uses `rmp_serde` (a MessagePack serializer/deserializer) for binary
/// serialization and deserialization of the struct. The struct must also implement `Serialize`
/// and `Deserialize` to work with `rmp_serde` correctly.

#[proc_macro_derive(BinaryConvert)] 
pub fn binary_convert_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a DeriveInput, which represents the structure
    // of the item (e.g., struct, enum) to which the macro is being applied.
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    
    // Extract the identifier (name) of the struct or enum from the parsed input.
    let name: Ident = ast.ident;

    // Generate the implementation of the `BinaryConvert` trait for the struct
    let gen = quote! {
        // Define the BinaryConvert trait
        pub trait BinaryConvert: Serialize {
            // Method to convert the struct into a byte vector
            fn convert_to_bytes(&self) -> Vec<u8>;

            // Method to convert a byte slice back into the struct
            fn convert_from_bytes(bytes: &[u8]) -> Self;
        }

        // Implement the BinaryConvert trait for the specific struct (identified by `#name`)
        impl BinaryConvert for #name
        where
            #name: Serialize + for<'de> serde::Deserialize<'de> + Default, // The struct must implement `Serialize`, `Deserialize`, and `Default`
        {
            // Implementation of the `convert_to_bytes` method
            fn convert_to_bytes(&self) -> Vec<u8> {
                // Attempt to serialize the struct into a MessagePack byte vector.
                if let Ok(vector) = rmp_serde::to_vec(self) {
                    vector // Return the byte vector if serialization was successful
                } else {
                    Vec::new() // Return an empty vector if serialization failed
                }
            }

            // Implementation of the `convert_from_bytes` method
            fn convert_from_bytes(bytes: &[u8]) -> Self {
                // Attempt to deserialize the byte slice into the struct.
                if let Ok(structure) = rmp_serde::from_slice(bytes) {
                    structure // Return the deserialized struct if successful
                } else {
                    Self::default() // Return the default value of the struct if deserialization failed
                }
            }
        }
    };

    // Return the generated code as a TokenStream, which will be inserted into the user's code.
    gen.into()
}
