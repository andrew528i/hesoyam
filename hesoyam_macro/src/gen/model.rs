use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let table_name = &ctx.table_name;
    let field_name = &ctx.field_name;
    let field_type = &ctx.field_type;
    let struct_field_ident = &ctx.struct_field_ident;
    let struct_field_type = &ctx.struct_field_type;

    let struct_attrs = &ctx.struct_attrs;

    let model_ident = struct_ident.to_string();
    let model_ident = format!("Model{}", model_ident);
    let model_ident = syn::Ident::new(model_ident.as_str(), struct_ident.span());

    quote! {
        use hesoyam::Model as #model_ident;

        #(#struct_attrs)*
        pub struct #struct_ident {
            pub id: Option<i64>,
            #(
                pub #struct_field_ident: #struct_field_type,
            )*
        }

        impl #model_ident for #struct_ident {
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
