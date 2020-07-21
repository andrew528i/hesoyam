pub struct DeleteClause {
    pub table_name: String,
}

impl Default for DeleteClause {
    fn default() -> Self {
        Self { table_name: "".to_owned() }
    }
}
