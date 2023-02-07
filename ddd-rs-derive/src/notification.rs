use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(attributes(notification), supports(enum_newtype, struct_any))]
struct NotificationInputReceiver {
    ident: syn::Ident,
    data: darling::ast::Data<NotificationVariantReceiver, ()>,
    #[darling(default)]
    handler: Option<darling::util::IdentString>,
}

#[derive(darling::FromVariant, Debug)]
struct NotificationVariantReceiver {
    ident: syn::Ident,
    fields: darling::ast::Fields<NotificationFieldReceiver>,
}

#[derive(darling::FromField, Debug)]
struct NotificationFieldReceiver {
    ty: syn::Type,
}

pub fn derive(input: TokenStream) -> TokenStream {
    use darling::ast::Data;

    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let NotificationInputReceiver {
        ident,
        data,
        handler,
        ..
    } = match NotificationInputReceiver::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    match data {
        Data::Enum(variants) => derive_enum(ident, variants, handler),
        Data::Struct(_) => derive_struct(ident),
    }
}

fn derive_enum(
    ident: syn::Ident,
    variants: Vec<NotificationVariantReceiver>,
    handler: Option<darling::util::IdentString>,
) -> TokenStream {
    let variant: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    let impl_from = variants.iter().map(|v| {
        let variant_ident = &v.ident;
        let variant_ty = &v
            .fields
            .iter()
            .next()
            .expect("Should always be a new-type enum")
            .ty;

        quote! {
            impl From<#variant_ty> for #ident {
                fn from(value: #variant_ty) -> Self {
                    Self::#variant_ident(value)
                }
            }
        }
    });

    let impl_notification_handler = handler.map(|i| {
        let notification_handler_ident = &i.as_ident();

        quote! {
            #[async_trait::async_trait]
            impl ddd_rs::application::NotificationHandler<#ident> for #notification_handler_ident {
                async fn handle(
                    &self,
                    notification: #ident,
                ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                    match notification {
                        #(
                            #ident::#variant(v) => self.handle(v).await,
                        )*
                    }
                }
            }
        }
    });

    quote! {
        impl ddd_rs::presentation::Notification for #ident {}

        #(
            #impl_from
        )*

        #impl_notification_handler
    }
    .into()
}

fn derive_struct(ident: syn::Ident) -> TokenStream {
    quote! (impl ddd_rs::presentation::Notification for #ident {}).into()
}
