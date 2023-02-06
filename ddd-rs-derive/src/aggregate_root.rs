use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(aggregate_root), supports(enum_newtype, struct_named))]
struct AggregateRootInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<AggregateRootVariantReceiver, AggregateRootFieldReceiver>,
    #[darling(default)]
    domain_event: Option<darling::util::IdentString>,
}

#[derive(darling::FromField, Debug)]
struct AggregateRootFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(darling::FromVariant, Debug)]
struct AggregateRootVariantReceiver {
    ident: syn::Ident,
}

pub fn derive(input: TokenStream) -> TokenStream {
    use darling::ast::Data;

    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let AggregateRootInputReceiver {
        ident,
        generics,
        data,
        domain_event,
        ..
    } = match AggregateRootInputReceiver::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    match data {
        Data::Enum(variants) => derive_enum(
            ident,
            generics,
            variants,
            domain_event.expect("Missing `domain_event` attribute on new-type enum"),
        ),
        Data::Struct(fields) => derive_struct(ident, generics, fields),
    }
}

fn derive_enum(
    ident: syn::Ident,
    generics: syn::Generics,
    variants: Vec<AggregateRootVariantReceiver>,
    domain_event: darling::util::IdentString,
) -> TokenStream {
    let domain_event_ident = domain_event.as_ident();

    let variant: Vec<_> = variants.into_iter().map(|v| v.ident).collect();

    quote! {
        impl #generics AggregateRoot for #ident #generics {
            type DomainEvent = #domain_event_ident;

            fn register_domain_event(&mut self, event: impl Into<Self::DomainEvent>) {
                match self {
                    #(
                        Self::#variant(v) => v.register_domain_event(event),
                    )*
                }
            }

            fn drain_domain_events(&mut self) -> Vec<Self::DomainEvent> {
                match self {
                    #(
                        Self::#variant(v) => v.drain_domain_events(),
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
    fields: darling::ast::Fields<AggregateRootFieldReceiver>,
) -> TokenStream {
    use syn::{
        AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, Type, TypePath,
    };

    // Detect the domain event type from a "domain_events: Vec<_>" field.
    let domain_event_ty = {
        let domain_events_field = fields.iter().find(|f| {
            f.ident
                .as_ref()
                .map(|ident| quote!(#ident).to_string() == "domain_events")
                .unwrap_or(false)
        });

        let generic_argument = domain_events_field.map(|f| match &f.ty {
            Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) => match &segments
                .first()
                .expect("Invalid empty path segments for `domain_events` field")
                .arguments
            {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => args
                    .first()
                    .expect("Invalid empty generic for `domain_events` field"),
                _ => panic!("`domain_events` field must be a Vec<_>"),
            },
            _ => panic!("Invalid type variant for `domain_events` field"),
        });

        generic_argument.map(|a| match a {
            GenericArgument::Type(ty) => ty,
            _ => panic!("Invalid type variant for `domain_events` Vec<T> generic argument"),
        })
    };

    // If it doesn't exist, use the unit type `()` as the domain event type and derive a dummy
    // implementation of `AggregateRoot`.
    match domain_event_ty {
        Some(ty) => quote! {
            impl #generics AggregateRoot for #ident #generics {
                type DomainEvent = #ty;

                fn register_domain_event(&mut self, event: impl Into<Self::DomainEvent>) {
                    self.domain_events.push(event.into())
                }

                fn drain_domain_events(&mut self) -> Vec<Self::DomainEvent> {
                    self.domain_events.drain(..).collect()
                }
            }
        },
        None => quote! {
            impl #generics AggregateRoot for #ident #generics {
                type DomainEvent = ();

                fn register_domain_event(&mut self, _event: impl Into<Self::DomainEvent>) {}

                fn drain_domain_events(&mut self) -> Vec<Self::DomainEvent> {
                    vec![]
                }
            }
        },
    }
    .into()
}
