pub type DbColumnStatement = (String, String, String, String, Option<String>, String);

#[allow(dead_code)]
pub type ColumnsStatement = Vec<ColumnStatement>;

#[derive(Debug, Clone)]
pub struct ColumnStatement {
    pub field: String,
    pub type_: String,
    pub null: String,
    pub key: String,
    pub default: Option<String>,
    pub extra: String,
}

impl From<DbColumnStatement> for ColumnStatement {
    fn from(value: DbColumnStatement) -> Self {
        ColumnStatement {
            field: value.0,
            type_: value.1,
            null: value.2,
            key: value.3,
            default: value.4,
            extra: value.5,
        }
    }
}
