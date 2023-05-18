use proc_macro2::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Ident, Fields, Type, Data};

fn map_fields<F>(fields: &Fields, mapper: F) -> TokenStream
    where F: FnMut((&Ident, &Type)) -> TokenStream,
{
    TokenStream::from_iter(
        fields
            .iter()
            .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
            .map(mapper),
    )
}

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    // let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());
    let ident_builder = format_ident!("{}Builder", ident);
    if let Data::Struct(r#struct) = input.data {
        let fields = r#struct.fields;
        if matches!(&fields, Fields::Named(_)) {
            let builder_fields = map_fields(&fields, |(ident, ty)| quote!(#ident: Option<#ty>, ));
            let builder_set_fields = map_fields(&fields, |(ident, ty)| {
                quote!(pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                })
            });
            let build_lets = map_fields(&fields, |(ident, _)| {
                quote!(
                    let #ident = self.#ident.ok_or(format!(
                        "field \"{}\" required, but not set yet.",
                        stringify!(#ident),
                    ))?;
                )
            });
            let build_values = map_fields(&fields, |(ident, _)| quote!(#ident,));
            let tokens = quote!(
                impl #ident {
                    pub fn builder() -> #ident_builder {
                        #ident_builder::default()
                    }
                }

                #[derive(Debug, Default)]
                pub struct #ident_builder {
                    #builder_fields
                }

                impl #ident_builder {
                    #builder_set_fields

                    pub fn build(self) -> Result<#ident, String> {
                        #build_lets
                        Ok(#ident { #build_values })
                    }
                }
            );
            return proc_macro::TokenStream::from(tokens);
        }
    }
    proc_macro::TokenStream::from(quote!())
}
