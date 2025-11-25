use rusqlite::{Connection, Error};

use crate::{Row, Store, Value};

pub struct SqliteStore {
}

impl SqliteStore {
    fn open(&self) -> Result<Connection, Error> {
        // todo save it in struct?
        let conn = Connection::open("./aino.db")?;
        Ok(conn)
    }
}

impl Store for SqliteStore {
    fn query(&self, query: String) -> Vec<Row> {
        let conn = match self.open() {
            Ok(c) => c,
            Err(e) => panic!("{}", e) // todo handle error
        };

        let mut stmt = match conn.prepare(&query) {
            Ok(s) => s,
            Err(e) => panic!("{}", e) // todo handle error
        };

        let mut rows: Vec<Row> = vec![];

        let rows_iter = stmt.query_map([], |row| {
            let mut values: Row = Row { values: vec![] };
            let mut index = 0;

            loop {
                let value = match row.get::<usize, Option<String>>(index) {
                    Ok(value) => Value::from(value.unwrap_or(String::from(""))),
                    Err(_) => break,
                };
                values.values.push(value);
                index += 1;
            }

            Ok(values)
        }).unwrap();

        for row in rows_iter {
            rows.push(row.unwrap());
        }

        rows
    }
}