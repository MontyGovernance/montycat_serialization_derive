// Importing the necessary crates for procedural macros
extern crate proc_macro;
use proc_macro::TokenStream; // TokenStream is used for passing input/output to/from the macro
use quote::quote; // Quote is used to generate Rust code as tokens
use syn::{parse_macro_input, DeriveInput, Ident, Fields, Data}; // Syn crate for parsing Rust syntax

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
        pub trait BinaryConvert: Serialize + for<'de> serde::Deserialize<'de> + Default + Sized {
            // Method to convert the struct into a byte vector
            fn convert_to_bytes(&self) -> Vec<u8>;
            // Method to convert a byte slice back into the struct
            fn convert_from_bytes(bytes: &[u8]) -> Self;
            // Method to convert a byte slice back into the struct
            fn convert_from_bytes_populate_option(bytes: &[u8]) -> Option<Self>;
            // Method to convert a byte slice back into the struct
            fn convert_to_bytes_populate_option(&self) -> Option<Vec<u8>>;
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

            fn convert_from_bytes_populate_option(bytes: &[u8]) -> Option<Self> {
                // Attempt to deserialize the byte slice into the struct.
                rmp_serde::from_slice(bytes).ok()
            }

            fn convert_to_bytes_populate_option(&self) -> Option<Vec<u8>> {
                // Attempt to serialize the struct into a MessagePack byte vector.
                rmp_serde::to_vec(self).ok()
            }

        }
    };

    // Return the generated code as a TokenStream, which will be inserted into the user's code.
    gen.into()
}

/// This procedural macro derives a trait called `RuntimeSchema` for the struct
/// it's applied to. The `RuntimeSchema` trait provides methods to introspect
/// the struct's fields at runtime, specifically to identify fields of certain
/// types (like `Pointer` and `Timestamp`), retrieve all field names and types,
/// and generate schema parameters.
///
//// ## Overview:
/// The trait defines three methods:
/// - `pointer_and_timestamp_fields(&self) -> Vec<(&'static str, &'static str)>`:
///   Returns a vector of field names and their types for fields that are either
///   `Pointer` or `Timestamp`.
/// - `field_names_and_types(&self) -> Vec<(&'static str, &'static str)>`:
///   Returns a vector of all field names and their types in the struct.
/// - `schema_params() -> (HashMap<&'static str, &'static str>, &'static str)`:
///   Returns a tuple containing a HashMap of field names to their types
///   and the name of the struct.
///
//// The implementation uses Rust's type identification to check field types
/// and generates the necessary code to fulfill the trait's requirements.
///
/// # Note:
/// The struct must have named fields for this macro to work correctly.
///
/// # Dependencies:
/// This macro assumes the existence of `Pointer` and `Timestamp` types in scope.
///
/// # Example Usage:
///
/// ```rust
/// #[derive(RuntimeSchema)]
/// struct MyStruct {
///    id: Pointer,
///   timestamp: Timestamp,
///   name: String,
/// }
/// ```
///
/// The above will generate an implementation of `RuntimeSchema` for `MyStruct`.
///
/// # Generated Methods:
/// - `pointer_and_timestamp_fields`: Will return `[("id", "Pointer"), ("timestamp", "Timestamp")]`
/// - `field_names_and_types`: Will return `[("id", "Pointer"), ("timestamp", "Timestamp"), ("name", "String")]`
/// - `schema_params`: Will return a HashMap with all field names and types, along with the struct name "MyStruct".
///
/// ```rust
/// let (schema_map, struct_name) = MyStruct::schema_params();
/// ```
///
/// The above will give you a HashMap of field names to types and the struct name.
///
#[proc_macro_derive(RuntimeSchema)]
pub fn montycat_schema_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut matches = Vec::new();
    let mut field_info = Vec::new();
    let mut schema_inserts = Vec::new();

    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in fields_named.named.iter() {
                let ident = &field.ident;
                let ty = &field.ty;

                matches.push(quote! {
                    if std::any::TypeId::of::<#ty>() == std::any::TypeId::of::<Pointer>() {
                        result.push((stringify!(#ident), "Pointer"));
                    }
                    if std::any::TypeId::of::<#ty>() == std::any::TypeId::of::<Timestamp>() {
                        result.push((stringify!(#ident), "Timestamp"));
                    }
                });

                field_info.push(quote! {
                    (stringify!(#ident), stringify!(#ty))
                });

                schema_inserts.push(quote! {
                    map.insert(stringify!(#ident), stringify!(#ty));
                });

            }
        }
    }

    let gen = quote! {
        impl RuntimeSchema for #name {
            fn pointer_and_timestamp_fields(&self) -> Vec<(&'static str, &'static str)> {
                let mut result = Vec::new();
                #(#matches)*
                result
            }

            fn field_names_and_types(&self) -> Vec<(&'static str, &'static str)> {
                vec![#(#field_info),*]
            }

            fn schema_params() -> (HashMap<&'static str, &'static str>, &'static str) {
                let mut map = std::collections::HashMap::new();
                #(#schema_inserts)*
                (map, stringify!(#name))
            }

        }
    };

    gen.into()
}
