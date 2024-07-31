#![warn(unused_extern_crates)]
use chrono::prelude::*;
use clap::Parser;
use color_eyre::{eyre::WrapErr, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    prelude::*,
    style::Stylize,
    text::{Line, Text},
    widgets::{block::Title, Block, Borders, LineGauge, Paragraph},
    Frame,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process;

mod errors;
mod tui;

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
    is_higher_order: Option<bool>,
    human_answer: Option<String>,
}

type Questions = Vec<Question>;

#[derive(Debug, Default, PartialEq)]
enum Mode {
    Classify,
    #[default]
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

#[derive(Debug, Default)]
pub struct App {
    json_path: std::path::PathBuf,
    questions: Questions,
    question_index: usize,
    mode: Mode,
    message: String,
    exit: bool,
    num_answered: usize,
}

impl App {
    fn new(
        json_path: std::path::PathBuf,
        questions: Questions,
        question_index: usize,
        mode: Mode,
        message: String,
        exit: bool,
        num_answered: usize,
    ) -> App {
        App {
            json_path,
            questions,
            question_index,
            mode,
            message,
            exit,
            num_answered,
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
        let current_q = &self.questions[self.question_index];

        let controls = {
            let mut i_vec = vec![
                " Prev".into(),
                "<Left>".blue().bold(),
                " Next".into(),
                "<Right>".blue().bold(),
                " Save".into(),
                "<s>".blue().bold(),
                " Quit ".into(),
                "<q> ".red().bold(),
            ];
            i_vec.splice(0..0, {
                match self.mode {
                    Mode::Classify => vec![
                        " Yes".into(),
                        "<y>".cyan().bold(),
                        " No".into(),
                        "<n>".cyan().bold(),
                    ],
                    Mode::Answer => vec![" Enter answer ".into(), "<1, 2, 3, 4, 5>".cyan().bold()],
                }
            });
            Title::from(Line::from(i_vec))
        };

        let question_index_text = Title::from(Line::from(vec![
            " Question ".into(),
            (self.question_index + 1).to_string().cyan(),
            " of ".into(),
            self.questions.len().to_string().cyan(),
            " ".into(),
        ]));

        let mut q_text: Vec<Line<'_>> = vec![Line::from(current_q.question.clone())];
        q_text.push(Line::from(""));
        q_text.extend(
            current_q
                .options
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    let letter_array = ["1", "2", "3", "4", "5", "6", "7"];
                    Line::from(format!("{}\n", letter_array[i].to_string() + " - " + text).yellow())
                })
                .collect::<Vec<_>>(),
        );
        // is the question answered or has it already been classified?
        let q_status = Line::from(match self.mode {
            Mode::Classify => {
                if let Some(is_higher_order) = current_q.is_higher_order {
                    format!("Current classification: {}", is_higher_order).blue()
                } else {
                    format!("MISSING CLASSIFICATION").red().bold()
                }
            }
            Mode::Answer => {
                if let Some(answer) = &current_q.human_answer {
                    format!("Current answer: {}", answer).blue()
                } else {
                    format!("MISSING ANSWER").red().bold()
                }
            }
        });
        q_text.push(Line::from(""));
        q_text.push(q_status);

        let instructions = Text::from(match self.mode {
            Mode::Classify => vec![
                Line::from("Is this a higher order question?".bold()),
                Line::from(""),
                Line::from("Higher order question: involves application, analyzing, evaluating."),
                Line::from(
                    "Lower order question: involves basic understanding and rote memorization.",
                ),
            ],
            Mode::Answer => vec![
                Line::from("What is the correct answer?".bold()),
                Line::from(""),
                Line::from("Type 1, 2, 3, 4, or 5 to select an answer."),
            ],
        });

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .split(frame.size());
        // for question and instructions
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(outer_layout[1]);

