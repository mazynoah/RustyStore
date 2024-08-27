extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Storing)]
pub fn storing_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_storing(&ast)
}

fn impl_storing(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Storing for #name {}
    };
    gen.into()
}
