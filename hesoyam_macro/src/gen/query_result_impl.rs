use quote::quote;
use syn::{DataStruct, DeriveInput};
use syn::export::TokenStream2;
use syn::spanned::Spanned;

pub(in crate) fn gen_query_result_impl(derive_input: DeriveInput) -> TokenStream2 {
    let struct_ident = derive_input.ident.clone();
    let data_struct = match derive_input.data {
        syn::Data::Struct(ds) => DataStruct::from(ds.clone()),
        _ => panic!("only struct can be a model"),
    };

    let field_name: Vec<String> = data_struct.fields.iter().
        map(|f| f.ident.as_ref().unwrap().to_string().clone()).
        collect();

    let struct_field_ident: Vec<syn::Ident> = data_struct.fields.iter().
        map(|f| {
            let name = f.ident.as_ref().unwrap().to_string();

            syn::Ident::new(name.as_str(), f.span())
        }).
        collect();

    quote! {
        impl From<hesoyam::client::Row> for #struct_ident {
            fn from(row: hesoyam::client::Row) -> Self {
                Self {
                    #(
                        #struct_field_ident: row.get(#field_name).unwrap()
                    ),*
                }
            }
        }
    }
}
