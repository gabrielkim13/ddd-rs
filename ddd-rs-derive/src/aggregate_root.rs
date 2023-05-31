use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput)]
#[darling(attributes(aggregate_root), supports(struct_named))]
struct AggregateRoot {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, AggregateRootField>,
}

#[derive(darling::FromMeta)]
struct DomainEventsMarker;

#[derive(darling::FromField)]
#[darling(attributes(aggregate_root))]
struct AggregateRootField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    domain_events: Option<DomainEventsMarker>,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let AggregateRoot {
        ident,
        generics,
        data,
        ..
    } = match AggregateRoot::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let fields = data.take_struct().unwrap();

    derive_aggregate_root(ident, generics, fields)
}

fn derive_aggregate_root(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<AggregateRootField>,
) -> TokenStream {
    let aggregate_root_ex = fields
        .into_iter()
        .find_map(|f| {
            f.domain_events.map(|_| {
                let domain_events_ident = f.ident.unwrap();
                let domain_events_ty = map_domain_event_ty(f.ty);

                quote! {
                    impl #generics #ident #generics {
                        fn register_domain_event(
                            &mut self,
                            domain_event: <Self as ddd_rs::domain::AggregateRootEx>::DomainEvent
                        ) {
                            self.#domain_events_ident.push(domain_event);
                        }
                    }

                    impl #generics ddd_rs::domain::AggregateRootEx for #ident #generics {
                        type DomainEvent = #domain_events_ty;

                        fn take_domain_events(&mut self) -> Vec<Self::DomainEvent> {
                            self.#domain_events_ident.drain(..).collect()
                        }
                    }
                }
            })
        })
        .unwrap_or_default();

    quote! {
        impl #generics ddd_rs::domain::AggregateRoot for #ident #generics {}

        #aggregate_root_ex
    }
    .into()
}

fn map_domain_event_ty(ty: syn::Type) -> syn::Type {
    use syn::{GenericArgument, PathArguments, Type};

    let mut ty_path = match ty {
        Type::Path(syn::TypePath { path, .. }) => path,
        _ => panic!("Domain events field type must be of kind `syn::Path`"),
    };

    let syn::PathSegment {
        ident, arguments, ..
    } = ty_path.segments.pop().unwrap().into_value();

    if ident != "Vec" {
        panic!("Domain events field must be a `Vec` when deriving this trait");
    }

    let arg = match arguments {
        PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { mut args, .. }) => {
            args.pop().unwrap().into_value()
        }
        _ => unreachable!(),
    };

    match arg {
        GenericArgument::Type(ty) => ty,
        _ => unreachable!(),
    }
}
