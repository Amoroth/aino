use std::{fs::read_to_string, path::Path, fmt::Debug};

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

impl Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Note {{ id: {}, content: '{}' }}", self.id, self.content)
    }
}

pub struct NoteRepo;

#[derive(Debug, Clone)]
pub struct NoteIOError;

// todo try to make this immutable
impl NoteRepo {
    pub fn get_all(&mut self, dir: &str) -> Vec<Note> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            return vec![];
        }

        notes_directory.read_dir().unwrap().filter_map(|dir_entry| {
            let entry = dir_entry.ok()?;
            // todo check extension / valid note somehow
            if !entry.path().is_file() {
                return None;
            }

            Some(Note {
                id: entry.file_name().into_string().ok()?.parse().unwrap_or(0),
                content: read_to_string(entry.path()).unwrap_or_default(),
            })
        }).collect()
    }

    pub fn get_by_id(&mut self, dir: &str, id: u64) -> Result<Note, NoteIOError> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            return Err(NoteIOError);
        }

        notes_directory.read_dir().unwrap().find_map(|dir_entry| {
            let entry = dir_entry.ok()?;
            if !entry.path().is_file() {
                return None;
            }

            if entry.file_name().into_string().ok()?.parse().unwrap_or(0) == id {
                return Some(Note {
                    id: entry.file_name().into_string().ok()?.parse().unwrap_or(0),
                    content: read_to_string(entry.path()).unwrap_or_default(),
                })
            }

            None
        }).ok_or(NoteIOError)
    }

    pub fn insert(&mut self, dir: &str, note: NoteInsert) -> Result<u32, NoteIOError> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            std::fs::create_dir_all(notes_directory).map_err(|_| NoteIOError)?;
        }

        let new_id = std::fs::read_dir(notes_directory)
            .map_err(|_| NoteIOError)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if !entry.path().is_file() {
                    return None;
                }
                entry.file_name().into_string().ok()?.parse::<u64>().ok()
            })
            .max()
            .unwrap_or(0) + 1;

        let new_file_path = notes_directory.join(new_id.to_string());
        std::fs::write(new_file_path, note.content).map_err(|_| NoteIOError)?;

        Ok(new_id as u32)
    }

    pub fn delete_by_id(&mut self, dir: &str, id: u32) -> Result<(), NoteIOError> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            return Err(NoteIOError);
        }

        let file_path = notes_directory.join(id.to_string());
        std::fs::remove_file(file_path).map_err(|_| NoteIOError)?;
        Ok(())
    }

    pub fn update(&mut self, dir: &str, note: &Note) -> Result<(), NoteIOError> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            return Err(NoteIOError);
        }

        let file_path = notes_directory.join(note.id.to_string());
        std::fs::write(file_path, note.content.clone()).map_err(|_| NoteIOError)?;
        Ok(())
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
