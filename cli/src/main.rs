use storage::{Store, StoreTrait};
use storage::backend::sqlite::SqliteStore;
use notes::NoteRepo;

fn main() {
    let mut store = Store::new(storage::Backend::Sqlite);

    store.exec(String::from("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT NOT NULL);"));
    // store.exec(String::from("INSERT INTO notes (content) VALUES ('This is my seventh note.');"));
    let mut repo = NoteRepo { store };
    let note = repo.select_note_by_id(7);
    match note {
        Ok(n) => println!("Note ID: {}, Content: {}", n.id, n.content),
        Err(e) => println!("Error retrieving note: {:?}", e),
    }
}
