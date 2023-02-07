use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(entity), supports(enum_newtype, struct_named))]
struct EntityInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EntityVariantReceiver, EntityFieldReceiver>,
    #[darling(default)]
    id: Option<darling::util::IdentString>,
}

#[derive(darling::FromField, Debug)]
struct EntityFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(darling::FromVariant, Debug)]
struct EntityVariantReceiver {
    ident: syn::Ident,
}

pub fn derive(input: TokenStream) -> TokenStream {
    use darling::ast::Data;

    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let EntityInputReceiver {
        ident,
        generics,
        data,
        id,
        ..
    } = match EntityInputReceiver::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    match data {
        Data::Enum(variants) => derive_enum(
            ident,
            generics,
            variants,
            id.expect("Missing `id` attribute on new-type enum"),
        ),
        Data::Struct(fields) => derive_struct(ident, generics, fields),
    }
}

fn derive_enum(
    ident: syn::Ident,
    generics: syn::Generics,
    variants: Vec<EntityVariantReceiver>,
    id: darling::util::IdentString,
) -> TokenStream {
    let id_ident = id.as_ident();

    let variant: Vec<_> = variants.into_iter().map(|v| v.ident).collect();

    quote! {
        impl #generics ddd_rs::domain::Entity for #ident #generics {
            type Id = #id_ident;

            fn id(&self) -> Self::Id {
                match self {
                    #(
                        Self::#variant(v) => v.id(),
                    )*
                }
            }

            fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
                match self {
                    #(
                        Self::#variant(v) => v.created_at(),
                    )*
                }
            }

            fn updated_at(&self) -> &chrono::DateTime<chrono::Utc> {
                match self {
                    #(
                        Self::#variant(v) => v.updated_at(),
                    )*
                }
            }
        }
    }
    .into()
}

fn derive_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<EntityFieldReceiver>,
) -> TokenStream {
    let id_ty = fields
        .into_iter()
        .find(|f| {
            f.ident
                .as_ref()
                .map(|ident| quote!(#ident).to_string() == "id")
                .unwrap_or(false)
        })
        .expect("Missing `id` field")
        .ty;

    quote! {
        impl #generics ddd_rs::domain::Entity for #ident #generics {
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
                use ddd_rs::domain::Entity;

                self.id() == other.id()
            }
        }

        impl #generics Eq for #ident #generics {}
    }
    .into()
}
