use rusqlite::{Connection, Error};

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
    fn map_query(&self, row_values: Vec<Vec<String>>, column_names: Vec<&str>) -> Vec<Row> {
        let mut rows: Vec<Row> = vec![];

        for values in row_values {
            let mut row = Row { values: vec![] };

            for (index, value) in values.iter().enumerate() {
                let column_name = column_names.get(index).unwrap_or(&"").to_string();
                row.values.push((column_name, value.to_string()));
            }

            rows.push(row);
        }

        rows
    }
}

// todo implement something like QueryResult instead of Vec<Row> that will hold column names and all rows and map them correctly usindg struct methods?
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

        let mut row_values: Vec<Vec<String>> = vec![];

        let rows_iter = stmt.query([]);

        let _ = rows_iter.unwrap().map(|row| {
            let mut values: Vec<String> = vec![];
            let mut index = 0;

            loop {
                let value = match row.get::<usize, Option<String>>(index) {
                    Ok(value) => value.unwrap_or(String::from("")),
                    Err(_) => break,
                };
                values.push(value);
                index += 1;
            }

            row_values.push(values);

            Ok(())
        });

        let column_names = stmt.column_names().to_vec();
        
        self.map_query(row_values, column_names)
        
        // let rows_iter = stmt.query_map([], |row| {
        //     let mut values: Row = Row { values: vec![] };
        //     let mut index = 0;

        //     loop {
        //         let value = match row.get::<usize, Option<String>>(index) {
        //             Ok(value) => value.unwrap_or(String::from("")),
        //             Err(_) => break,
        //         };
        //         let column_name: String = stmt.column_name(index).unwrap_or("").to_string();
        //         values.values.push((column_name, value));
        //         index += 1;
        //     }

        //     Ok(values)
        // }).unwrap();

        // for (index, row) in rows_iter.enumerate() {
        //     rows.push(row.unwrap());
        // }

        // rows
    }
}