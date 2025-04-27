use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use strum::Display;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Wether to include furigana above Kanji
    #[arg(short, long)]
    pub furigana: bool,

    /// Sub Commands DOCTESTDOCTEST
    #[command(subcommand)]
    pub command: Commands

}


#[derive(Subcommand)]
pub enum Commands {
    /// Quiz Subcommand.
    /// Creates a quiz using chosen Kanji, Vocab & Grammar
    Quiz {
        /// Runs an interactive quiz allowing the user to choose / write answers. 
        /// If not set, will just display the quiz
        #[arg(short, long)]
        interactive: bool,

        /// If set, will skip generating a prompt & will use the given prompt result to display the quiz
        #[arg(short, long)]
        prompt_result: Option<String>,
        
        /// If set, will display the answers
        #[arg(short, long)]
        answers: bool,

        /// JLPT Level Kanji, Vocabulary & Grammar to use
        #[arg(short, long)]
        level: Level,
    }
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize, Hash, Display)]
pub enum Level {
    /// JLPT N5
    N5,
    /// JLPT N4
    N4,
    /// JLPT N3
    N3,
    /// JLPT N2
    N2,
    /// JLPT N1
    N1
}


impl Cli {

    /// Initialization function for the Cli object.
    /// Useful for decoupling any other modules using this
    /// object from the `clap` crate
    pub fn init() -> Self {
        Cli::parse()
    }

}