mod backend;

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

pub enum Value {
    Integer(i64),
    Null
}

pub struct Row {
    pub values: Vec<Value>
}

pub enum QueryType {
    Select
}

pub enum ConditionOperator {
    Eq
}

pub struct Condition {
    pub column: String,
    pub operator: ConditionOperator,
    pub value: Vec<String>
}

pub struct Query {
    pub query_type: QueryType,
    pub select: Vec<String>,
    pub from: String,
    pub conditions: Vec<Condition>
}

pub struct Store {}

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
