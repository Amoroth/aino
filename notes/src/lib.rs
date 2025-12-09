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
    pub fn get_all(&mut self) -> Vec<Note> {
        let results: Vec<Note> = self.store
            .query("SELECT * FROM notes".to_string())
            .iter()
            .map(|row| {
                Note {
                    id: row.get("id").unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
                    content: row.get("content").unwrap_or(&"".to_string()).to_string(),
                }
            })
            .collect();
        results
    }

    pub fn select_by_id(&mut self, id: u64) -> Result<Note, StoreQueryError> {
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

    pub fn insert(&mut self, note: NoteInsert) {
        // todo exec should return Result
        // todo sanitize input to avoid sql injection via parameterized queries
        self.store.exec(format!("INSERT INTO notes (content) VALUES ('{}');", &note.content));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_inserts_new_note() {
        // todo when you can pass connection string use in momory database for testing
        assert_eq!(4, 4);
    }
}
