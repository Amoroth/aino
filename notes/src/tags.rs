use std::fmt::Debug;

pub struct Tag {
    pub name: String
}

impl Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tag {{ name: '{}' }}", self.name)
    }
}

impl Clone for Tag {
    fn clone(&self) -> Self {
        Tag {
            name: self.name.clone(),
        }
    }
}

pub struct TagRepo;

// todo try to make this unmutable
impl TagRepo {
    pub fn insert(&mut self, tag: Tag) {
        // todo
    }

    pub fn get_all_by_note_id(&mut self, note_id: u64) -> Vec<Tag> {
        // todo
        vec![]
    }

    pub fn find_by_name(&mut self, name: &str) -> Result<Tag, ()> {
        Err(())
    }

    pub fn add_tags_to_note_id(&mut self, note_id: u64, tags: &Vec<Tag>) {
        // todo
    }
}
