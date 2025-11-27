use storage::{Store, StoreQueryError};
use storage::backend::sqlite::SqliteStore;

pub struct Note {
    pub id: u64,
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Note {
            id: self.id,
        }
    }
}

pub struct NoteRepo {
    pub store: SqliteStore
}

impl NoteRepo {
    pub fn select_note_by_id(&self, id: u64) -> Result<Note, StoreQueryError> {
        let result: Vec<Note> = self.store
            .query("SELECT * FROM notes WHERE id = ".to_string() + &id.to_string())
            .iter()
            .map(|row| {
                Note {
                    id: row.get("id").unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
                }
            })
            .take(1)
            .collect();
        Ok(result[0].clone())
    }
}

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
