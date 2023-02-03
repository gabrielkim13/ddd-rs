use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(eq_component), supports(enum_unit, struct_any))]
struct ValueObjectInputReceiver {
    ident: syn::Ident,
    data: darling::ast::Data<ValueObjectVariantReceiver, ValueObjectFieldReceiver>,
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

#[derive(darling::FromVariant, Debug)]
struct ValueObjectVariantReceiver {
    ident: syn::Ident,
}

pub fn derive(input: TokenStream) -> TokenStream {
    use darling::ast::Data;

    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ValueObjectInputReceiver { ident, data, .. } =
        match ValueObjectInputReceiver::from_derive_input(&derive_input) {
            Ok(receiver) => receiver,
            Err(e) => return TokenStream::from(e.write_errors()),
        };

    match data {
        Data::Enum(variants) => derive_enum(ident, variants),
        Data::Struct(fields) => derive_struct(ident, fields),
    }
}

fn derive_enum(ident: syn::Ident, variants: Vec<ValueObjectVariantReceiver>) -> TokenStream {
    let variant_clone = variants.iter().map(|v| {
        let ident = &v.ident;

        quote!(Self::#ident => Self::#ident)
    });

    let variant_partial_ord = variants.iter().map(|v| {
        let ident = &v.ident;

        quote! {
            (Self::#ident, Self::#ident) => Some(std::cmp::Ordering::Equal),
            (Self::#ident, _) => Some(std::cmp::Ordering::Greater),
            (_, Self::#ident) => Some(std::cmp::Ordering::Less)
        }
    });

    quote! {
        impl ValueObject for #ident {}

        impl Clone for #ident {
            fn clone(&self) -> Self {
                match self {
                    #(
                        #variant_clone,
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
                #[allow(unreachable_patterns)]
                match (self, other) {
                    #(
                        #variant_partial_ord,
                    )*
                }
            }
        }
    }
    .into()
}

fn derive_struct(
    ident: syn::Ident,
    fields: darling::ast::Fields<ValueObjectFieldReceiver>,
) -> TokenStream {
    let fields = fields
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
        .collect::<Vec<_>>();

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
