use anyhow::{Context, Result};
use clap::Parser;
use crossterm::cursor::MoveTo;
use crossterm::style::Stylize;
use crossterm::terminal::Clear;
use crossterm::{execute, terminal::ClearType};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
    is_higher_order: Option<bool>,
    human_answer: Option<String>,
}

type Questions = Vec<Question>;

enum Mode {
    Classify,
    Answer,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    // Either "classify" or "answer"
    mode: String,

    // PATH to the .json file
    json_path: std::path::PathBuf,
}

fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("Failed to clear screen");
}

fn save_json(json_path: &std::path::PathBuf, questions: &Questions) -> Result<()> {
    let new_data = serde_json::to_string_pretty(&questions)
        .with_context(|| "Failed to serialize JSON".to_string())?;
    fs::write(json_path, new_data).with_context(|| "Failed to write JSON to file.".to_string())?;
    println!("Data saved.");
    Ok(())
}

fn get_answer_from_alpha_option(option: &str, question: &mut Question) -> Option<String> {
    let index = match option {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        _ => 100000,
    };

    if index < question.options.len() {
        Some(question.options[index].clone())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse cli arguements and load mode and .json
    let args = Cli::parse();

    let mode = match args.mode.as_str() {
        "classify" => Ok(Mode::Classify),
        "answer" => Ok(Mode::Answer),
        _ => Err("Mode must be either 'classify' or 'answer'"),
    }?;
    let data = fs::read_to_string(&args.json_path)
        .with_context(|| format!("could not read file: {}", &args.json_path.display()))?;
    let mut questions: Questions =
        serde_json::from_str(&data).with_context(|| "JSON not parsable".to_string())?;

    // QOL vars
    let len_questions: usize = questions.len();
    let mut current_index = 0;
    let mut num_answered = questions
        .iter()
        .filter(|question| question.human_answer.is_some())
        .count();
    let mut num_classified = questions
        .iter()
        .filter(|question| question.is_higher_order.is_some())
        .count();

    let p_bar = ProgressBar::new(questions.len() as u64);
    p_bar.set_style(ProgressStyle::with_template("{wide_bar}")?);
    p_bar.inc(num_answered as u64);

    loop {
        clear_screen(); // Clear the terminal screen before showing a new question
        match mode {
            Mode::Classify => p_bar.set_position(num_classified as u64),
            Mode::Answer => p_bar.set_position(num_answered as u64),
        }
        let question = &mut questions[current_index];
        println!(
            "{}",
            format!("Question {} of {}\n", current_index + 1, len_questions).cyan()
        );

        println!("\n{}", question.question);
        let letter_array = ["a", "b", "c", "d", "e", "f", "g"];
        for (i, option) in question.options.iter().enumerate() {
            println!("{}. {}", letter_array[i], option);
        }

        match mode {
            Mode::Classify => {
                println!("Answer: {}", question.answer);
                if let Some(is_higher_order) = question.is_higher_order {
                    println!(
                        "\nIs Higher Order question: {}",
                        format!("{}", is_higher_order).blue()
                    );
                } else {
                    println!(
                        "{}",
                        "\nIs Higher Order question: NOT YET CLASSIFIED"
                            .to_string()
                            .red()
                    );
                }
                println!("{}", "\n------------------------------\n\nIs this a higher order question? (y/n), (f)orward, (b)ackward, (s)ave, (q)uit\n\n------------------------------\n\n".to_string().cyan());
                println!("Higher order question: involves application, analyzing, evaluating.\nLower order question: involves basic understanding and rote memorization.");
            }
            Mode::Answer => {
                if let Some(human_answer) = &question.human_answer {
                    println!(
                        "\nCurrent selected answer: {}",
                        human_answer.to_string().blue()
                    );
                } else {
                    println!("{}", "\nNO ANSWER SELECTED YET.".to_string().red());
                };

                println!("{}", "\n---------------------------------\nEnter your answer (a, b, c, d, etc.).\nTo navigate: (q)uit, (s)ave, (j) - previous question, (k) - next question.\n".to_string().cyan());
            }
        }

        // get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // input logic
        match mode {
            Mode::Classify => match input {
                "y" => {
                    if current_index < len_questions - 1 {
                        current_index += 1
                    };
                    if question.is_higher_order.is_none() {
                        num_classified += 1;
                    };
                    question.is_higher_order = Some(true);
                }
                "n" => {
                    if current_index < len_questions - 1 {
                        current_index += 1
                    };
                    if question.is_higher_order.is_none() {
                        num_classified += 1
                    };
                    question.is_higher_order = Some(false)
                }

                "f" => {
                    if current_index < len_questions - 1 {
                        current_index += 1
                    }
                }
                "b" => {
                    if current_index > 0 {
                        current_index -= 1;
                    }
                }
                _ => {}
            },
            Mode::Answer => match input {
                "a" | "b" | "c" | "d" | "e" | "f" => {
                    if let Some(human_answer) = get_answer_from_alpha_option(input, question) {
                        if question.human_answer.is_none() {
                            num_answered += 1
                        }

                        question.human_answer = Some(human_answer);
                        if current_index < len_questions - 1 {
                            current_index += 1
                        };
                    }
                }
                "k" => {
                    if current_index < len_questions - 1 {
                        current_index += 1
                    }
                }
                "j" => {
                    if current_index > 0 {
                        current_index -= 1;
                    }
                }
                _ => {}
            },
        }

        match input {
            "s" => save_json(&args.json_path, &questions)?,
            "q" => break,
            _ => {}
        }

        clear_screen(); // Clear the terminal screen before showing a new question
    }

    // Save the modified JSON file when exiting
    save_json(&args.json_path, &questions)?;

    Ok(())
}
