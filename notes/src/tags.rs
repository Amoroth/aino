use std::fmt::Debug;

use store::{Store, StoreTrait, Value};

pub struct TagInsert {
    pub name: String
}

pub struct Tag {
    pub id: u64,
    pub name: String
}

impl Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tag {{ id: {}, name: '{}' }}", self.id, self.name)
    }
}

impl Clone for Tag {
    fn clone(&self) -> Self {
        Tag {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

pub struct TagRepo {
    pub store: Store,
}

// todo try to make this unmutable
impl TagRepo {
    pub fn insert(&mut self, tag: TagInsert) {
        self.store.exec("INSERT INTO tags (name) VALUES (?1)".to_string(), &[Value::Text(tag.name)]);
    }

    pub fn get_all_by_note_id(&mut self, note_id: u64) -> Vec<Tag> {
        let results: Vec<Tag> = self.store
            .query("SELECT t.id, t.name FROM tags t INNER JOIN note_tags nt ON t.id = nt.tag_id WHERE nt.note_id = ?1".to_string(), &[Value::Integer(note_id as i64)])
            .iter()
            .map(|row| {
                Tag {
                    id: row.get("id").unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
                    name: row.get("name").unwrap_or(&"".to_string()).to_string(),
                }
            })
            .collect();
        results
    }

    pub fn find_by_name(&mut self, name: &str) -> Result<Tag, ()> {
        let result: Vec<Tag> = self.store
            .query("SELECT * FROM tags WHERE name = ?1;".to_string(), &[Value::Text(name.to_string())])
            .iter()
            .map(|row| {
                Tag {
                    id: row.get("id").unwrap_or(&"0".to_string()).parse::<u64>().unwrap(),
                    name: row.get("name").unwrap_or(&"".to_string()).to_string(),
                }
            })
            .take(1)
            .collect();

        if result.len() == 0 {
            return Err(());
        }
        Ok(result[0].clone())
    }

    pub fn add_tags_to_note_id(&mut self, note_id: u64, tags: &Vec<Tag>) {
        for tag in tags.iter() {
            self.store.exec("INSERT INTO note_tags (note_id, tag_id) VALUES (?1, ?2);".to_string(), &[Value::Integer(note_id as i64), Value::Integer(tag.id as i64)]);
        }
    }
}
