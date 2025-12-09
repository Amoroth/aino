use rusqlite::{Connection, Error};

use crate::{Row, StoreTrait, StoreQueryError};

pub struct SqliteStore {
    connection: Option<Connection>,
}

// todo try to make this unmutable if new creates connection none
// lazyly (or maybe uses some pooling mechanism?) if could create
// the connection right here making mutable not needed
impl SqliteStore {
    pub fn new() -> Self {
        SqliteStore { connection: None }
    }

    fn open(&mut self) -> Result<&Connection, Error> {
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
    fn exec(&mut self, query: String) {
        let conn = match self.open() {
            Ok(c) => c,
            Err(e) => panic!("{}", e) // todo handle error
        };

        match conn.execute(&query, []) {
            Ok(_) => (),
            Err(e) => panic!("{}", e) // todo handle error
        }
    }

    fn insert(&mut self, query: String) -> Result<i64, StoreQueryError> {
        // todo check if its actually an insert because it wont work for any other query
        // todo move the check up to the base store?
        self.exec(query);

        let conn = match self.open() {
            Ok(c) => c,
            Err(e) => panic!("{}", e) // todo handle error
        };

        match conn.query_one("SELECT last_insert_rowid()", [], |row| row.get::<usize, i64>(0)) {
            Ok(id) => Ok(id),
            Err(_) => Err(StoreQueryError)
        }
    }

    fn query(&mut self, query: String) -> Vec<Row> {
        // todo add a logging library to handle levels and not log this by default
        println!("Executing query: {}", query);
        let conn = match self.open() {
            Ok(c) => c,
            Err(e) => panic!("{}", e) // todo handle error
        };

        let mut stmt = match conn.prepare(&query) {
            Ok(s) => s,
            Err(e) => panic!("{}", e) // todo handle error
        };

        let columns = stmt.columns();
        let column_names: Vec<String> = columns.iter().map(|col| col.name().to_string()).collect();
        let column_types: Vec<String> = columns.iter().map(|col| col.decl_type().unwrap_or("").to_string()).collect();

        let rows_iter = stmt.query([]).unwrap();

        let row_values = rows_iter.mapped(|row| {
            println!("Row found");
            let mut values: Vec<(String, String)> = vec![];
            let mut index = 0;

            for column_type in column_types.iter() {
                println!("Getting column index: {}", index);

                let value: String = match column_type.as_str() {
                    "TEXT" => match row.get(index) {
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
                    "INTEGER" => match row.get::<usize, i64>(index) {
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
                println!("Value: {}", value);
                values.push((column_names.get(index).unwrap_or(&String::from("")).clone(), value));
                index += 1;
            }

            Ok(values)
        });

        println!("Query returned {} rows", row_values.size_hint().0);
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
