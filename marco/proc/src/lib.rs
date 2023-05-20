use proc_macro2::TokenStream;
use syn::{Fields, Ident, Type};

pub use builder::*;

mod builder;


pub(crate) fn map_fields<F>(fields: &Fields, mapper: F) -> TokenStream
    where F: FnMut((&Ident, &Type)) -> TokenStream,
{
    TokenStream::from_iter(
        fields
            .iter()
            .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
            .map(mapper),
    )
}
