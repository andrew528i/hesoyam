use quote::quote;
use syn::{DeriveInput, AttributeArgs, parse_macro_input};
use syn::export::{TokenStream, ToTokens};
use syn::spanned::Spanned;
use hesoyam_core::FieldType;
use darling::FromMeta;
// use proc_macro::TokenStream as ArgsTokenStream;

type Dialect = String;

#[derive(Debug, FromMeta)]
struct ModelArgs {
    #[darling(default)]
    table_name: Option<String>,
    #[darling(default)]
    dialect: Dialect,
}

#[proc_macro_attribute]
pub fn model(args: TokenStream, input: TokenStream) -> TokenStream {
    let macro_args = parse_macro_input!(args as AttributeArgs);
    let model_args = match ModelArgs::from_list(&macro_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let ast: DeriveInput = syn::parse(input).unwrap();
    let struct_name = &ast.ident;
    let table_name = match model_args.table_name {
        Some(v) => v,
        None => parse_table_name(&ast),
    };
    let data_struct = match ast.data {
        syn::Data::Struct(ref ds) => ds,
        _ => panic!("only struct can be model"),
    };

    // TODO: let (field_name, field_type, field_ident) = ...
    let field_name: Vec<String> = data_struct.fields.iter().
        map(|f| f.ident.as_ref().unwrap().to_string().clone()).
        collect();

    let field_type: Vec<String> = data_struct.fields.iter().
        map(|f| f.ty.to_token_stream().to_string()).
        collect();

    let field_raw_type: Vec<syn::Type> = data_struct.fields.iter().
        map(|f| f.clone().ty).
        collect();

    let field_internal_type: Vec<FieldType> = field_type.iter().
        map(|f| FieldType::from_type_string(&f)).
        collect();

    let field_ident: Vec<syn::Ident> = data_struct.fields.iter().
        map(|f| {
            let name = f.ident.as_ref().unwrap().to_string();
            let name = format!("field_{}", name);

            syn::Ident::new(name.as_str(), f.span())
        }).
        collect();

    let ident: Vec<syn::Ident> = data_struct.fields.iter().
        map(|f| {
            let name = f.ident.as_ref().unwrap().to_string();

            syn::Ident::new(name.as_str(), f.span())
        }).
        collect();

    let insert_one_ident = format!("{}InsertOne", struct_name.to_string());
    let insert_one_ident = syn::Ident::new(&insert_one_ident, ast.span());

    let insert_many_ident = format!("{}InsertMany", struct_name.to_string());
    let insert_many_ident = syn::Ident::new(&insert_many_ident, ast.span());

    let input = quote! {
        use hesoyam::Model as MModel;

        #ast

        impl #struct_name {
            #(
                #[allow(non_upper_case_globals)]
                const #field_ident: hesoyam::Field = hesoyam::Field {
                    name: #field_name,
                    field_type: #field_internal_type,
                };
            )*
        }

        impl MModel for #struct_name {
            fn table_name() -> String { String::from(#table_name) }

            fn fields() -> Vec<hesoyam::Field> {
                let mut fields = Vec::new();

                #(
                    let name = #field_name;
                    let field_type = hesoyam::FieldType::from_type_string(#field_type);
                    let field = hesoyam::Field { name, field_type };

                    fields.push(field);
                )*

                fields
            }
        }

        trait #insert_one_ident {
            fn insert(#(#ident: #field_raw_type),*) -> hesoyam::QueryBuilder;
        }

        impl #insert_one_ident for #struct_name {
            fn insert(#(#ident: #field_raw_type),*) -> hesoyam::QueryBuilder {
                let mut value: std::collections::HashMap<hesoyam::Field, Box<dyn Any>> = std::collections::HashMap::new();

                #(
                    value.insert(#struct_name::#field_ident, Box::new(#ident));
                )*

                hesoyam::QueryBuilder::insert(
                    #struct_name::table_name(),
                    #struct_name::fields(),
                    vec![value])
            }
        }

        trait #insert_many_ident {
            fn insert_many(&self) -> hesoyam::QueryBuilder;
        }

        impl #insert_many_ident for Vec<#struct_name> {
            fn insert_many(&self) -> hesoyam::QueryBuilder {
                let mut values: Vec<std::collections::HashMap<hesoyam::Field, Box<dyn Any>>> = Vec::new();

                for v in self.iter() {
                    let mut value: std::collections::HashMap<hesoyam::Field, Box<dyn Any>> = std::collections::HashMap::new();

                    #(
                        value.insert(#struct_name::#field_ident, Box::new(v.#ident.clone()));
                    )*

                    values.push(value);
                }

                hesoyam::QueryBuilder::insert(
                    #struct_name::table_name(),
                    #struct_name::fields(),
                    values)
            }
        }
    };

    TokenStream::from(input)
}

fn parse_table_name(ast: &DeriveInput) -> String {
    let table_name = ast.ident.to_string();

    format!("{}s", table_name.to_lowercase())
}
