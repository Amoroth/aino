use storage::{Store};
use storage::backend::sqlite::SqliteStore;
use notes::NoteRepo;

fn main() {
    let store = SqliteStore {};

    store.exec(String::from("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT NOT NULL);"));
    // store.exec(String::from("INSERT INTO notes (content) VALUES ('This is my first note.');"));
    let repo = NoteRepo { store };
    let note = repo.select_note_by_id(1);
    match note {
        Ok(n) => println!("Note ID: {}, Content: {}", n.id, n.content),
        Err(e) => println!("Error retrieving note: {:?}", e),
    }
}
