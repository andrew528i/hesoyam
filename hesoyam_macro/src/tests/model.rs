pub(in crate) const DIALECT: &'static str = "postgres";
pub(in crate) const TABLE_NAME: &'static str = "custom_table_name";

#[model(dialect = DIALECT, table_name = TABLE_NAME)]
pub(in crate) struct TestModel {
    age: i32,
    name: String,
    weight: f32,
}
