extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod repr_std140;

#[proc_macro_attribute]
pub fn repr_std140(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty(), "#[repr_std140] does not take arguments.");

    let input = parse_macro_input!(input as DeriveInput);

    repr_std140::expand_repr_std140(&input)
        .unwrap_or_else(compile_error)
        .into()
}

fn compile_error(message: String) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#message);
    }
}
