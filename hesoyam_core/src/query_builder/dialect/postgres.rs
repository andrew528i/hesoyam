use crate::{Field, FieldType, InsertQueryBuilder, InsertValue, QueryBuilder, QueryBuilderType, Result, DeleteQueryBuilder, Condition, Operator};
use crate::query_builder::Dialect;

pub struct PostgresDialect<'a> {
    query_builder: &'a QueryBuilder,
}

impl<'a> PostgresDialect<'a> {
    pub fn new(query_builder: &'a QueryBuilder) -> Self {
        Self { query_builder }
    }

    // insert
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

    // delete
    pub fn delete_to_sql(&self, builder: &DeleteQueryBuilder) -> Result<String> {
        let conditions = self.delete_conditions_to_sql(&builder.conditions)?;

        Ok(format!(
            "delete from `{table_name}` where {conditions};",
            table_name=builder.table_name,
            conditions=conditions,
        ))
    }

    pub fn delete_conditions_to_sql(&self, conditions: &Vec<Condition>) -> Result<String> {
        let condition_values: Vec<String> = conditions.iter().
            map(|c| self.condition_to_sql(c).unwrap()).
            collect();

        Ok(condition_values.join(" and "))
    }

    pub fn condition_to_sql(&self, condition: &Condition) -> Result<String> {
        let operator = self.operator_to_sql(&condition.operator);
        let mut condition_value: Option<String> = None;

        if let Some(v) = condition.value.downcast_ref::<String>() {
            condition_value = Some(format!("'{}'", v));
        }

        if let Some(v) = condition.value.downcast_ref::<Field>() {
            condition_value = Some(format!("`{}`", v.name));
        }

        if let Some(v) = condition.value.downcast_ref::<i32>() {
            condition_value = Some(format!("{}", v));
        }

        if let Some(v) = condition.value.downcast_ref::<u32>() {
            condition_value = Some(format!("{}", v));
        }

        let condition_value = match condition_value {
            Some(v) => v,
            None => return Err(
                format!("failed to compile condition_value: {:#?}", condition.value).into())
        };

        Ok(format!(
            "{name} {operator} {value}",
            name=condition.name,
            operator=operator,
            value=condition_value,
        ))
    }

    pub fn operator_to_sql(&self, operator: &Operator) -> String {
        match operator {
            Operator::Eq => "=",
            Operator::NotEq => "<>",
            Operator::Lt => "<",
            Operator::Lte => "<=",
            Operator::Gt => ">",
            Operator::Gte => ">=",
            Operator::Like => "like",
            Operator::Is => "is",
            Operator::IsNot => "is not",
        }.to_owned()
    }
}

impl<'a> Dialect for PostgresDialect<'a> {
    fn to_sql(&self) -> Result<String> {
        match &self.query_builder.builder_type {
            QueryBuilderType::Insert(builder) => self.insert_to_sql(builder),
            QueryBuilderType::Delete(builder) => self.delete_to_sql(builder),
            _ => unimplemented!(),
        }
    }
}
