mod cli_command;
mod note_commands;
mod print_utils;
mod config;

use std::env;

use cli_command::{CliCommandBuilder, CliCommand};
use storage::{Store, StoreTrait};

const ROOT_VERSION: &str = "0.1.0";

fn main() {
    let mut store = Store::new(storage::Backend::Sqlite);

    // add this to some kind of one time initialization routine
    store.exec(String::from("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT NOT NULL);"));
    store.exec(String::from("CREATE TABLE IF NOT EXISTS tags (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);"));
    store.exec(String::from("CREATE TABLE IF NOT EXISTS note_tags (id INTEGER PRIMARY KEY AUTOINCREMENT, note_id INTEGER NOT NULL, tag_id INTEGER NOT NULL, FOREIGN KEY(note_id) REFERENCES notes(id), FOREIGN KEY(tag_id) REFERENCES tags(id));"));

    // todo #945 add variadic positional argument
    // todo #946 add option to builder, to let help not be action taken if no command is not specified and instead print error
    let cli: CliCommand = CliCommandBuilder::default()
        .set_name("Aino")
        .set_version(ROOT_VERSION)
        .set_description("A simplistic tool for managing notes")
        .add_subcommand(&note_commands::build_new_command())
        .add_subcommand(&note_commands::build_list_command())
        .add_subcommand(&note_commands::build_get_command())
        .add_subcommand(&note_commands::build_search_command())
        .add_subcommand(&note_commands::build_delete_command())
        .add_subcommand(&note_commands::build_edit_command())
        .build();
    cli.run(env::args());
}

// todo #947 better error handling
// todo #948 add tests
// todo #949 save notes in markdown/org-mode files with json as a manifest/metadata
// todo #950 edit note tags and others
// todo #951 projects support and persistant switching between them
// todo #952 active tui
// todo #954 expose api as a library for external usage
