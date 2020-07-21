use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_delete_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let struct_span = &ctx.struct_span;
    let dialect = &ctx.model_args.dialect;

    // let delete_ident = format!("{}Delete", struct_ident.to_string());
    // let delete_ident = syn::Ident::new(&delete_ident, struct_span.clone());

    let delete_many_ident = format!("{}DeleteMany", struct_ident.to_string());
    let delete_many_ident = syn::Ident::new(&delete_many_ident, struct_span.clone());

    quote! {
        // pub trait #delete_ident {
        //     fn delete(&self) -> hesoyam::QueryBuilder;
        // }
        //
        // impl #delete_ident for #struct_ident {
        //     fn delete(&self) -> hesoyam::QueryBuilder {
        //     }
        // }

        pub trait #delete_many_ident {
            fn delete(conditions: Vec<hesoyam::Condition>) -> hesoyam::QueryBuilder;
        }

        impl #delete_many_ident for #struct_ident {
            fn delete(conditions: Vec<hesoyam::Condition>) -> hesoyam::QueryBuilder {
                let mut query_builder = hesoyam::QueryBuilder::delete(#dialect.to_owned());

                query_builder.model(
                    #struct_ident::table_name(),
                    #struct_ident::fields()).
                filter(conditions);

                query_builder
            }
        }
    }
}
