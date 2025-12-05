use storage::{Store, StoreTrait, StoreQueryError};

pub struct NoteInsert {
    pub content: String
}

pub struct Note {
    pub id: u64,
    pub content: String
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Note {
            id: self.id,
            content: self.content.clone(),
        }
    }
}

pub struct NoteRepo {
    pub store: Store,
}

// todo try to make this unmutable
impl NoteRepo {
    pub fn select_note_by_id(&mut self, id: u64) -> Result<Note, StoreQueryError> {
        let result: Vec<Note> = self.store
            .query("SELECT * FROM notes WHERE id = ".to_string() + &id.to_string())
            .iter()
            .map(|row| {
                Note {
                    id: row.get("id").unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
                    content: row.get("content").unwrap_or(&"".to_string()).to_string(),
                }
            })
            .take(1)
            .collect();

        if result.len() == 0 {
            return Err(StoreQueryError);
        }
        Ok(result[0].clone())
    }

    pub fn insert_note(&mut self, note: NoteInsert) {
        // todo exec should return Result
        // todo sanitize input to avoid sql injection via parameterized queries
        self.store.exec(format!("INSERT INTO notes (content) VALUES ('{}');", &note.content));
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
