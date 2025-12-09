use std::fmt::Debug;

use storage::{Store, StoreTrait};

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
        self.store.exec(format!("INSERT INTO tags (name) VALUES ('{}');", &tag.name));
    }

    pub fn get_all_for_note(&mut self, note_id: u64) -> Vec<Tag> {
        let results: Vec<Tag> = self.store
            .query(format!("SELECT t.id, t.name FROM tags t INNER JOIN note_tags nt ON t.id = nt.tag_id WHERE nt.note_id = {};", note_id))
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
            .query(format!("SELECT * FROM tags WHERE name = '{}';", name))
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

    pub fn add_tags_to_note(&mut self, note_id: u64, tags: &Vec<Tag>) {
        for tag in tags.iter() {
            self.store.exec(format!("INSERT INTO note_tags (note_id, tag_id) VALUES ({}, {});", note_id, tag.id));
        }
    }
}
