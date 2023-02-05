use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(supports(struct_named))]
struct EntityInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), EntityFieldReceiver>,
}

#[derive(darling::FromField, Debug)]
struct EntityFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let EntityInputReceiver {
        ident,
        generics,
        data,
        ..
    } = match EntityInputReceiver::from_derive_input(&derive_input) {
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
        impl #generics Entity for #ident #generics {
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

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                self.id() == other.id()
            }
        }

        impl #generics Eq for #ident #generics {}
    }
    .into()
}
