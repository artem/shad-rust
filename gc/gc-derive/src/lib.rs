use proc_macro::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Ident};

#[proc_macro_derive(Scan)]
pub fn derive_scan(input: TokenStream) -> TokenStream {
    // TODO: your code here.
    unimplemented!()
}
