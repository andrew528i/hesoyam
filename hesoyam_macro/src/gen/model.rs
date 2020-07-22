use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let table_name = &ctx.table_name;
    let field_name = &ctx.field_name;
    let field_type = &ctx.field_type;

    quote! {
        use hesoyam::Model as _Model;

        impl _Model for #struct_ident {
            fn table_name() -> String { String::from(#table_name) }

            fn fields() -> Vec<hesoyam::Field> {
                let mut fields = Vec::new();
                let id_field = hesoyam::Field {
                    name: "id",
                    table_name: #table_name,
                    field_type: hesoyam::FieldType::Integer,
                    is_primary_key: true,
                    is_null: false,
                };

                fields.push(id_field);

                #(
                    let name = #field_name;
                    let field_type = #field_type;
                    let field = hesoyam::Field {
                        name,
                        field_type,
                        table_name: #table_name,
                        is_primary_key: false,
                        is_null: false,
                    };

                    fields.push(field);
                )*

                fields
            }
        }
    }
}
