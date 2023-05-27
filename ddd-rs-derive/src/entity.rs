use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput)]
#[darling(attributes(entity), supports(struct_named))]
struct Entity {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, EntityField>,
}

#[derive(darling::FromField)]
#[darling(attributes(entity))]
struct EntityField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    id: bool,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let Entity {
        ident,
        generics,
        data,
        ..
    } = match Entity::from_derive_input(&derive_input) {
        Ok(entity) => entity,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let fields = data.take_struct().unwrap();

    derive_struct(ident, generics, fields)
}

fn derive_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<EntityField>,
) -> TokenStream {
    let id_field = fields
        .into_iter()
        .find(|f| f.id)
        .expect("Missing `id` field");

    let id_ident = id_field.ident.unwrap();
    let id_ty = id_field.ty;

    quote! {
        impl #generics ddd_rs::domain::Entity for #ident #generics {
            type Id = #id_ty;

            fn id(&self) -> &Self::Id {
                &self.#id_ident
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