        // add txt to layout
        frame.render_widget(
            Paragraph::default().alignment(Alignment::Center).block(
                Block::new().title(Title::from(self.message.clone()).alignment(Alignment::Right)),
            ),
            outer_layout[0],
        );
        frame.render_widget(
            Paragraph::new(Text::from(q_text))
                .wrap(ratatui::widgets::Wrap { trim: true })
                .block(
                    Block::new()
                        .borders(Borders::TOP | Borders::RIGHT)
                        .title(question_index_text)
                        .title_alignment(Alignment::Left)
                        .padding(ratatui::widgets::Padding::new(1, 1, 1, 1)),
                ),
            inner_layout[0],
        );
        frame.render_widget(
            Paragraph::new(instructions)
                .block(
                    Block::new()
                        .borders(Borders::TOP | Borders::LEFT)
                        .padding(ratatui::widgets::Padding::new(1, 1, 1, 1)),
                )
                .wrap(ratatui::widgets::Wrap { trim: true }),
            inner_layout[1],
        );
        frame.render_widget(
            LineGauge::default()
                .block(
                    Block::default()
                        .title(controls.alignment(Alignment::Center))
                        .borders(Borders::TOP),
                )
                .ratio(self.num_answered as f64 / self.questions.len() as f64)
                .filled_style(
                    Style::default()
                        .fg(Color::LightCyan)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .label(format!(
                    "Question progress: {}%",
                    (self.num_answered as f64 * 100 as f64 / self.questions.len() as f64).round()
                )),
            outer_layout[2],
        );
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit()?,
            KeyCode::Char('s') => self.save()?,
            KeyCode::Left => self
                .decrement_index()
                .wrap_err("overflow substraction error")?,
            KeyCode::Right => self
                .increment_index()
                .wrap_err("overflow addition error somehow")?,
            _ => {}
        }
        if self.mode == Mode::Classify {
            match key_event.code {
                KeyCode::Char('y') => {
                    if self.questions[self.question_index]
                        .is_higher_order
                        .is_none()
                    {
                        self.increment_num_answered()?;
                    }
                    self.questions[self.question_index].is_higher_order = Some(true)
                }
                KeyCode::Char('n') => {
                    if self.questions[self.question_index]
                        .is_higher_order
                        .is_none()
                    {
                        self.increment_num_answered()?;
                    }
                    self.questions[self.question_index].is_higher_order = Some(false)
                }
                _ => {}
            }
        }
        if self.mode == Mode::Answer {
            // TODO
            match key_event.code {
                KeyCode::Char(value) => match value {
                    '1' | '2' | '3' | '4' | '5' | '6' => {
                        if let Some(human_answer) = get_answer_from_alphanum_option(
                            &value.to_string(),
                            &self.questions[self.question_index],
                        ) {
                            if self.questions[self.question_index].human_answer.is_none() {
                                self.increment_num_answered()?;
                            };
                            self.questions[self.question_index].human_answer = Some(human_answer);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.exit = true;
        save_json(&self.json_path, &self.questions).wrap_err("save_json failed")?;
        Ok(())
    }

    fn save(&mut self) -> Result<()> {
        // Get the current UTC time
        let now = Utc::now();
        save_json(&self.json_path, &self.questions).wrap_err("save_json failed")?;
        let message = format!("Progress saved at {}", now);
        self.message = message;
        Ok(())
    }

    fn decrement_index(&mut self) -> Result<()> {
        self.question_index = self.question_index.saturating_sub(1);
        Ok(())
    }

    fn increment_index(&mut self) -> Result<()> {
        if self.question_index < self.questions.len() - 1 {
            self.question_index += 1;
        };
        Ok(())
    }

    fn increment_num_answered(&mut self) -> Result<()> {
        self.num_answered += 1;
        Ok(())
    }
}

fn save_json(json_path: &std::path::PathBuf, questions: &Questions) -> Result<()> {
    let new_data = serde_json::to_string_pretty(&questions)
        .wrap_err("Failed to serialize JSON while saving.")?;
    fs::write(json_path, new_data).wrap_err("Failed to write JSON to file.")?;
    Ok(())
}

fn get_answer_from_alphanum_option(option: &str, question: &Question) -> Option<String> {
    let index = match option {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        "4" => 3,
        "5" => 4,
        "6" => 5,
        _ => 100000,
    };

    if index < question.options.len() {
        Some(question.options[index].clone())
    } else {
        None
    }
}

fn get_num_answered(mode: &Mode, questions: &Questions) -> usize {
    match mode {
        Mode::Classify => questions
            .iter()
            .filter(|question| question.is_higher_order.is_some())
            .count(),
        Mode::Answer => questions
            .iter()
            .filter(|question| question.human_answer.is_some())
            .count(),
    }
}

fn main() -> Result<()> {
    errors::install_hooks()?;
    // parse cli arguements and load mode and .json
    let args = Cli::parse();

    let mode = match args.mode.as_str() {
        "classify" => Mode::Classify,
        "answer" => Mode::Answer,
        _ => {
            eprintln!("Mode must be either 'classify' or 'answer'");
            process::exit(1)
        }
    };
    let data = fs::read_to_string(&args.json_path)
        .with_context(|| format!("could not read file: {}", &args.json_path.display()))?;
    let questions: Questions = serde_json::from_str(&data).wrap_err("JSON not parsable")?;
    let num_answered: usize = get_num_answered(&mode, &questions);

    let mut terminal = tui::init()?;

    let mut app: App = App::new(
        args.json_path,
        questions,
        0,
        mode,
        "".to_string(),
        false,
        num_answered,
    );

    app.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
