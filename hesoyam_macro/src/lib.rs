use quote::quote;
use syn::{AttributeArgs, DeriveInput, parse_macro_input};
use syn::export::TokenStream;

use crate::context::ModelContext;
use crate::gen::model::gen_model_code;
use crate::gen::model_delete::gen_model_delete_code;
use crate::gen::model_impl::gen_model_impl_code;
use crate::gen::model_insert::gen_model_insert_code;
use crate::gen::model_select::gen_model_select_code;
use crate::gen::model_update::gen_model_update_code;
use crate::gen::query_result_impl::gen_query_result_impl;

#[cfg(test)]
mod tests;

mod context;
mod gen;

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
    let model_update_code = gen_model_update_code(&ctx);
    let model_select_code = gen_model_select_code(&ctx);

    let output = quote! {
        #derive_input_copy

        #model_code
        #model_impl_code
        #model_insert_code
        #model_delete_code
        #model_update_code
        #model_select_code
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn query_result(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_copy = input.clone();
    let derive_input = parse_macro_input!(input as DeriveInput);
    let derive_input_copy = parse_macro_input!(input_copy as DeriveInput);

    let impl_code = gen_query_result_impl(derive_input);

    let output = quote! {
        #derive_input_copy

        #impl_code
    };

    TokenStream::from(output)
}
