use crossterm::cursor::MoveTo;
use crossterm::terminal::Clear;
use crossterm::{
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::ClearType,
};

use crossterm::style::{PrintStyledContent, Stylize};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
    is_higher_order: Option<bool>,
}

type Questions = Vec<Question>;

fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("Failed to clear screen");
}

fn print_colored(text: &str, color: Color) {
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        PrintStyledContent(text.with(color)),
        ResetColor
    )
    .expect("Failed to print colored text");
}

fn main() {
    // Load and parse the JSON file as a command line arguement
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: question_cli <path_to_questions.json>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let data = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(error) => {
            println!("Error message: {}\nUnable to read the provided file path. Please make sure it is correct and the file is accessible.", error);
            return;
        }
    };

    let mut questions: Questions = match serde_json::from_str(&data) {
        Ok(result) => result,
        Err(e) => {
            println!("JSON invalid. Error message:\n{}", e);
            return;
        }
    };
    let len_questions: usize = questions.len();
    let mut current_index = 0;

    loop {
        clear_screen(); // Clear the terminal screen before showing a new question

        let question = &mut questions[current_index];
        print_colored(
            &format!("Question {} of {}\n", current_index + 1, len_questions,),
            Color::Cyan,
        );
        println!("{}", question.question);
        for (i, option) in question.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
        println!("Answer: {}", question.answer);
        if let Some(is_higher_order) = question.is_higher_order {
            println!("\nIs Higher Order question: {}", is_higher_order);
        } else {
            print_colored(
                "\nIs Higher Order question: NOT YET CLASSIFIED\n",
                Color::Red,
            );
        }

        print_colored(&format!("\n------------------------------\n\nIs this a higher order question? (y/n), (f)orward, (b)ackward, (s)ave, (q)uit\n\n------------------------------\n\n"), Color::DarkCyan);
        println!("Higher order question: involves application, analyzing, evaluating.\nLower order question: involves basic understanding and rote memorization.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        match input {
            "y" => {
                if current_index < len_questions - 1 {
                    current_index += 1
                };
                question.is_higher_order = Some(true)
            }
            "n" => {
                if current_index < len_questions - 1 {
                    current_index += 1
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
            "save" => {
                let new_data =
                    serde_json::to_string_pretty(&questions).expect("Failed to serialize data");
                fs::write(file_path, new_data).expect("Unable to write file");
                println!("Data saved.");
            }
            "q" => break,
            _ => println!("Invalid input, please try again."),
        }
    }

    // Save the modified JSON file when exiting
    let new_data = serde_json::to_string_pretty(&questions).expect("Failed to serialize data");
    fs::write(file_path, new_data).expect("Unable to write file");
    println!("Data saved.");
}
