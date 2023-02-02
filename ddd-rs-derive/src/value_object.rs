use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(eq_component), supports(struct_any))]
struct ValueObjectInputReceiver {
    ident: syn::Ident,
    data: darling::ast::Data<(), ValueObjectFieldReceiver>,
}

#[derive(darling::FromField, Debug)]
#[darling(forward_attrs(eq_component))]
struct ValueObjectFieldReceiver {
    ident: Option<syn::Ident>,
    attrs: Vec<syn::Attribute>,
}

impl ValueObjectFieldReceiver {
    pub fn is_eq_component(&self) -> bool {
        self.attrs.iter().any(|f| {
            f.path
                .get_ident()
                .map(|ident| ident == "eq_component")
                .unwrap_or(false)
        })
    }
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ValueObjectInputReceiver { ident, data, .. } =
        match ValueObjectInputReceiver::from_derive_input(&derive_input) {
            Ok(receiver) => receiver,
            Err(e) => return TokenStream::from(e.write_errors()),
        };

    let fields = {
        let fields = data
            .take_struct()
            .expect("Should always be a struct")
            .fields;

        fields
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                (
                    f.is_eq_component(),
                    f.ident
                        .as_ref()
                        .map(|ident| quote!(#ident))
                        .unwrap_or_else(|| {
                            let i = syn::Index::from(i);

                            quote!(#i)
                        }),
                )
            })
            .collect::<Vec<_>>()
    };

    let field = fields.iter().map(|(_, f)| f);
    let eq_field = fields
        .iter()
        .filter(|(is_eq_component, _)| *is_eq_component)
        .map(|(_, f)| f);

    quote! {
        impl ValueObject for #ident {}

        impl Clone for #ident {
            fn clone(&self) -> Self {
                Self {
                    #(
                        #field: self.#field.clone(),
                    )*
                }
            }
        }

        impl PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
            }
        }

        impl PartialOrd for #ident {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                #(
                    match self.#eq_field.partial_cmp(&other.#eq_field) {
                        Some(std::cmp::Ordering::Equal) => {},
                        ord => return ord,
                    }
                )*

                Some(std::cmp::Ordering::Equal)
            }
        }
    }
    .into()
}
