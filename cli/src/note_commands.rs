use std::{collections::HashMap, io::Write};

use crate::cli_command::{CliCommandBuilder, CliCommand, CliCommandOption};
use crate::{print_utils, config};
use notes::tags::{TagInsert, Tag, TagRepo};
use notes::notes::{NoteInsert, Note, NoteRepo};
use storage::Store;

pub fn build_new_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("new")
        .add_alias("add")
        .set_description("Create a new note")
        .add_argument("note")
        .add_option(
            &CliCommandOption {
                name: "interactive".to_string(),
                short_name: Some("i".to_string()),
                description: Some("Create note interactivly through an external editor. One has to be provided through config or it will fail.".to_string()),
                is_flag: false
            }
        )
        .add_option(
            &CliCommandOption {
                name: "tag".to_string(),
                short_name: Some("t".to_string()),
                description: Some("Add a tag to the note".to_string()),
                is_flag: false
            }
        ).set_action(|args: HashMap<String, Vec<String>>| {
            let note_content = if args.contains_key("interactive") || !args.contains_key("note") {
                match get_from_editor(None) {
                    Ok(content) => content,
                    Err(EditorOutputError) => {
                        if args.contains_key("note") {
                            args.get("note").and_then(|v| v.last()).unwrap_or(&String::new()).to_string()
                        } else {
                            return;
                        }
                    }
                }
            } else if let Some(note) = args.get("note") {
                note.last().unwrap_or(&String::from("")).to_string()
            } else {
                // todo #941 make it easier to write
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note name is required."));
                return;
            };

            // todo #942 ask if user wants to create empty note anyway
            if note_content.trim().is_empty() {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note content is empty, not creating note."));
                return;
            }

            let store = Store::new(storage::Backend::Sqlite);
            let mut tag_repo = TagRepo { store };

            println!("Creating new note: {note_content}");
            let tags: Vec<Tag> = args.get("tag").unwrap_or(&vec![]).clone().iter().map(|t| {
                tag_repo.find_by_name(t).unwrap_or_else(|_| {
                    tag_repo.insert(TagInsert { name: t.to_string() });
                    tag_repo.find_by_name(t).unwrap()
                })
            }).collect();

            if !tags.is_empty() {
                println!("With tags: {tags:?}");
            }

            // todo pass in store as a layer to cli builder or something
            let store = Store::new(storage::Backend::Sqlite);
            let mut repo = NoteRepo { store };
            let note_id = repo.insert(NoteInsert { content: note_content.trim().to_string() });
            tag_repo.add_tags_to_note_id(note_id.unwrap() as u64, &tags); // handle error
        }).build()
}

pub fn build_list_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("list")
        .add_alias("ls")
        .set_description("List all notes")
        .add_option(
            &CliCommandOption {
                name: "tag".to_string(),
                short_name: Some("t".to_string()),
                description: Some("Search by a tag".to_string()),
                is_flag: false
            }
        ).set_action(|args: HashMap<String, Vec<String>>| {
            let store = Store::new(storage::Backend::Sqlite);
            let mut repo = NoteRepo { store };
            let mut notes = repo.get_all();
            if notes.is_empty() {
                println!("{}", print_utils::colorize(print_utils::Color::warning(), "No notes found."));
            } else {
                // let tags = args.get("tag").unwrap_or(&vec![]).clone();

                // if !tags.is_empty() {
                //     notes.retain(|note| note.tags.iter().any(|tag| tags.contains(tag)));
                // }

                println!("Notes:");
                for note in notes {
                    let note_content = if note.content.len() > 50 {
                        format!("{}...", &note.content[..47])
                    } else {
                        note.content.clone()
                    };
                    println!("{}. {}", note.id, note_content);
                }
            }
        }).build()
}

pub fn build_get_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("get")
        .set_description("Get a single note by its id")
        .add_argument("id")
        .set_action(|args: HashMap<String, Vec<String>>| {
            if let Some(id_str) = args.get("id").and_then(|v| v.last()) {
                if let Ok(id) = id_str.parse::<u32>() {
                    let store = Store::new(storage::Backend::Sqlite);
                    let mut repo = NoteRepo { store };

                    // todo store should operate on i64 instead
                    // todo Option instead of Result?
                    if let Ok(note) = repo.get_by_id(id.into()) {
                        println!("{}", note.content);
                    } else {
                        eprintln!("{}", print_utils::colorize(print_utils::Color::warning(), format!("Note with id {id} not found.").as_str()));
                    }
                } else {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Invalid id: {id_str}").as_str()));
                }
            } else {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note id is required."));
            }
        }).build()
}

