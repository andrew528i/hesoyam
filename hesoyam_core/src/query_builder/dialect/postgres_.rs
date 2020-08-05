use std::any::Any;

use chrono::{DateTime, Utc};

use crate::{Condition, DeleteToSql, Field, FieldType, InsertToSql, InsertValue, Operator, QueryBuilder, QueryType, Selectable, SelectToSql, SetValue, UpdateToSql};
use crate::error::*;

pub struct PostgresDialect<'a> {
    query_builder: &'a QueryBuilder,
}

impl<'a> ToString for PostgresDialect<'_> {
    fn to_string(&self) -> String {
        match self.query_builder.query_type {
            QueryType::Insert => self.insert_to_sql(),
            QueryType::Delete => self.delete_to_sql(),
            QueryType::Update => self.update_to_sql(),
            QueryType::Select => self.select_to_sql(),
        }.unwrap()
    }
}

impl<'a> PostgresDialect<'a> {
    pub fn new(query_builder: &'a QueryBuilder) -> Self {
        Self { query_builder }
    }

    pub fn condition_to_sql(&self, condition: &Condition) -> Result<String> {
        let operator = self.operator_to_sql(&condition.operator);
        let condition_value = self.value_to_sql(&condition.value);

        Ok(format!(
            "{name} {operator} {value}",
            name=condition.name,
            operator=operator,
            value=condition_value,
        ))
    }

    fn conditions_to_sql(&self, conditions: &Vec<Condition>) -> Result<String> {
        let condition_values: Vec<String> = conditions.iter().
            map(|c| self.condition_to_sql(c).unwrap()).
            collect();

        Ok(condition_values.join(" and "))
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
            Operator::In => "in",
        }.to_owned()
    }

    pub fn value_to_sql(&self, value: &Box<dyn Any>) -> String {
        let mut str_value: Option<String> = None;

        if let Some(v) = value.downcast_ref::<String>() {
            str_value = Some(format!("'{}'", v.replace("'", "''")));
        }

        if let Some(v) = value.downcast_ref::<Field>() {
            str_value = Some(format!("{}", v.name));
        }

        if let Some(v) = value.downcast_ref::<i32>() {
            str_value = Some(format!("{}", v));
        }

        if let Some(v) = value.downcast_ref::<u32>() {
            str_value = Some(format!("{}", v));
        }

        if let Some(v) = value.downcast_ref::<Vec<String>>() {
            let values = v.iter().
                map(|s| format!("'{}'", s.replace("'", "''"))).
                collect::<Vec<String>>().
                join(",");
            str_value = Some(format!("({})", values));
        }

        if let Some(v) = value.downcast_ref::<Vec<&str>>() {
            let values = v.iter().
                map(|&s| format!("'{}'", s.replace("'", "''"))).
                collect::<Vec<String>>().
                join(",");
            str_value = Some(format!("({})", values));
        }

        if let Some(v) = value.downcast_ref::<Vec<i64>>() {
            let values = v.iter().
                map(|&s| format!("{}", s)).
                collect::<Vec<String>>().
                join(",");
            str_value = Some(format!("({})", values));
        }

        match str_value {
            Some(v) => v,
            None => panic!("failed to compile sql_value: {:#?}", value)
        }
    }

    pub fn update_values_to_sql(&self, values: &SetValue) -> String {
        let mut update_values = Vec::new();

        for f in values.keys() {
            let v = values.get(f).unwrap();
            let v = self.value_to_sql(v);

            update_values.push(
                format!(
                    "{field_name}={value}",
                    field_name=f.name,
                    value=v))
        }

        update_values.join(",")
    }
}

impl<'a> InsertToSql for PostgresDialect<'_> {
    fn insert_to_sql(&self) -> Result<String> {
        let table_name = &self.query_builder.insert_clause.table_name;
        let model_fields = &self.query_builder.insert_clause.fields;
        let insert_values = &self.query_builder.insert_clause.values;

        let fields = self.insert_fields_to_sql(table_name, model_fields)?;
        let values = self.insert_values_to_sql(model_fields, insert_values)?;

        Ok(format!(
            "insert into {table_name} {fields} values {values};",
            table_name=table_name,
            fields=fields,
            values=values,
        ))
    }

