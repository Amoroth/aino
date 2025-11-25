pub mod backend; // todo not public

// todo get storage struct
// todo add a way to assign a driver (sqlite only for now)
// todo add a way to structure record generically
// todo insert a new record
// todo read a row
// todo modify row
// todo delete row
// todo add dependencies behind features for sqlite and so on

// todo a way to select backend

pub enum Backend {
    #[cfg(feature = "sqlite")]
    Sqlite
}

pub struct Row {
    pub values: Vec<String>
}

pub trait Store {
    fn query(&self, query: String) -> Vec<Row>;
}


#[derive(Debug, Clone)]
pub struct StoreQueryError;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