pub fn build_search_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("search")
        .set_description("Search for a note by a query string")
        .add_argument("query")
        .add_option(
            &CliCommandOption {
                name: "tag".to_string(),
                short_name: Some("t".to_string()),
                description: Some("Narrow search to a tag".to_string()),
                is_flag: false
            }
        )
        .set_action(|args: HashMap<String, Vec<String>>| {
            let query = args.get("query").and_then(|v| v.last());
            let tags = args.get("tag");

            if query.is_none() && tags.is_none() {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Query is required."));
            }

            let store = Store::new(storage::Backend::Sqlite);
            let mut repo = NoteRepo { store };

            let mut all_notes = repo.get_all();

            // filter by tags
            // if let Some(tags_list) = tags {
            //     all_notes.retain(|n| n.tags.iter().any(|t| tags_list.contains(t)));
            // }

            // filter by query
            if let Some(query_string) = query {
                all_notes = slow_search(&all_notes, query_string)
            }

            if all_notes.is_empty() {
                println!("{}", print_utils::colorize(print_utils::Color::warning(), "No notes found."));
            } else {
                println!("Notes:");
                for note in all_notes {
                    println!("{}. {}", note.id, note.content);
                }
            }
        }).build()
}

// todo move this somewhere where it makes sense, like to notes crate
// realistically, i should use something like ripgrep here, read up on Boyer–Moore string search algo and maybe implement it?
pub fn slow_search(notes: &[Note], query: &str) -> Vec<Note> {
    notes.iter()
        .filter(|&note| note.content.contains(query))
        .cloned()
        .collect()
}

pub fn build_delete_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("delete")
        .add_alias("remove")
        .add_alias("rm")
        .set_description("Delete a single note by its id")
        .add_argument("id")
        .set_action(|args: HashMap<String, Vec<String>>| {
            if let Some(id_str) = args.get("id").and_then(|v| v.last()) {
                if let Ok(id) = id_str.parse::<u32>() {
                    let store = Store::new(storage::Backend::Sqlite);
                    let mut repo = NoteRepo { store };

                    if repo.get_by_id(id.into()).is_ok() {
                        repo.delete_by_id(id);
                    } else {
                        eprintln!("{}", print_utils::colorize(print_utils::Color::warning(), format!("Note with id {id} not found.").as_str()));
                    }
                } else {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Invalid id: {id_str}").as_str()));
                }
            } else {
                eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note id is required."));
            }
        }).build()
}

pub fn build_edit_command() -> CliCommand {
    CliCommandBuilder::default()
        .set_name("edit")
        .set_description("Edit a single note by its id")
        .add_argument("id")
        .add_option(
            &CliCommandOption {
                name: "message".to_string(),
                short_name: Some("m".to_string()),
                description: Some("Replace note by this string. If --interactive option is passed, it is discarded.".to_string()),
                is_flag: false
            }
        ).add_option(
            &CliCommandOption {
                name: "interactive".to_string(),
                short_name: Some("i".to_string()),
                description: Some("Edit note interactivly through an external editor. One has to be provided through config or it will fail.".to_string()),
                is_flag: false
            }
        ).set_action(|args: HashMap<String, Vec<String>>| {
            let id_str = args.get("id").and_then(|v| v.last());
            let id = match id_str {
                Some(id) => match id.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Invalid id: {id}").as_str()));
                        return;
                    }
                },
                None => {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Note id is required."));
                    return;
                }
            };

            let store = Store::new(storage::Backend::Sqlite);
            let mut repo = NoteRepo { store };

            let mut note = match repo.get_by_id(id.into()) {
                Ok(note) => note,
                Err(_) => {
                    eprintln!("{}", print_utils::colorize(print_utils::Color::warning(), format!("Note with id {id} not found.").as_str()));
                    return;
                }
            };

            let edited_note_content = if args.contains_key("interactive") || !args.contains_key("message") {
                match get_from_editor(Some(note.content)) {
                    Ok(content) => content,
                    Err(EditorOutputError) => {
                        if args.contains_key("message") {
                            args.get("message").and_then(|v| v.last()).unwrap_or(&String::new()).to_string()
                        } else {
                            return;
                        }
                    }
                }
            } else {
                args.get("message").and_then(|v| v.last()).unwrap_or(&String::new()).to_string()
            };

            note.content = edited_note_content.trim().to_string();
            repo.update(&note);
        }).build()
}

struct EditorOutputError;

fn get_from_editor(put_content: Option<String>) -> Result<String, EditorOutputError> {
    let config = config::get_config();
    let editor = match config.editor {
        Some(e) => e,
        None => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), "No editor available!"));
            return Err(EditorOutputError);
        }
    };
    
    // save note to temporary file
    let temp_file_path = "/tmp/rustic_note_tmp.txt".to_string();
    let mut file = match std::fs::File::create(&temp_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error creating temporary file: {e}").as_str()));
            return Err(EditorOutputError);
        }
    };

    if put_content.is_some() {
        if let Err(e) = file.write_all(put_content.unwrap_or_default().to_string().trim().as_bytes()) {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error writing note to temporary file: {e}").as_str()));
            return Err(EditorOutputError);
        }
    }

    std::process::Command::new(editor)
        .arg(&temp_file_path)
        .spawn()
        .unwrap_or_else(|_| {panic!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Failed to run editor"))})
        .wait()
        .unwrap_or_else(|_| {panic!("{}", print_utils::colorize(print_utils::Color::error(), "Error: Editor returned a non-zero status"))});
    
    // read the edited note back
    match std::fs::read_to_string(&temp_file_path) {
        Ok(content) => Ok(content),
        Err(e) => {
            eprintln!("{}", print_utils::colorize(print_utils::Color::error(), format!("Error reading edited note: {e}").as_str()));
            Err(EditorOutputError)
        }
    }
}
