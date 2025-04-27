use std::collections::HashMap;

use config::Config;
use serde::{Serialize, Deserialize};

use crate::cli::Level;

const CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub openai_token: String,
    pub quiz_prompt: String,
    pub jlpt_levels: HashMap<Level, JlptLevel>,
}


#[derive(Deserialize, Serialize)]
pub struct JlptLevel {
    #[serde(alias = "Kanji")]
    pub kanji: Vec<String>,

    #[serde(alias = "Grammar")]
    pub grammar: Vec<String>
}


/// Parses the configuration file into the `Configuration` struct
pub fn parse_configuration() -> Configuration {
    Config::builder()
                .add_source(config::File::with_name(CONFIG_FILE_NAME))
                .build()
                .unwrap()
                .try_deserialize::<Configuration>()
                .unwrap()
}
