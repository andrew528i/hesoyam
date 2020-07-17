use quote::quote;
use syn::export::TokenStream2;

use crate::context::ModelContext;

pub(in crate) fn gen_model_impl_code(ctx: &ModelContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let field_name = &ctx.field_name;
    let field_type = &ctx.field_type;
    let field_ident = &ctx.field_ident;

    quote! {
        impl #struct_ident {
            #(
                #[allow(non_upper_case_globals)]
                const #field_ident: hesoyam::Field = hesoyam::Field {
                    name: #field_name,
                    field_type: #field_type,
                };
            )*
        }
    }
}
