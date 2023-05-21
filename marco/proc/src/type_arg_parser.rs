use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput};

#[proc_macro_derive(arg_parser_type, attributes(arg_parser_property))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;

    let mut debug_fields = vec![];
    if let syn::Data::Struct(ds) = ast.data {
        if let syn::Fields::Named(fields) = ds.fields {
            for field in fields.named.iter() {
                let field_name = field.ident.clone().unwrap();
                if let Some(attr) = field.attrs.clone().iter().next() {
                    match parse_args_attr_value(&attr) {
                        Ok(v) => {
                            // TODO
                        },
                        Err(err) => return err.to_compile_error().into(),
                    }
                }
            }
        }
    }
    let tokens = quote!{
	};
    tokens.into()
}

fn parse_args_attr_value(attr: &syn::Attribute) -> Result<Option<syn::LitStr>, syn::Error> {
    if let Some(seg) = attr.path.segments.first() {
        if seg.ident == "arg_parser_property" {
            let args = attr.parse_args()?;
            if let syn::Meta::NameValue(values) = args {
                let arg_name = &values.path.segments.first().unwrap().ident;
                if arg_name == "name" {
                    if let syn::Lit::Str(name) = values.lit {
                        return Ok(Some(name));
                    }
                } else {
                    return Err(syn::Error::new(attr.bracket_token.span, "expected `args(name = \"...\")`".to_owned()));
                }
            }
        }
    }
    return Ok(None)
}