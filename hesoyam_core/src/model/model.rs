use crate::model::Field;

pub trait Model {
    fn table_name() -> String;
    fn fields() -> Vec<Field>;
}

// pub struct Model {
//     pub table_name: String,
//     pub fields: Vec<Field>,
// }
//
// impl Model {
//     pub fn from_syn_struct(table_name: String, item: &DataStruct) -> Self {
//         let fields = item.clone().fields.into_iter().
//             map(Field::from_syn_field).
//             collect();
//
//         Self {
//             table_name,
//             fields,
//         }
//     }
// }
