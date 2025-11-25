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
    fn query(&self, query: String) -> Vec<Vec<String>> {
        let conn = match self.open() {
            Ok(c) => c,
            Err(e) => panic!("{}", e) // todo handle error
        };

        let mut stmt = match conn.prepare(&query) {
            Ok(s) => s,
            Err(e) => panic!("{}", e) // todo handle error
        };

        let mut rows: Vec<Vec<String>> = vec![];

        let rows_iter = stmt.query_map([], |row| {
            let mut values: Vec<String> = vec![];
            let mut index = 0;

            loop {
                let value = match row.get::<usize, Option<String>>(index) {
                    Ok(value) => value.unwrap().into(),
                    Err(e) => break,
                };
                values.push(value);
            }

            Ok(values)
        }).unwrap();

        for row in rows_iter {
            rows.push(row.unwrap());
        }

        rows
    }
}