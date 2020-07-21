use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_update_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let dialect = &ctx.model_args.dialect;
    let struct_span = &ctx.struct_span;
    let field_ident = &ctx.field_ident;
    let struct_field_type = &ctx.struct_field_type;
    let struct_field_ident = &ctx.struct_field_ident;

    let update_ident = format!("{}Update", struct_ident.to_string());
    let update_ident = syn::Ident::new(&update_ident, struct_span.clone());

    quote! {
        use hesoyam::ToSql as _ToSql;

        pub trait #update_ident {
            fn update(#(#field_ident: #struct_field_type),*) -> hesoyam::CompiledQuery;
        }

        impl #update_ident for #struct_ident {
            fn update(#(#field_ident: #struct_field_type),*) -> hesoyam::CompiledQuery {
                // let mut value: std::collections::HashMap<hesoyam::Field, Box<dyn std::any::Any>> = std::collections::HashMap::new();
                //
                // #(
                //     value.insert(#struct_ident::#field_ident, Box::new(#field_ident.clone()));
                // )*
                //
                // hesoyam::QueryBuilder::insert(#dialect.to_owned()).
                //     model(
                //         #struct_ident::table_name(),
                //         #struct_ident::fields()).
                //     values(vec![value]).
                //     to_sql().unwrap()
            }
        }
    }
}
