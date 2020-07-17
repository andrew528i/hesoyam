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

                #(
                    let name = #field_name;
                    let field_type = #field_type;
                    let field = hesoyam::Field { name, field_type };

                    fields.push(field);
                )*

                fields
            }
        }
    }
}
