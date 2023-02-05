use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(domain_event), supports(enum_newtype, struct_named))]
struct DomainEventInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<DomainEventVariantReceiver, DomainEventFieldReceiver>,
    #[darling(default)]
    handler: Option<darling::util::IdentString>,
}

#[derive(darling::FromField, Debug)]
struct DomainEventFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

#[derive(darling::FromVariant, Debug)]
struct DomainEventVariantReceiver {
    ident: syn::Ident,
}

pub fn derive(input: TokenStream) -> TokenStream {
    use darling::ast::Data;

    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let DomainEventInputReceiver {
        ident,
        generics,
        data,
        handler,
        ..
    } = match DomainEventInputReceiver::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    match data {
        Data::Enum(variants) => derive_enum(ident, generics, variants, handler),
        Data::Struct(fields) => derive_struct(ident, generics, fields),
    }
}

fn derive_enum(
    ident: syn::Ident,
    generics: syn::Generics,
    variants: Vec<DomainEventVariantReceiver>,
    handler: Option<darling::util::IdentString>,
) -> TokenStream {
    let variant_id = variants.iter().map(|v| {
        let ident = &v.ident;

        quote!(Self::#ident(v) => v.id)
    });

    let variant_at = variants.iter().map(|v| {
        let ident = &v.ident;

        quote!(Self::#ident(v) => &v.at)
    });

    let impl_handler = handler.map(|handler| {
        let variant_arm = variants.iter().map(|v| {
            let variant_ident = &v.ident;

            quote!(#ident::#variant_ident(v) => self.handle(v).await)
        });

        quote! {
            #[async_trait::async_trait]
            impl DomainEventHandler<#ident> for #handler {
                async fn handle(
                    &self,
                    event: #ident,
                ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                    match event {
                        #(
                            #variant_arm,
                        )*
                    }
                }
            }
        }
    });

    quote! {
        impl #generics DomainEvent for #ident #generics {
            fn id(&self) -> uuid::Uuid {
                match self {
                    #(
                        #variant_id,
                    )*
                }
            }

            fn at(&self) -> &chrono::DateTime<chrono::Utc> {
                match self {
                    #(
                        #variant_at,
                    )*
                }
            }
        }

        #impl_handler
    }
    .into()
}

fn derive_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    fields: darling::ast::Fields<DomainEventFieldReceiver>,
) -> TokenStream {
    let extra_fields = fields
        .into_iter()
        .filter(|f| {
            f.ident
                .as_ref()
                .map(|ident| !matches!(quote!(#ident).to_string().as_str(), "id" | "at"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let new_arg = extra_fields.iter().filter_map(|f| {
        f.ident.as_ref().map(|ident| {
            let ty = &f.ty;

            quote!(#ident: #ty)
        })
    });

    let init_field = extra_fields
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|ident| quote!(#ident)));

    quote! {
        impl #generics #ident #generics {
            pub fn new(#(#new_arg, )*) -> Self {
                Self {
                    id: uuid::Uuid::new_v4(),
                    #(
                        #init_field,
                    )*
                    at: chrono::Utc::now(),
                }
            }
        }

        impl #generics DomainEvent for #ident #generics {
            fn id(&self) -> uuid::Uuid {
                self.id
            }

            fn at(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.at
            }
        }
    }
    .into()
}
