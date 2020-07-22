use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_select_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let struct_span = &ctx.struct_span;
    let dialect = &ctx.model_args.dialect;
    let field_ident = &ctx.field_ident;

    let select_ident = format!("{}Select", struct_ident.to_string());
    let select_ident = syn::Ident::new(&select_ident, struct_span.clone());

    // TODO: add params: select(func::avg(User::field_age))
    quote! {
        pub trait #select_ident {
            fn select() -> hesoyam::QueryBuilder;
        }

        impl #select_ident for #struct_ident {
            fn select() -> hesoyam::QueryBuilder {
                let mut query_builder = hesoyam::QueryBuilder::select_(
                    #dialect.to_owned(),
                    vec![
                        #struct_ident::field_id.into(),
                        #(#struct_ident::#field_ident.into()),*
                    ]);

                query_builder.model(
                    #struct_ident::table_name(),
                    #struct_ident::fields(),
                );

                query_builder
            }
        }
    }
}
