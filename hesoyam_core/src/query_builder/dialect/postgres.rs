use crate::{Field, FieldType, InsertQueryBuilder, InsertValue, QueryBuilder, QueryBuilderType, Result};
use crate::query_builder::Dialect;

pub struct PostgresDialect<'a> {
    query_builder: &'a QueryBuilder,
}

impl<'a> PostgresDialect<'a> {
    pub fn new(query_builder: &'a QueryBuilder) -> Self {
        Self { query_builder }
    }

    fn insert_to_sql(&self, builder: &InsertQueryBuilder) -> Result<String> {
        let fields = self.insert_fields_to_sql(&builder.table_name, &builder.fields)?;
        let values = self.insert_values_to_sql(&builder.fields, &builder.values)?;

        Ok(format!(
            "insert into `{table_name}` `{fields}` values {values};",
            table_name=builder.table_name,
            fields=fields,
            values=values,
        ))
    }

    fn insert_fields_to_sql(&self, table_name: &String, fields: &Vec<Field>) -> Result<String> {
        let field_names: Vec<String> = fields.iter().
            map(|f| format!("`{}`.`{}`", table_name, f.name)).
            collect();

        let insert_fields = format!("({})", field_names.join(","));

        Ok(insert_fields)
    }

    fn insert_values_to_sql(
        &self,
        fields: &Vec<Field>,
        values: &Vec<InsertValue>,
    ) -> Result<String> {
        let insert_values: Vec<String> = values.iter().
            map(|v| self.insert_value_to_sql(fields, v).unwrap()).
            collect();

        let insert_values = insert_values.join(",");

        Ok(insert_values)
    }

    fn insert_value_to_sql(&self, fields: &Vec<Field>, value: &InsertValue) -> Result<String> {
        let mut value_parts = Vec::new();

        for f in fields.iter() {
            let field_value = value.get(f).unwrap();
            let insert_value: String = match f.field_type {
                FieldType::String => {
                    let v = field_value.downcast_ref::<String>().unwrap();

                    format!("'{}'", v)
                },

                FieldType::SmallUnsignedInteger => field_value.downcast_ref::<u8>().unwrap().to_string(),
                FieldType::UnsignedInteger => field_value.downcast_ref::<u32>().unwrap().to_string(),
                FieldType::BigUnsignedInteger => field_value.downcast_ref::<u64>().unwrap().to_string(),
                FieldType::SmallInteger => field_value.downcast_ref::<i8>().unwrap().to_string(),
                FieldType::Integer => field_value.downcast_ref::<i32>().unwrap().to_string(),
                FieldType::BigInteger => field_value.downcast_ref::<i64>().unwrap().to_string(),

                FieldType::Boolean => {
                    let v = field_value.downcast_ref::<bool>().unwrap();

                    match v {
                        true => "TRUE".to_owned(),
                        false => "FALSE".to_owned(),
                    }
                },

                FieldType::Array(_) => unimplemented!(),
                FieldType::Enum(_) => unimplemented!(),
            };

            value_parts.push(insert_value);
        }

        let insert_value = format!("({})", value_parts.join(","));

        Ok(insert_value)
    }
}

impl<'a> Dialect for PostgresDialect<'a> {
    fn to_sql(&self) -> Result<String> {
        match &self.query_builder.builder_type {
            QueryBuilderType::Insert(builder) => self.insert_to_sql(builder),
            _ => unimplemented!(),
        }
    }
}
