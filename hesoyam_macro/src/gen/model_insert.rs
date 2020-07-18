use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_insert_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let dialect = &ctx.model_args.dialect;
    let struct_span = &ctx.struct_span;
    let field_ident = &ctx.field_ident;
    let struct_field_type = &ctx.struct_field_type;
    let struct_field_ident = &ctx.struct_field_ident;

    let insert_one_ident = format!("{}InsertOne", struct_ident.to_string());
    let insert_one_ident = syn::Ident::new(&insert_one_ident, struct_span.clone());

    let insert_many_ident = format!("{}InsertMany", struct_ident.to_string());
    let insert_many_ident = syn::Ident::new(&insert_many_ident, struct_span.clone());

    quote! {
        pub trait #insert_one_ident {
            fn save(#(#field_ident: #struct_field_type),*) -> hesoyam::QueryBuilder;
        }

        impl #insert_one_ident for #struct_ident {
            fn save(#(#field_ident: #struct_field_type),*) -> hesoyam::QueryBuilder {
                let mut value: std::collections::HashMap<hesoyam::Field, Box<dyn std::any::Any>> = std::collections::HashMap::new();

                #(
                    value.insert(#struct_ident::#field_ident, Box::new(#field_ident));
                )*

                hesoyam::QueryBuilder::insert(
                    #dialect.to_owned(),
                    #struct_ident::table_name(),
                    #struct_ident::fields(),
                    vec![value])
            }
        }

        pub trait #insert_many_ident {
            fn save(&self) -> hesoyam::QueryBuilder;
        }

        impl #insert_many_ident for Vec<#struct_ident> {
            fn save(&self) -> hesoyam::QueryBuilder {
                let mut values: Vec<std::collections::HashMap<hesoyam::Field, Box<dyn std::any::Any>>> = Vec::new();

                for v in self.iter() {
                    let mut value: std::collections::HashMap<hesoyam::Field, Box<dyn std::any::Any>> = std::collections::HashMap::new();

                    #(
                        value.insert(#struct_ident::#field_ident, Box::new(v.#struct_field_ident.clone()));
                    )*

                    values.push(value);
                }

                hesoyam::QueryBuilder::insert(
                    #dialect.to_owned(),
                    #struct_ident::table_name(),
                    #struct_ident::fields(),
                    values)
            }
        }
    }
}
