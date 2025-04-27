use colored::Colorize;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{
    cli::{Cli, Commands, Level},
    config::Configuration,
};

const KANJI_HANDLEBAR_STRING: &str = "{{kanji}}";
const GRAMMAR_HANDLEBAR_STRING: &str = "{{grammar}}";
const JLPT_LEVEL_HANDLEBAR_STRING: &str = "{{jlpt_level}}";

const DISPLAY_DIVIDER: &str = "───────────────────────────────";

#[derive(Display, Deserialize, Serialize)]
pub enum QuestionTypes {
    SentenceCompletion,
    ReadingComprehension,
}

/// Struct defining a quiz question
#[derive(Deserialize, Serialize)]
pub struct Question {
    pub question: String,
    pub choices: Vec<String>,
    pub answer: String,
    pub answer_explanation: String,
}

/// Struct defining the JSON response returned by the LLM model.
/// This struct should be an exact match of the response
/// returned by the LLM model, and should be used as reference
/// when writing the prompt that will be sent to said model.
#[derive(Deserialize, Serialize)]
pub struct LLMResponse {
    pub question_type: QuestionTypes,
    pub questions: Vec<Question>,

    #[serde(default)]
    pub text_piece: String,
}

/// The messages used in the chat with an OpenAI compatible LLM
#[derive(Serialize)]
pub struct LLMMessage {
    role: String,
    content: String,
}

/// The request model sent to an OpenAI compatible LLM
#[derive(Serialize)]
pub struct LLMRequest {
    model: String,
    store: bool,
    messages: Vec<LLMMessage>,
}

pub fn _display_response(llm_response: Vec<LLMResponse>, answers: &bool) {
    for response in llm_response {
        println!("{}", DISPLAY_DIVIDER.truecolor(131, 139, 167));

        match response.question_type {
            QuestionTypes::SentenceCompletion => {
                println!("{}", "Sentence Completion".truecolor(186, 187, 241));
            }
            QuestionTypes::ReadingComprehension => {
                println!("{}", "Reading Comprehension".truecolor(186, 187, 241));
                println!("{}", response.text_piece.truecolor(133, 193, 220));
            }
        };

        println!("{}", DISPLAY_DIVIDER.truecolor(131, 139, 167));

        for question in response.questions {
            println!(
                "{} {}",
                "Question:".truecolor(140, 170, 238),
                question.question.truecolor(133, 193, 220)
            );
            println!("{}", question.choices.join(", ").truecolor(238, 190, 190));

            if *answers == true {
                println!();
                println!(
                    "{} {}",
                    "Answer:".truecolor(140, 170, 238),
                    question.answer.truecolor(166, 209, 137)
                );
            }
            println!();
        }
    }
}

/// Creates a prompt.
/// Uses `prompt_template` as a template, and injects
/// `kanji`, `grammar`, and `jlpt_level` into thier respective
/// handlebars in the template
pub fn _create_prompt(
    prompt_template: &str,
    kanji: &Vec<String>,
    grammar: &Vec<String>,
    jlpt_level: &Level,
) -> String {
    prompt_template
        .replace(KANJI_HANDLEBAR_STRING, &kanji.join(","))
        .replace(GRAMMAR_HANDLEBAR_STRING, &grammar.join(","))
        .replace(JLPT_LEVEL_HANDLEBAR_STRING, &jlpt_level.to_string())
}

/// Main quiz logic, which is:
/// 1. Create the prompt
/// 2. Send prompt to OpenAI-like API
/// 3. Parse response
/// 4. Display / Export / Interact with response
pub fn quiz_logic(cli: Cli, configuration: Configuration) {
    match cli.command {
        Commands::Quiz {
            interactive,
            answers,
            level,
            prompt_result,
        } => {
            let jlpt_level: Level = level;

            if interactive {
                todo!("Interactive was set, but it is not supported yet");
            }

            if *&configuration.jlpt_levels.contains_key(&jlpt_level) != true {
                panic!(
                    "The jlpt level given by the command flag did not match the configuration, which should have failed parsing."
                )
            }

            let jlpt_level_configuration = &configuration.jlpt_levels[&jlpt_level];

            match prompt_result {
                Some(prompt) => {
                    let deserialized_results: Vec<LLMResponse> =
                        serde_json::from_str(&prompt).unwrap();
                    _display_response(deserialized_results, &answers);
                }
                None => {
                    let prompt = _create_prompt(
                        &configuration.quiz_prompt,
                        &jlpt_level_configuration.kanji,
                        &jlpt_level_configuration.grammar,
                        &jlpt_level,
                    );

                    println!("Prompt: {:?}", prompt);
                }
            };
        }
    }
}
