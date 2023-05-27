use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput)]
struct AggregateRoot {
    ident: syn::Ident,
    generics: syn::Generics,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let AggregateRoot {
        ident, generics, ..
    } = match AggregateRoot::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    quote!(impl #generics ddd_rs::domain::AggregateRoot for #ident #generics {}).into()
}
