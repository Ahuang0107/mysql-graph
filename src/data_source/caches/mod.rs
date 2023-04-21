use std::collections::HashMap;

pub type Schemas = HashMap<String, Tables>;
pub type Tables = HashMap<String, Vec<TableColumn>>;

#[derive(Default, Debug, Clone)]
pub struct Caches {
    pub schemas: Schemas,
}

#[derive(Debug, Clone)]
pub struct TableColumn {
    pub field: String,
    pub type_: String,
    pub null: String,
    pub key: String,
    pub default: String,
    pub extra: String,
}

impl TableColumn {
    pub fn from_field(field: &str) -> Self {
        Self {
            field: field.to_string(),
            type_: String::new(),
            null: String::new(),
            key: String::new(),
            default: String::new(),
            extra: String::new(),
        }
    }
}
