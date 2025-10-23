use colored::*;
use std::error::Error;
use std::fs;

use crate::grammar::{Derivation, Grammar, Rule};

pub mod grammar;
pub mod lexer;

pub enum Command {
    Help { help_command: Option<String> },
    Print { file_path: String, numbered: bool },
    List { list_command: Option<String> },
    Derive { derive_command: String },
}

pub struct Config {
    pub command: Command,
    pub grammar: Grammar,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let rules = vec![
            Rule::new('E', "E+e"),
            Rule::new('E', "Ee"),
            Rule::new('E', "eeE"),
            Rule::new('E', "Gp"),
            Rule::new('G', "s"),
            Rule::new('E', "x"),
        ];
        let grammar = Grammar::from_rules(rules);

        let cmd = args[1].as_str();
        let command = match cmd {
            "help" => {
                let sub = args.get(2).cloned();
                Command::Help { help_command: sub }
            }
            "print" => {
                if args.len() < 3 {
                    return Err("Missing file path for print");
                }

                let file_path = args[2].clone();
                let numbered: bool;
                if args.len() > 3 {
                    numbered = args[3].to_lowercase() == "--numbered";
                } else {
                    numbered = false;
                }
                Command::Print {
                    file_path,
                    numbered,
                }
            }
            "list" => {
                let sub = args.get(2).cloned();
                Command::List { list_command: sub }
            }
            "derive" => {
                if args.len() < 3 {
                    return Err("Enter Derive Command");
                }

                let derive_command = args[2].clone();
                Command::Derive {
                    derive_command: derive_command,
                }
            }
            _ => return Err("Unknown command"),
        };

        Ok(Config { command, grammar })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Command::Help { help_command } => help(help_command)?,
        Command::Print {
            file_path,
            numbered,
        } => print(file_path, numbered)?,
        Command::List { list_command } => list(&config.grammar, list_command)?,
        Command::Derive { derive_command } => derive(&config.grammar, derive_command)?,
    }

    Ok(())
}

pub fn print(path: String, numbered: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    if numbered {
        // Calculate width needed for the largest line number so the pipe aligns.
        let total_lines = contents.lines().count();
        let width = total_lines.to_string().len();

        let mut counter = 0;
        for line in contents.lines() {
            counter += 1;
            // Format string so pipe remains in same spot throughout output
            let num_str = format!("{num:>width$}", num = counter, width = width).yellow();
            println!("{} {} {line}", num_str, "|".yellow(), line = line);
        }
    } else {
        println!("{contents}");
    }

    Ok(())
}

pub fn help(help_command: Option<String>) -> Result<(), Box<dyn Error>> {
    if help_command.is_some() {
        let help_command = help_command.unwrap().to_lowercase();
        if help_command == "print" {
            println!(
                "
Prints the contents of a file.

{}

- The file path is required following the print argument.
- Numbered flag is optional, adding it will add line numbers to the output.
            ",
                "print 'file_path' (--numbered)".yellow()
            );
        } else if help_command == "list" {
            println!(
                "
Prints all useable commands.
            "
            );
        } else {
            println!("{}", "Command not found".red());
        }
    } else {
        println!(
            "
{}\t\tProvides help information for Rose commands
{}\t\tPrints text from a specified file
{}\t\tPrints All Commands
",
            "HELP".yellow(),
            "PRINT".yellow(),
            "LIST".yellow()
        );
    }

    Ok(())
}

pub fn list(grammar: &Grammar, list_command: Option<String>) -> Result<(), Box<dyn Error>> {
    if list_command.is_some() {
        let list_command = list_command.unwrap().to_lowercase();
        if list_command == "rules" {
            for (_, rule) in grammar.rules.iter().enumerate() {
                println!("{}", rule.display())
            }
        }
    } else {
        println!(
            "
    {}
    {}
    {}
    ",
            "HELP".yellow(),
            "PRINT".yellow(),
            "LIST".yellow()
        );
    }

    Ok(())
}

pub fn derive(grammar: &Grammar, derive_command: String) -> Result<(), Box<dyn Error>> {
    if derive_command.to_lowercase() == "random" {
        let mut derivation = Derivation::new(&grammar);
        println!(
            "Random Derived Word: {}",
            derivation
                .print_random(&grammar, Some(20))
                .unwrap_or("No Word Generated".to_string())
                .yellow()
        );
    }

    Ok(())
}
