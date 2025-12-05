use storage::{Store, StoreTrait};
use notes::{NoteInsert, NoteRepo};

fn main() {
    let mut store = Store::new(storage::Backend::Sqlite);

    store.exec(String::from("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT NOT NULL);"));
    let mut repo = NoteRepo { store };
    repo.insert_note(NoteInsert { content: "This is my eight note".to_string() });
    let note = repo.select_note_by_id(7);
    match note {
        Ok(n) => println!("Note ID: {}, Content: {}", n.id, n.content),
        Err(e) => println!("Error retrieving note: {:?}", e),
    }
}
