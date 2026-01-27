use std::{fs::read_to_string, path::Path, fmt::Debug};

pub struct NoteInsert {
    pub content: String
}

pub struct NoteDetails {
    pub title: Option<String>,
}

impl NoteDetails {
    pub fn new() -> Self {
        NoteDetails {
            title: None
        }
    }

    pub fn set_title(&mut self, new_title: &str) {
        self.title = Some(new_title.to_string());
    }
}

impl Clone for NoteDetails {
    fn clone(&self) -> Self {
        NoteDetails {
            title: self.title.clone(),
        }
    }
}

pub struct Note {
    pub id: u64,
    pub content: String,
    pub details: NoteDetails,
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Note {
            id: self.id,
            content: self.content.clone(),
            details: self.details.clone(),
        }
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Note {{ id: {}, content: '{}' }} | NoteDetails {{ title: '{:?}' }}", self.id, self.content, self.details.title)
    }
}

pub struct NoteRepo;

#[derive(Debug, Clone)]
pub struct NoteIOError;

const SUPPORTED_FORMATS: [&str; 1] = ["md"];

// todo try to make this immutable
impl NoteRepo {
    pub fn get_all(&mut self, dir: &str) -> Vec<Note> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            return vec![];
        }

        notes_directory.read_dir().unwrap().filter_map(|dir_entry| {
            let entry = dir_entry.ok()?;
            // todo this is way too long to check extension, right?
            // todo valid note somehow
            if !entry.path().is_file() || !SUPPORTED_FORMATS.contains(&entry.path().extension().unwrap_or_default().to_str().unwrap_or_default()) {
                return None;
            }

            let note_header = read_note_header(entry.path().to_str().unwrap_or_default());

            Some(Note {
                id: entry.path().file_stem()?.to_str().unwrap_or_default().parse().unwrap_or(0),
                content: String::new(),
                details: note_header.unwrap_or_else(|_| { NoteDetails::new() })
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
            if !entry.path().is_file() || !SUPPORTED_FORMATS.contains(&entry.path().extension().unwrap_or_default().to_str().unwrap_or_default()) {
                return None;
            }

            let entry_name = entry.path().file_stem()?.to_str().unwrap_or_default().parse().unwrap_or(0);

            if entry_name == id {
                return Some(Note {
                    id: entry_name,
                    content: read_note_content(entry.path().to_str().unwrap_or_default()).unwrap_or_default(),
                    details: NoteDetails::new(),
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
                entry.path().file_stem().unwrap_or_default().to_str().unwrap_or_default().parse::<u64>().ok()?.into()
            })
            .max()
            .unwrap_or(0) + 1;

        let mut note_content = note.content.clone();

        if note.content.split_once("\n").unwrap_or_default().0 != "---" {
            let default_header = "---\n---\n";
            note_content = default_header.to_string() + note.content.as_str();
        }

        // todo should/can it be more safe?
        let new_file_path = notes_directory.join(new_id.to_string() + ".md");
        std::fs::write(new_file_path, note_content).map_err(|_| NoteIOError)?;

        Ok(new_id as u32)
    }

    pub fn delete_by_id(&mut self, dir: &str, id: u32) -> Result<(), NoteIOError> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            return Err(NoteIOError);
        }

        let file_path = notes_directory.join(id.to_string() + ".md");
        std::fs::remove_file(file_path).map_err(|_| NoteIOError)?;
        Ok(())
    }

    pub fn update(&mut self, dir: &str, note: &Note) -> Result<(), NoteIOError> {
        let notes_directory = Path::new(dir);
        if !notes_directory.exists() {
            return Err(NoteIOError);
        }

        let file_path = notes_directory.join(note.id.to_string() + ".md");
        std::fs::write(file_path, note.content.clone()).map_err(|_| NoteIOError)?;
        Ok(())
    }
}

fn read_note_header(note_path: &str) -> Result<NoteDetails, NoteIOError> {
    use std::io::BufRead;

    let mut note_header = NoteDetails::new();

    let file = std::fs::File::open(note_path);
    if file.is_ok() {
        let mut header_lines: Vec<String> = vec![];
        let file_buffer = std::io::BufReader::new(file.unwrap());
        let file_lines = file_buffer.lines();
        let mut file_beginning = true;
        let mut header_exists = false;

        for line in file_lines {
            if let Ok(line_content) = line {
                note_header.set_title(line_content.clone().as_str());

                if file_beginning && line_content == "---" {
                    file_beginning = false;
                    header_exists = true;
                } else if header_exists && line_content == "---" {
                    break;
                } else if !file_beginning && header_exists {
                    header_lines.push(line_content);
                } else {
                    break;
                }
            }
        }

        for header_line in header_lines {
            if header_line.starts_with("title: ") {
                note_header.set_title(header_line.clone().replace("title: ", "").as_str());
            }
        }

        return Ok(note_header)
    }

    Err(NoteIOError)
}

fn read_note_content(note_path: &str) -> Result<String, NoteIOError> {
    use std::io::BufRead;

    let mut note_header = NoteDetails::new();

    let file = std::fs::File::open(note_path);
    if file.is_ok() {
        let mut content_lines: Vec<String> = vec![];
        let file_buffer = std::io::BufReader::new(file.unwrap());
        let file_lines = file_buffer.lines();
        let mut file_beginning = true;
        let mut header_exists = false;

        for line in file_lines {
            if let Ok(line_content) = line {
                note_header.set_title(line_content.clone().as_str());

                if file_beginning && line_content == "---" {
                    file_beginning = false;
                    header_exists = true;
                } else if header_exists && line_content == "---" {
                    header_exists = false;
                } else if !file_beginning && !header_exists {
                    content_lines.push(line_content);
                }
            }
        }

        return Ok(content_lines.join("\n"))
    }

    Err(NoteIOError)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_inserts_new_note() {
        // todo when you can pass connection string use in momory database for testing
        assert_eq!(4, 4);
    }
}
