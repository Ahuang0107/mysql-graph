#[allow(unused)]
use std::collections::HashMap;

#[derive(Default)]
pub struct Cache {
    pub tables: HashMap<String, Vec<TableColumn>>,
}

pub struct TableColumn {
    pub field: String,
    pub type_: String,
    pub null: String,
    pub key: String,
    pub default: String,
    pub extra: String,
}
