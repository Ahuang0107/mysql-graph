use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use mysql_async::prelude::Query;
use mysql_async::{Conn, Pool};
use tokio::sync::Mutex;

use caches::Caches;
use caches::Schemas;
pub use caches::TableColumn;

mod caches;

pub struct DataSource {
    pub url: String,
    conn: Arc<Mutex<Conn>>,
    caches: Arc<std::sync::Mutex<Caches>>,
}

impl DataSource {
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            url: url.to_string(),
            conn: Arc::new(Mutex::new(Pool::from_url(url)?.get_conn().await?)),
            caches: Arc::new(std::sync::Mutex::new(Caches::default())),
        })
    }
    pub fn flash_schemas(&mut self) {
        let conn = self.conn.clone();
        let caches = self.caches.clone();
        tokio::spawn(async move {
            let mut conn = conn.lock().await;
            if let Ok(schemas) = "show schemas;"
                .fetch::<String, &mut Conn>(conn.borrow_mut())
                .await
            {
                let mut caches = caches.lock().unwrap();
                {
                    // 添加新的schema
                    for schema in schemas.iter() {
                        if !caches.schemas.contains_key(schema) {
                            caches.schemas.insert(schema.clone(), HashMap::new());
                        }
                    }
                }
                {
                    // 删除已经没有的schema
                    let mut need_delete_keys = vec![];
                    for key in caches.schemas.keys() {
                        if !schemas.contains(key) {
                            need_delete_keys.push(key.clone());
                        }
                    }
                    for key in need_delete_keys {
                        caches.schemas.remove(&key);
                    }
                }
            }
        });
    }
    pub fn flash_tables(&mut self, schema: &str) {
        let conn = self.conn.clone();
        let caches = self.caches.clone();
        let schema = schema.to_string();
        tokio::spawn(async move {
            let mut conn = conn.lock().await;
            format!("use {schema};")
                .ignore::<&mut Conn>(conn.borrow_mut())
                .await
                .expect(&format!("unable to use `{schema}`"));
            if let Ok(r_tables) = "show tables;"
                .fetch::<String, &mut Conn>(conn.borrow_mut())
                .await
            {
                let mut caches = caches.lock().unwrap();
                let tables = caches.schemas.get_mut(&schema).unwrap();
                tables.clear();
                for r_table_name in r_tables {
                    tables.insert(r_table_name, vec![]);
                }
            }
        });
    }
    pub fn flash_table_columns(&mut self, schema: &str, table: &str) {
        let conn = self.conn.clone();
        let caches = self.caches.clone();
        let schema = schema.to_string();
        let table = table.to_string();
        tokio::spawn(async move {
            let mut conn = conn.lock().await;
            if let Ok(r_columns) = format!("show columns from `{schema}`.`{table}`")
                .fetch::<(String, String, String, String, Option<String>, String), &mut Conn>(
                    conn.borrow_mut(),
                )
                .await
            {
                println!("{r_columns:?}");
                let mut caches = caches.lock().unwrap();
                let columns = caches
                    .schemas
                    .get_mut(&schema)
                    .unwrap()
                    .get_mut(&table)
                    .unwrap();
                columns.clear();
                for (field, _, _, _, _, _) in r_columns {
                    columns.push(TableColumn::from_field(&field));
                }
            }
        });
    }
    pub fn get_schemas(&self) -> Schemas {
        self.caches.lock().unwrap().schemas.clone()
    }
}
