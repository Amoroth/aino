use rusqlite::{Connection, Error, ToSql};
use rusqlite::types::Null;

use crate::{Row, StoreTrait, StoreQueryError, Value};

impl From<rusqlite::Error> for StoreQueryError {
    fn from(_e: rusqlite::Error) -> Self {
        StoreQueryError{}
    }
}

#[derive(Debug)]
pub struct SqliteStore {
    connection: Option<Connection>,
}

impl Clone for SqliteStore {
    fn clone(&self) -> Self {
        SqliteStore {
            connection: None, // do not clone the connection
        }
    }
}

// todo try to make this unmutable if new creates connection none
// lazyly (or maybe uses some pooling mechanism?) if could create
// the connection right here making mutable not needed
impl SqliteStore {
    pub fn new() -> Self {
        SqliteStore { connection: None }
    }

    fn open(&mut self) -> Result<&Connection, StoreQueryError> {
        if self.connection.is_some() {
            return Ok(self.connection.as_ref().unwrap());
        }

        let conn = Connection::open("./aino.db")?; // todo make connection string assignable
        self.connection = Some(conn);

        Ok(self.connection.as_ref().unwrap())
    }
}

// todo implement something like QueryResult instead of Vec<Row> that will hold column names and all rows and map them correctly usindg struct methods?
impl StoreTrait for SqliteStore {
    // todo parametries for query and parsing the query to database agnostic way
    fn exec(&mut self, query: String, params: &[Value]) -> Result<(), StoreQueryError> {
        println!("Executing exec: {}", query);
        let conn = self.open()?;

        let params: Vec<&dyn ToSql> = params.iter().map(|p| {
            match p {
                Value::Null => &Null as &dyn ToSql,
                Value::Integer(i) => i as &dyn ToSql,
                Value::Real(f) => f as &dyn ToSql,
                Value::Text(s) => s as &dyn ToSql,
                Value::Blob(b) => b as &dyn ToSql,
            }
        }).collect();

        match conn.execute(&query, params.as_slice()) {
            Ok(_) => Ok(()),
            Err(_e) => {
                // todo logging
                Err(StoreQueryError{})
            }
        }
    }

    fn exec_script(&mut self, query: String) -> Result<(), StoreQueryError> {
        let conn = self.open()?;

        match conn.execute_batch(&query) {
            Ok(_) => Ok(()),
            Err(_e) => {
                // todo logging
                Err(StoreQueryError{})
            }
        }
    }

    fn insert(&mut self, query: String, params: &[Value]) -> Result<u32, StoreQueryError> {
        println!("Executing insert: {}", query);
        // todo check if its actually an insert because it wont work for any other query
        // todo move the check up to the base store?
        self.exec(query, params)?;

        let conn = self.open()?;

        match conn.query_one("SELECT last_insert_rowid()", [], |row| row.get::<usize, u32>(0)) {
            Ok(id) => Ok(id),
            Err(_) => Err(StoreQueryError)
        }
    }

    // todo change this to result?
    fn query(&mut self, query: String, params: &[Value]) -> Vec<Row> {
        // todo add a logging library to handle levels and not log this by default
        println!("Executing query: {}", query);
        let conn = match self.open() {
            Ok(c) => c,
            Err(_e) => {
                // todo logging
                return vec![];
            }
        };

        let mut stmt = match conn.prepare(&query) {
            Ok(s) => s,
            Err(_e) => {
                // todo logging
                return vec![];
            }
        };

        let columns = stmt.columns();
        let column_names: Vec<String> = columns.iter().map(|col| col.name().to_string()).collect();
        let column_types: Vec<String> = columns.iter().map(|col| col.decl_type().unwrap_or("").to_string()).collect();

        let params: Vec<&dyn ToSql> = params.iter().map(|p| {
            match p {
                Value::Null => &Null as &dyn ToSql,
                Value::Integer(i) => i as &dyn ToSql,
                Value::Real(f) => f as &dyn ToSql,
                Value::Text(s) => s as &dyn ToSql,
                Value::Blob(b) => b as &dyn ToSql,
            }
        }).collect();

        let rows_iter = stmt.query(params.as_slice());
        
        if rows_iter.is_err() {
            return vec![];
        }

        let rows_iter = rows_iter.unwrap();

        let row_values = rows_iter.mapped(|row| {
            let mut values: Vec<(String, String)> = vec![];
            let mut index = 0;

            for column_type in column_types.iter() {
                let value: String = match column_type.as_str() {
                    "TEXT" | "BLOB" => match row.get(index) {
                            Ok(value) => value,
                            Err(Error::InvalidColumnType(_, _, _)) => {
                                index += 1;
                                continue
                            },
                            Err(e) => {
                                println!("{}", e);
                                break
                            },
                        },
                    "INTEGER" => match row.get::<usize, u32>(index) {
                            Ok(value) => value.to_string(),
                            Err(Error::InvalidColumnType(_, _, _)) => {
                                index += 1;
                                continue
                            },
                            Err(e) => {
                                println!("{}", e);
                                break
                            },
                        },
                    "REAL" => match row.get::<usize, f64>(index) {
                            Ok(value) => value.to_string(),
                            Err(Error::InvalidColumnType(_, _, _)) => {
                                index += 1;
                                continue
                            },
                            Err(e) => {
                                println!("{}", e);
                                break
                            },
                        },
                    "" => {
                        index += 1;
                        continue
                    },
                    _ => {
                        index += 1;
                        continue
                    },
                };
                values.push((column_names.get(index).unwrap_or(&String::from("")).clone(), value));
                index += 1;
            }

            Ok(values)
        });

        row_values.map(|r| {
            let r = r.unwrap();
            Row { values: r }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_store_without_connection() {
        let store = SqliteStore::new();
        assert!(store.connection.is_none());
    }
}
