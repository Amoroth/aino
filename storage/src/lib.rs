pub mod backend; // todo not public

// todo get storage struct
// todo add a way to structure record generically
// todo read a row
// todo modify row
// todo delete row
// todo add dependencies behind features for sqlite and so on

pub enum Backend {
    #[cfg(feature = "sqlite")]
    Sqlite
}

pub struct Row {
    pub values: Vec<(String, String)>
}

impl Row {
    // todo generic for autmatic pasring?
    pub fn get(&self, column_name: &str) -> Option<&String> {
        for (name, value) in &self.values {
            if name == column_name {
                return Some(value);
            }
        }
        None
    }
}

pub trait StoreTrait {
    fn exec(&mut self, query: String) -> ();
    /// Executes a query, same as exec but returns last inserted id
    fn insert(&mut self, query: String) -> Result<i64, StoreQueryError>;
    fn query(&mut self, query: String) -> Vec<Row>;
}

pub struct Store {
    pub backend: Backend,
    #[cfg(feature = "sqlite")]
    sqlite_store: Option<backend::sqlite::SqliteStore>
}

impl Store {
    pub fn new(backend: Backend) -> Self {
        match backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                Store {
                    backend,
                    sqlite_store: Some(backend::sqlite::SqliteStore::new())
                }
            }
        }
    }
}

impl StoreTrait for Store {
    fn exec(&mut self, query: String) {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                if let Some(store) = &mut self.sqlite_store {
                    store.exec(query);
                }
            }
        }
    }

    fn insert(&mut self, query: String) -> Result<i64, StoreQueryError> {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                if let Some(store) = &mut self.sqlite_store {
                    store.insert(query)
                } else {
                    Err(StoreQueryError)
                }
            }
        }
    }

    fn query(&mut self, query: String) -> Vec<Row> {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                if let Some(store) = &mut self.sqlite_store {
                    return store.query(query);
                } else {
                    return vec![];
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StoreQueryError;
