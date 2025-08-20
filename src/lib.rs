extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::Visibility::Inherited;
use syn::{parse_macro_input, Data, DeriveInput, Visibility};

#[proc_macro_attribute]
pub fn vis(args: TokenStream, input: TokenStream) -> TokenStream {
    let visibility_input = parse_macro_input!(args as Visibility);
    let mut input_ast = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(ref mut stru) = input_ast.data {
        if let syn::Fields::Named(ref mut fields) = stru.fields {
            for field in fields.named.iter_mut() {
                if matches!(field.vis, Inherited) {
                    field.vis = visibility_input.clone();
                }
            }
        } else {
            return syn::Error::new_spanned(
                &stru.fields,
                "vis macro only supports structs with named fields",
            )
                .to_compile_error()
                .into();
        }
    }

    let expanded = quote! {
        #input_ast
    };

    expanded.into()
}
