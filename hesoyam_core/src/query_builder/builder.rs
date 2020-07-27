use crate::{ClickhouseDialect, Condition, DeleteClause, Field, InsertClause, InsertValue, PostgresDialect, Selectable, SelectClause, SetValue, UpdateClause, WhereClause};
use crate::client::{ClientManager, QueryResult};
use crate::error::*;

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
    pub select_clause: SelectClause,
}

pub trait ToSql {
    fn to_sql(&self) -> Result<String>;
}

impl ToSql for QueryBuilder {
    fn to_sql(&self) -> Result<String> {
        // TODO: implement Dialect trait
        let dialect: Box<dyn ToString> = match self.dialect.as_str() {
            // TODO: dialect name -> const
            "postgres" => Box::new(PostgresDialect::new(&self)),
            "clickhouse" => Box::new(ClickhouseDialect::new(&self)),
            _ => unimplemented!(),
        };

        Ok(dialect.to_string())
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
            select_clause: SelectClause::default(),
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
            select_clause: SelectClause::default(),
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
            select_clause: SelectClause::default(),
        }
    }

    pub fn select_(dialect: String, values: Vec<Selectable>) -> Self {
        Self {
            query_type: QueryType::Select,
            dialect,
            insert_clause: InsertClause::default(),
            delete_clause: DeleteClause::default(),
            update_clause: UpdateClause::default(),
            where_clause: WhereClause::default(),
            select_clause: SelectClause::from_values(values),
        }
    }

    pub fn select(&mut self, values: Vec<Selectable>) -> &mut Self {
        self.select_clause.values.extend(values);

        self
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
            },
            QueryType::Select => {
                self.select_clause.table_name = table_name;
            }
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

    pub fn exec(&self, client_manager: &mut ClientManager) -> Result<QueryResult> {
        let client = client_manager.get_client(&self.dialect).unwrap();
        let query = self.to_sql().unwrap();

        client.query(query.as_str())
    }
}
