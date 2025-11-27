use std::any::{Any, TypeId};

use rusqlite::{Connection, Error, ToSql};

use crate::{Row, Store};

pub struct SqliteStore {
}

impl SqliteStore {
    fn open(&self) -> Result<Connection, Error> {
        // todo save it in struct?
        let conn = Connection::open("./aino.db")?;
        Ok(conn)
    }

    // todo move to base?
    fn map_query(&self, row_values: Vec<Vec<String>>, column_names: Vec<String>) -> Vec<Row> {
        let mut rows: Vec<Row> = vec![];

        for values in row_values {
            let mut row = Row { values: vec![] };

            for (index, value) in values.iter().enumerate() {
                let column_name = column_names.get(index).unwrap_or(&"".to_string()).to_string();
                row.values.push((column_name, value.to_string()));
            }

            rows.push(row);
        }

        rows
    }
}

// todo implement something like QueryResult instead of Vec<Row> that will hold column names and all rows and map them correctly usindg struct methods?
impl Store for SqliteStore {
    // todo parametries for query and parsing the query to database agnostic way
    fn exec(&self, query: String) {
        let conn = match self.open() {
            Ok(c) => c,
            Err(e) => panic!("{}", e) // todo handle error
        };

        match conn.execute(&query, []) {
            Ok(_) => (),
            Err(e) => panic!("{}", e) // todo handle error
        }
    }

    fn query(&self, query: String) -> Vec<Row> {
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
            let mut values: Vec<String> = vec![];
            let mut index = 0;

            for column_name in column_types.iter() {
                println!("Getting column index: {}", index);

                let value: String = match column_name.as_str() {
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
                values.push(value);
                index += 1;
            }

            Ok(values)
        });

        let row_values: Vec<Vec<String>> = row_values.map(|r| r.unwrap()).collect();
        println!("Query returned {} rows", row_values.len());
        self.map_query(row_values, column_names)
    }
}