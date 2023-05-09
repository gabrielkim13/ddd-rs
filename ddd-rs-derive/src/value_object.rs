use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(
    attributes(eq_component),
    supports(enum_unit, enum_newtype, struct_any)
)]
struct ValueObjectInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
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
            f.path()
                .get_ident()
                .map(|ident| ident == "eq_component")
                .unwrap_or(false)
        })
    }
}

#[derive(darling::FromField, Debug)]
struct ValueObjectVariantNewTypeReceiver;

#[derive(darling::FromVariant, Debug)]
struct ValueObjectVariantReceiver {
    ident: syn::Ident,
    fields: darling::ast::Fields<ValueObjectVariantNewTypeReceiver>,
}

impl ValueObjectVariantReceiver {
    pub fn is_unit(&self) -> bool {
        self.fields.is_empty()
    }
}

pub fn derive(input: TokenStream) -> TokenStream {
    use darling::ast::Data;

    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ValueObjectInputReceiver {
        ident,
        generics,
        data,
        ..
    } = match ValueObjectInputReceiver::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    match data {
        Data::Enum(variants) => derive_enum(ident, generics, variants),
        Data::Struct(fields) => derive_struct(ident, generics, fields),
    }
}

fn derive_enum(
    ident: syn::Ident,
    generics: syn::Generics,
    variants: Vec<ValueObjectVariantReceiver>,
) -> TokenStream {
    let is_enum_unit = variants.iter().all(|v| v.is_unit());

    if is_enum_unit {
        derive_enum_unit(ident, generics, variants)
    } else {
        derive_enum_newtype(ident, generics, variants)
    }
}

fn derive_enum_unit(
    ident: syn::Ident,
    generics: syn::Generics,
    variants: Vec<ValueObjectVariantReceiver>,
) -> TokenStream {
    let variant: Vec<_> = variants.into_iter().map(|v| v.ident).collect();

    quote! {
        impl #generics ddd_rs::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                match self {
                    #(
                        Self::#variant => Self::#variant,
                    )*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
            }
        }

        impl #generics PartialOrd for #ident #generics {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                #[allow(unreachable_patterns)]
                match (self, other) {
                    #(
                        (Self::#variant, Self::#variant) => Some(std::cmp::Ordering::Equal),
                        (Self::#variant, _) => Some(std::cmp::Ordering::Greater),
                        (_, Self::#variant) => Some(std::cmp::Ordering::Less),
                    )*
                }
            }
        }
    }
    .into()
}

fn derive_enum_newtype(
    ident: syn::Ident,
    generics: syn::Generics,
    variants: Vec<ValueObjectVariantReceiver>,
) -> TokenStream {
    let variant: Vec<_> = variants.into_iter().map(|v| v.ident).collect();

    quote! {
        impl #generics ddd_rs::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                match self {
                    #(
                        Self::#variant => Self::#variant,
                    )*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
            }
        }

        impl #generics PartialOrd for #ident #generics {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match (self, other) {
                    #(
                        (Self::#variant(l), Self::#variant(r)) => l.partial_cmp(r),
                    )*
                    _ => None,
                }
            }
        }
    }
    .into()
}

fn derive_struct(
    ident: syn::Ident,
    generics: syn::Generics,
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
        impl #generics ddd_rs::domain::ValueObject for #ident #generics {}

        impl #generics Clone for #ident #generics {
            fn clone(&self) -> Self {
                Self {
                    #(
                        #field: self.#field.clone(),
                    )*
                }
            }
        }

        impl #generics PartialEq for #ident #generics {
            fn eq(&self, other: &Self) -> bool {
                matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
            }
        }

        impl #generics PartialOrd for #ident #generics {
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
