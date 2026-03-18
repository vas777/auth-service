extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{self, Attribute, FnArg, Ident, ItemFn, Pat, Stmt, parse::{Parse, ParseStream}, parse_macro_input, parse_quote, Result};
use darling::{ToTokens};

struct LogArgs { 
    clean: bool 
}

impl Parse for LogArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(LogArgs { clean: false });
        }
        
        let ident: Ident = input.parse()?;
        if ident == "clean" {
            Ok(LogArgs { clean: true })
        } else {
            Err(syn::Error::new(ident.span(), "expected `clean`"))
        }
    }
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as LogArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    
    impl_log_call(&attr_args, &mut input)
}

fn impl_log_call(attr_args: &LogArgs, input: &mut ItemFn) -> TokenStream {

    if attr_args.clean {
        let clean_stmt: Stmt = parse_quote! {
            app.clean_up().await;
        };
        input.block.stmts.push(clean_stmt);
    }
    // let fn_name = &input.sig.ident;

    // if attr_args.verbose {
    //     let fn_args = extract_arg_names(input);
    //     let statements = generate_verbose_log(fn_name, fn_args);

    //     input.block.stmts.splice(0..0, statements);
    // } else {
    //     input.block.stmts.insert(0, parse_quote! {
    //         println!("[Info] calling {}", stringify!(#fn_name));
    //     });
    // }

    input.to_token_stream().into()
}