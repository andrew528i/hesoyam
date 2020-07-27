use crate::tests::model::*;

#[test]
fn test_model_impl() {
    assert_eq!(TestModel::table_name(), TABLE_NAME);
}