    fn insert_fields_to_sql(&self, _table_name: &String, fields: &Vec<Field>) -> Result<String> {
        let field_names: Vec<String> = fields.iter().
            filter(|&f| !f.is_primary_key).
            map(|f| format!("{}", f.name)).
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
            let field_value = match value.get(f) {
                Some(v) => v,
                None => {
                    if f.is_primary_key {
                        continue
                    } else {
                        panic!("required field not found: {}", f.name)
                    }
                },
            };

            let insert_value: String = match &f.field_type {
                FieldType::String => {
                    let v = field_value.downcast_ref::<String>().unwrap();

                    format!("'{}'", v.replace("'", "''"))
                },

                FieldType::SmallUnsignedInteger => field_value.downcast_ref::<u8>().unwrap().to_string(),
                FieldType::UnsignedInteger => field_value.downcast_ref::<u32>().unwrap().to_string(),
                FieldType::BigUnsignedInteger => field_value.downcast_ref::<u64>().unwrap().to_string(),
                FieldType::SmallInteger => field_value.downcast_ref::<i8>().unwrap().to_string(),
                FieldType::Integer => field_value.downcast_ref::<i32>().unwrap().to_string(),
                FieldType::BigInteger => field_value.downcast_ref::<i64>().unwrap().to_string(),

                FieldType::Float => field_value.downcast_ref::<f32>().unwrap().to_string(),
                FieldType::BigFloat => field_value.downcast_ref::<f64>().unwrap().to_string(),

                FieldType::Boolean => {
                    let v = field_value.downcast_ref::<bool>().unwrap();

                    match v {
                        true => "TRUE".to_owned(),
                        false => "FALSE".to_owned(),
                    }
                },

                FieldType::DateTime => {
                    let value = field_value.downcast_ref::<DateTime<Utc>>().unwrap();

                    format!("'{}'", value.format("%Y-%m-%d %H:%M:%S %:z").to_string())
                }

                FieldType::Array(_) => unimplemented!(),
                FieldType::Enum(_) => unimplemented!(),
            };

            value_parts.push(insert_value);
        }

        let insert_value = format!("({})", value_parts.join(","));

        Ok(insert_value)
    }
}

impl<'a> DeleteToSql for PostgresDialect<'_> {
    fn delete_to_sql(&self) -> Result<String> {
        let conditions = self.conditions_to_sql(&self.query_builder.where_clause.conditions)?;

        Ok(format!(
            "delete from {table_name} where {conditions};",
            table_name=self.query_builder.delete_clause.table_name,
            conditions=conditions,
        ))
    }
}

impl<'a> UpdateToSql for PostgresDialect<'_> {
    fn update_to_sql(&self) -> Result<String> {
        let table_name = &self.query_builder.update_clause.table_name;
        let set_values = self.update_values_to_sql(&self.query_builder.update_clause.values);

        let query = match &self.query_builder.where_clause.conditions.len() {
            0 => format!(
                "update {table_name} set {set_values}",
                table_name=table_name,
                set_values=set_values),
            _ => {
                let where_ = self.conditions_to_sql(&self.query_builder.where_clause.conditions)?;

                format!(
                    "update {table_name} set {set_values} where {where_}",
                    table_name=table_name,
                    set_values=set_values,
                    where_=where_)
            },
        };

        Ok(query)
    }
}

impl<'a> SelectToSql for PostgresDialect<'_> {
    fn select_to_sql(&self) -> Result<String> {
        let table_name = &self.query_builder.select_clause.table_name;
        let fields = self.select_fields_to_sql(&self.query_builder.select_clause.values);

        let mut result = format!(
            "select {fields} from {table_name}",
            fields=fields,
            table_name=table_name);

        if self.query_builder.where_clause.conditions.len() > 0 {
            let where_ = self.conditions_to_sql(&self.query_builder.where_clause.conditions)?;

            result.push_str(format!(" where {where_}", where_=where_).as_str())
        }

        if let Some(limit) = self.query_builder.limit_clause.limit {
            result.push_str(format!(" limit {}", limit).as_str());

            if let Some(offset) = self.query_builder.limit_clause.offset {
                result.push_str(format!(" offset {}", offset).as_str());
            }
        }

        result.push_str(";");

        Ok(result)
    }

    fn select_fields_to_sql(&self, values: &Vec<Selectable>) -> String {
        let values: Vec<String> = values.iter().
            map(|s| self.select_value_to_sql(s)).
            collect();

        values.join(",")
    }

    fn select_value_to_sql(&self, value: &Selectable) -> String {
        match value {
            Selectable::Field(f) => format!("{}.{}", f.table_name, f.name),
            Selectable::All => "*".to_owned(),
            _ => unimplemented!(),
        }
    }
}
