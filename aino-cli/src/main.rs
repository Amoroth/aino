mod note_commands;
mod config;

use std::env;

use cli::{CliCommandBuilder, CliCommand};

const ROOT_VERSION: &str = "0.1.0";

fn main() {
    // todo #945 add variadic positional argument
    // todo #946 add option to builder, to let help not be action taken if no command is not specified and instead print error
    let cli: CliCommand = CliCommandBuilder::default()
        .set_name("Aino")
        .set_version(ROOT_VERSION)
        .set_description("A simplistic knowledge assistant.")
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
// todo inbox
// todo version control through git, if user wants it, it initializes repo and helps with simple commands like undo or something