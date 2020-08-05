use darling::FromMeta;
use syn::{AttributeArgs, DataStruct, DeriveInput, Ident};
use syn::export::{Span, ToTokens};
use syn::spanned::Spanned;

use hesoyam_core::FieldType;

#[derive(Debug, FromMeta)]
pub(in crate) struct ModelArgs {
    #[darling(default)]
    pub table_name: Option<String>,
    pub dialect: String,
}

pub(in crate) struct ModelContext {
    pub model_args: ModelArgs,
    pub data_struct: DataStruct,
    pub struct_span: Span,
    pub struct_ident: Ident,
    pub table_name: String,
    pub struct_attrs: Vec<syn::Attribute>,

    pub field_name: Vec<String>,
    pub field_type: Vec<FieldType>,
    pub field_ident: Vec<syn::Ident>,
    pub struct_field_ident: Vec<syn::Ident>,
    pub struct_field_type: Vec<syn::Type>,
}

impl ModelContext {
    pub fn parse(attribute_args: AttributeArgs, derive_input: DeriveInput) -> ModelContext {
        // parse model struct
        let struct_ident = derive_input.ident.clone();
        let struct_span = derive_input.span();
        let data_struct = match derive_input.data {
            syn::Data::Struct(ds) => DataStruct::from(ds.clone()),
            _ => panic!("only struct can be a model"),
        };

        let struct_attrs = derive_input.attrs;

        // parse model args: table_name and dialect
        let model_args = match ModelArgs::from_list(&attribute_args) {
            Ok(v) => v,
            Err(e) => panic!("failed to parse model args: {}", e),
        };
        let table_name = parse_table_name(&struct_ident, &model_args);

        // prepare Vec of field's info
        let field_name: Vec<String> = data_struct.fields.iter().
            map(|f| f.ident.as_ref().unwrap().to_string().clone()).
            collect();

        let field_type: Vec<FieldType> = data_struct.fields.iter().
            map(|f| FieldType::from_type_string(&f.ty.to_token_stream().to_string())).
            collect();

        let field_ident: Vec<syn::Ident> = data_struct.fields.iter().
            map(|f| {
                let name = f.ident.as_ref().unwrap().to_string();
                let name = format!("field_{}", name);

                syn::Ident::new(name.as_str(), f.span())
            }).
            collect();

        let struct_field_ident: Vec<syn::Ident> = data_struct.fields.iter().
            map(|f| {
                let name = f.ident.as_ref().unwrap().to_string();

                syn::Ident::new(name.as_str(), f.span())
            }).
            collect();

        let struct_field_type: Vec<syn::Type> = data_struct.fields.iter().
            map(|f| f.clone().ty).
            collect();

        ModelContext {
            model_args,
            data_struct,
            struct_ident,
            struct_span,
            table_name,
            struct_attrs,

            field_name,
            field_type,
            field_ident,
            struct_field_type,
            struct_field_ident,
        }
    }
}

pub(in crate) fn parse_table_name(struct_name: &Ident, model_args: &ModelArgs) -> String {
    match &model_args.table_name {
        Some(v) => v.clone(),
        None => format!("{}s", struct_name.to_string().to_lowercase()),
    }
}
