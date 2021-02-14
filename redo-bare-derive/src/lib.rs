
#![no_std]

use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(Register)]
pub fn register_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_register_macro(&ast)
}

fn impl_register_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Register for #name { }
    };
    gen.into()
}