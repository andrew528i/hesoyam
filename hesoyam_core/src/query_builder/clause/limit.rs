#[derive(Default)]
pub struct LimitClause {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
