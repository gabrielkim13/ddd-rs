use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(supports(struct_named))]
struct AggregateRootInputReceiver {
    ident: syn::Ident,
    data: darling::ast::Data<(), AggregateRootFieldReceiver>,
}

#[derive(darling::FromField, Debug)]
struct AggregateRootFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub fn derive(input: TokenStream) -> TokenStream {
    use syn::{
        AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, Type, TypePath,
    };

    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let AggregateRootInputReceiver { ident, data, .. } =
        match AggregateRootInputReceiver::from_derive_input(&derive_input) {
            Ok(receiver) => receiver,
            Err(e) => return TokenStream::from(e.write_errors()),
        };

    let fields = data
        .as_ref()
        .take_struct()
        .expect("Should always be named struct")
        .fields;

    let domain_events_ty = match fields.into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|ident| quote!(#ident).to_string() == "domain_events")
            .unwrap_or(false)
    }) {
        Some(field) => &field.ty,
        None => panic!("Missing `domain_events` field"),
    };

    let generic_argument = match domain_events_ty {
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
    };

    let domain_event_ty = match generic_argument {
        GenericArgument::Type(ty) => ty,
        _ => panic!("Invalid type variant for `domain_events` Vec<T> generic argument"),
    };

    quote! {
        impl AggregateRoot for #ident {
            type DomainEvent = #domain_event_ty;

            fn register_domain_event(&mut self, event: Self::DomainEvent) {
                self.domain_events.push(event)
            }

            fn drain_domain_events(&mut self) -> Vec<Self::DomainEvent> {
                self.domain_events.drain(..).collect()
            }
        }
    }
    .into()
}
