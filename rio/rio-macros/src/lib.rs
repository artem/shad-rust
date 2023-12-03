use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test(_attrs: TokenStream, stream: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(stream as ItemFn);
    input.sig.asyncness = None;

    // Source: https://docs.rs/tokio-macros/1.8.0/src/tokio_macros/entry.rs.html#384
    let body = &input.block;
    let brace_token = input.block.brace_token;
    let block_expr = quote! {
        {
            return ::rio::Runtime::new_current_thread()
                .block_on(body);
        }
    };
    input.block = syn::parse2(quote! {
        {
            let body = async #body;
            #block_expr
        }
    })
    .expect("Parsing failure");
    input.block.brace_token = brace_token;

    let result = quote! {
        #[test]
        #input
    };

    result.into()
}
