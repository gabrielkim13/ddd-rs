use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(supports(struct_named))]
struct EntityInputReceiver {
    ident: syn::Ident,
    data: darling::ast::Data<(), EntityFieldReceiver>,
}

#[derive(darling::FromField, Debug)]
struct EntityFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let EntityInputReceiver { ident, data, .. } =
        match EntityInputReceiver::from_derive_input(&derive_input) {
            Ok(receiver) => receiver,
            Err(e) => return TokenStream::from(e.write_errors()),
        };

    let fields = data
        .as_ref()
        .take_struct()
        .expect("Should always be named struct")
        .fields;

    let id_ty = match fields.into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|ident| quote!(#ident).to_string() == "id")
            .unwrap_or(false)
    }) {
        Some(field) => &field.ty,
        None => panic!("Missing `id` field"),
    };

    quote! {
        impl Entity for #ident {
            type Id = #id_ty;

            fn id(&self) -> Self::Id {
                self.id
            }

            fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.created_at
            }

            fn updated_at(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.updated_at
            }
        }

        impl PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                self.id() == other.id()
            }
        }

        impl Eq for #ident {}
    }
    .into()
}
