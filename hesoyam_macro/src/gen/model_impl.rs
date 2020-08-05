use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_impl_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let field_name = &ctx.field_name;
    let table_name = &ctx.table_name;
    let field_type = &ctx.field_type;
    let field_ident = &ctx.field_ident;
    let struct_field_ident = &ctx.struct_field_ident;
    let struct_field_type = &ctx.struct_field_type;

    quote! {
        impl #struct_ident {
            pub const field_id: hesoyam::Field = hesoyam::Field {
                name: "id",
                table_name: #table_name,
                field_type: hesoyam::FieldType::Integer,
                is_primary_key: true,
                is_null: false,
            };

            #(
                #[allow(non_upper_case_globals)]
                pub const #field_ident: hesoyam::Field = hesoyam::Field {
                    name: #field_name,
                    table_name: #table_name,
                    field_type: #field_type,
                    is_primary_key: false,
                    is_null: false,
                };
            )*

            pub fn new(#(#struct_field_ident: #struct_field_type),*) -> Self {
                Self {
                    id: None,
                    #(
                        #struct_field_ident,
                    )*
                }
            }
        }

        impl From<hesoyam::client::Row> for #struct_ident {
            fn from(row: hesoyam::client::Row) -> Self {
                let id: hesoyam::error::Result<i64> = row.get("id");

                Self {
                    id: id.ok(),
                    #(
                        #struct_field_ident: row.get(#field_name).unwrap()
                    ),*
                }
            }
        }
    }
}
