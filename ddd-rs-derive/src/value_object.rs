use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput)]
#[darling(attributes(value_object), supports(struct_named))]
struct ValueObject {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, ValueObjectField>,
}

#[derive(darling::FromField)]
#[darling(attributes(value_object))]
struct ValueObjectField {
    ident: Option<syn::Ident>,
    #[darling(default)]
    eq: bool,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ValueObject {
        ident,
        generics,
        data,
        ..
    } = match ValueObject::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let fields = data.take_struct().unwrap();

    derive_struct(ident, generics, fields)
}

fn derive_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<ValueObjectField>,
) -> TokenStream {
    let fields = fields
        .into_iter()
        .map(|f| (f.eq, f.ident.as_ref().map(|ident| quote!(#ident)).unwrap()))
        .collect::<Vec<_>>();

    let field = fields.iter().map(|(_, f)| f);
    let eq_field = fields.iter().filter_map(|(eq, f)| eq.then_some(f));

    quote! {
        impl #generics ddd_rs::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                Self {
                    #(#field: self.#field.clone(),)*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                true #( && self.#eq_field == other.#eq_field)*
            }
        }
    }
    .into()
}
