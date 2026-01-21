pub mod backend; // todo not public

// todo get storage struct
// todo add a way to structure record generically
// todo read a row
// todo modify row
// todo delete row
// todo add dependencies behind features for sqlite and so on

#[derive(Debug, Clone)]
pub enum Backend {
    #[cfg(feature = "sqlite")]
    Sqlite
}

// todo rename to StoreValue
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

#[derive(Debug)]
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
    fn exec(&mut self, query: String, params: &[Value]) -> Result<(), StoreQueryError>;
    /// Executes multiple statements in a batch. Does not accept prepared queries!
    fn exec_script(&mut self, query: String) -> Result<(), StoreQueryError>;
    /// Executes a query, same as exec but returns last inserted id
    fn insert(&mut self, query: String, params: &[Value]) -> Result<u32, StoreQueryError>;
    fn query(&mut self, query: String, params: &[Value]) -> Vec<Row>;
}

#[derive(Debug)]
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

impl Clone for Store {
    fn clone(&self) -> Self {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                Store {
                    backend: Backend::Sqlite,
                    sqlite_store: self.sqlite_store.as_ref().map(|s| s.clone()),
                }
            }
        }
    }
}

// todo async
impl StoreTrait for Store {
    fn exec(&mut self, query: String, params: &[Value]) -> Result<(), StoreQueryError> {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                if let Some(store) = &mut self.sqlite_store {
                    store.exec(query, params)
                } else {
                    Err(StoreQueryError)
                }
            }
        }
    }

    fn exec_script(&mut self, query: String) -> Result<(), StoreQueryError> {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                if let Some(store) = &mut self.sqlite_store {
                    store.exec_script(query)
                } else {
                    Err(StoreQueryError)
                }
            }
        }
    }

    fn insert(&mut self, query: String, params: &[Value]) -> Result<u32, StoreQueryError> {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                if let Some(store) = &mut self.sqlite_store {
                    store.insert(query, params)
                } else {
                    Err(StoreQueryError)
                }
            }
        }
    }

    fn query(&mut self, query: String, params: &[Value]) -> Vec<Row> {
        match self.backend {
            #[cfg(feature = "sqlite")]
            Backend::Sqlite => {
                if let Some(store) = &mut self.sqlite_store {
                    return store.query(query, params);
                } else {
                    return vec![];
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StoreQueryError;
