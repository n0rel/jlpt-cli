mod cli;
mod quiz;
mod config;


use quiz::quiz_logic;

use crate::cli::Cli;
use crate::config::parse_configuration;

fn main() {

    let cli = Cli::init();
    let configuration = parse_configuration();

    quiz_logic(cli, configuration);
}
