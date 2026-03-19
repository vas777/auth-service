extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{self, ItemFn, parse_macro_input };
use quote::quote;

#[proc_macro_attribute]
pub fn test_help(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    
    let fn_name = &input.sig.ident;
    let fn_input = &input.sig.inputs;
    let fn_return = &input.sig.output;
    let fn_body = &input.block;

    let res = quote! {
        #[tokio::test]
        async fn #fn_name(#fn_input) #fn_return { 
            let mut app = TestApp::new().await;
            #fn_body
           app.clean_up().await;
        }
    };

    res.into()
}
