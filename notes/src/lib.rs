use storage;

pub struct Note {}

pub struct NoteRepo {
    pub store: storage::Store
};

impl NoteRepo {
    pub fn select_note_by_id(&self, id: u64) -> Note {
        let result = self.store.query();
        Note {}
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
