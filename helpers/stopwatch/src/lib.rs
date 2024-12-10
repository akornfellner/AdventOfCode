extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract relevant information from the input function
    let fn_name = input_fn.sig.ident;
    let fn_block = input_fn.block;
    let fn_inputs = input_fn.sig.inputs;
    let fn_output = input_fn.sig.output;

    // Generate the new function with logging
    let expanded = quote! {
        fn #fn_name(#fn_inputs) #fn_output {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            println!("Time: {}ms", start.elapsed().as_millis());
            result
        }
    };

    TokenStream::from(expanded)
}
