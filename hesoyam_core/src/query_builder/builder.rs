use crate::{Field, Result, InsertClause, PostgresDialect, CompiledQuery, DeleteClause, Condition, WhereClause, UpdateClause, SetValue, InsertValue};

#[derive(Debug)]
pub enum QueryType {
    Insert,
    Delete,
    Update,
    Select,
}

pub struct QueryBuilder {
    pub query_type: QueryType,
    pub dialect: String,
    pub insert_clause: InsertClause,
    pub delete_clause: DeleteClause,
    pub update_clause: UpdateClause,
    pub where_clause: WhereClause,
}

pub trait ToSql {
    fn to_sql(&self) -> Result<CompiledQuery>;
}

impl ToSql for QueryBuilder {
    fn to_sql(&self) -> Result<CompiledQuery> {
        let dialect = match self.dialect.as_str() {
            // TODO: dialect name -> const
            "postgres" => {
                PostgresDialect::new(&self)
            },
            _ => unimplemented!(),
        };

        Ok(CompiledQuery {
            dialect: self.dialect.clone(),
            query: dialect.to_string(),
        })
    }
}

impl QueryBuilder {
    pub fn insert(dialect: String) -> Self {
        Self {
            query_type: QueryType::Insert,
            dialect,
            insert_clause: InsertClause::default(),
            delete_clause: DeleteClause::default(),
            update_clause: UpdateClause::default(),
            where_clause: WhereClause::default(),
        }
    }

    pub fn delete(dialect: String) -> Self {
        Self {
            query_type: QueryType::Delete,
            dialect,
            insert_clause: InsertClause::default(),
            delete_clause: DeleteClause::default(),
            update_clause: UpdateClause::default(),
            where_clause: WhereClause::default(),
        }
    }

    pub fn update(dialect: String) -> Self {
        Self {
            query_type: QueryType::Update,
            dialect,
            insert_clause: InsertClause::default(),
            delete_clause: DeleteClause::default(),
            update_clause: UpdateClause::default(),
            where_clause: WhereClause::default(),
        }
    }

    pub fn model(&mut self, table_name: String, fields: Vec<Field>) -> &mut Self {
        match &self.query_type {
            QueryType::Insert => {
                self.insert_clause.table_name = table_name;
                self.insert_clause.fields = fields;
            },
            QueryType::Delete => {
                self.delete_clause.table_name = table_name;
            },
            QueryType::Update => {
                self.update_clause.table_name = table_name;
            }
            _ => unimplemented!(),
        }

        self
    }

    pub fn values(&mut self, values: Vec<InsertValue>) -> &mut Self {
        match &self.query_type {
            QueryType::Insert => {
                self.insert_clause.values = values;
            },
            _ => unimplemented!(),
        }

        self
    }

    pub fn filter(&mut self, conditions: Vec<Condition>) -> &mut Self {
        self.where_clause.conditions = conditions;

        self
    }

    pub fn set(&mut self, values: SetValue) -> &mut Self {
        match &self.query_type {
            QueryType::Update => {
                self.update_clause.values = values;
            },
            _ => unimplemented!(),
        };

        self
    }

    // pub fn build(&self) -> Self {
    //     Self {
    //         query_type: self.query_type.clone(),
    //         dialect: self.dialect.clone(),
    //         insert_clause: self.insert_clause.clone(),
    //         delete_clause: self.delete_clause.clone(),
    //         update_clause: self.update_clause.clone(),
    //         where_clause: self.where_clause.clone(),
    //     }
    // }
    //
    // pub fn delete() -> Self {
    //
    // }
    //
    // pub fn update() -> Self {
    //
    // }
    //
    // pub fn select() -> Self {
    //
    // }

    // pub fn insert(
    //     dialect: String,
    //     table_name: String,
    //     fields: Vec<Field>,
    //     values: Vec<InsertValue>,
    // ) -> InsertQueryBuilder {
    //     InsertQueryBuilder { dialect, table_name, fields, values }
    // }
    //
    // pub fn delete(
    //     dialect: String,
    //     table_name: String,
    //     conditions: Vec<Condition>,
    // ) -> DeleteQueryBuilder {
    //     DeleteQueryBuilder { dialect, table_name, conditions }
    // }
    //
    // pub fn update(
    //     dialect: String,
    //     table_name: String,
    //     update_fields: Vec<UpdateField>,
    // ) -> UpdateQueryBuilder {
    //     UpdateQueryBuilder {
    //         dialect,
    //         table_name,
    //         update_fields,
    //         filters: vec![],
    //     }
    // }
}
