use rusqlite::{Connection, Row};
use serenity::prelude::TypeMapKey;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct Database {
    pub connection: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new<T: Into<String>>(path: T) -> Self {
        Self {
            connection: Arc::new(Mutex::new(
                Connection::open(Path::new(&PathBuf::from(path.into()).join("data.db"))).unwrap(),
            )),
        }
    }

    pub fn get_one<S: Into<String>, T, F>(
        &self,
        sql: S,
        params: &[&(dyn rusqlite::ToSql)],
        map_row: F,
    ) -> rusqlite::Result<T>
    where
        F: FnMut(&Row) -> rusqlite::Result<T>,
    {
        let connection = self.connection.lock().expect("Failed to lock connection");
        let mut stmt = connection.prepare(&sql.into())?;
        let row = stmt.query_one(params, map_row)?;

        Ok(row)
    }

    pub fn get_many<S: Into<String>, T, F>(
        &self,
        sql: S,
        params: &[&dyn rusqlite::ToSql],
        map_row: F,
    ) -> rusqlite::Result<Vec<T>>
    where
        F: FnMut(&Row) -> rusqlite::Result<T>,
    {
        let connection = self.connection.lock().expect("Failed to lock connection");
        let mut stmt = connection.prepare(&sql.into())?;
        let rows = stmt.query_map(params, map_row)?;

        let mut ok_rows: Vec<T> = Vec::new();

        for row in rows {
            match row {
                Ok(row) => ok_rows.push(row),
                Err(err) => return Err(err),
            }
        }
        Ok(ok_rows)
    }

    pub fn run<S: Into<String>>(
        &self,
        sql: S,
        params: &[&dyn rusqlite::ToSql],
    ) -> rusqlite::Result<()> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        let mut stmt = connection.prepare(&sql.into())?;
        stmt.execute(params)?;
        Ok(())
    }
}

impl TypeMapKey for Database {
    type Value = Database;
}

#[macro_export]
macro_rules! impl_from_row {
    ($struct_name:ident, $enum_name:ident { $( $field:ident : $typ:ty ),* $(,)? }) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(
                pub $field: $typ,
            )*
        }
        impl $struct_name {
            pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
                Ok(Self {
                    $(
                        $field: row.get::<_, $typ>(stringify!($field).replace("r#", "").as_str())?,
                    )*
                })
            }
        }

         #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $enum_name {
            $(
                #[allow(non_camel_case_types)]
                $field,
            )*
        }

        impl $enum_name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $enum_name::$field => stringify!($field),
                    )*
                }
            }
        }
    };
}
