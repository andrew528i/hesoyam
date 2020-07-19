use quote::quote;
use syn::{AttributeArgs, DeriveInput, parse_macro_input};
use syn::export::TokenStream;

use crate::context::ModelContext;
use crate::gen::model::gen_model_code;
use crate::gen::model_impl::gen_model_impl_code;
use crate::gen::model_insert::gen_model_insert_code;
use crate::gen::model_delete::gen_model_delete_code;

mod context;
mod gen;
mod insert;

#[proc_macro_attribute]
pub fn model(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_copy = input.clone();
    let attribute_args = parse_macro_input!(args as AttributeArgs);
    let derive_input = parse_macro_input!(input as DeriveInput);
    let derive_input_copy = parse_macro_input!(input_copy as DeriveInput);

    let ctx = ModelContext::parse(attribute_args, derive_input);

    // generate code for Model trait
    let model_code = gen_model_code(&ctx);
    let model_impl_code = gen_model_impl_code(&ctx);
    let model_insert_code = gen_model_insert_code(&ctx);
    let model_delete_code = gen_model_delete_code(&ctx);

    let output = quote! {
        #derive_input_copy

        #model_code
        #model_impl_code
        #model_insert_code
        #model_delete_code
    };

    // println!("{}", output.to_string());
    // panic!("debug interruption");

    TokenStream::from(output)
}
