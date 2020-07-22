use crate::{Result, ToSql, Field, InsertValue, Selectable};

pub trait Dialect: ToSql {
    // create_schema(&self) -> Result<String>;
    // to_sql()
    // execute()
}

pub(in crate) trait InsertToSql {
    fn insert_to_sql(&self) -> Result<String>;

    fn insert_fields_to_sql(
        &self,
        table_name: &String,
        fields: &Vec<Field>,
    ) -> Result<String>;

    fn insert_values_to_sql(
        &self,
        fields: &Vec<Field>,
        values: &Vec<InsertValue>,
    ) -> Result<String>;

    fn insert_value_to_sql(
        &self,
        fields: &Vec<Field>,
        value: &InsertValue,
    ) -> Result<String>;
}

pub(in crate) trait DeleteToSql {
    fn delete_to_sql(&self) -> Result<String>;
}

pub(in crate) trait UpdateToSql {
    fn update_to_sql(&self) -> Result<String>;
}

pub(in crate) trait SelectToSql {
    fn select_to_sql(&self) -> Result<String>;
    fn select_fields_to_sql(&self, values: &Vec<Selectable>) -> String;
    fn select_value_to_sql(&self, value: &Selectable) -> String;
}
