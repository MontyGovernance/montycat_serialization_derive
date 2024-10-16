extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(BinaryConvert)]
pub fn binary_convert_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name: Ident = ast.ident;

    let gen = quote! {

        pub trait BinaryConvert: Serialize {
            fn convert_to_bytes(&self) -> Vec<u8>;
            fn convert_from_bytes(bytes: &[u8]) -> Self;
        }

        impl BinaryConvert for #name
        where
            #name: Serialize + for<'de> serde::Deserialize<'de>,
        {
            fn convert_to_bytes(&self) -> Vec<u8> {
                rmp_serde::to_vec(self).expect("Failed to convert to bytes")
            }

            fn convert_from_bytes(bytes: &[u8]) -> Self {
                rmp_serde::from_slice(bytes).expect("Failed to convert from bytes")
            }
        }

    };

    gen.into()
}