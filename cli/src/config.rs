use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RusticConfig {
    pub notes_directory: String,
    pub editor: Option<String>,
}

fn default_config() -> RusticConfig {
    println!("Creating a default configuration file.");
    let default_config = RusticConfig {
        notes_directory: ".".to_string(),
        editor: None,
    };

    let toml_string = toml::to_string_pretty(&default_config).unwrap();
    std::fs::write("config.toml", toml_string).unwrap();
    default_config
}

pub fn get_config() -> RusticConfig {
    match std::fs::read_to_string("config.toml") {
        Ok(data) => toml::from_str(&data).unwrap_or_else(|_| default_config()),
        Err(_) => default_config(),
    }
}

// todo #940 try to guess a default editor before returning None
// todo create default config if not present