use colored::*;
use std::error::Error;
use std::fs;

pub mod grammar;

pub enum Command {
    Help { help_command: Option<String> },
    Print { file_path: String, numbered: bool },
    List,
}

pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

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
            "list" => Command::List,
            _ => return Err("Unknown command"),
        };

        Ok(Config { command })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Command::Help { help_command } => help(help_command)?,
        Command::Print {
            file_path,
            numbered,
        } => print(file_path, numbered)?,
        Command::List => list()?,
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
            // Right-align the counter within `width` characters, colorize it, then add a small spacer and the colored pipe.
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

pub fn list() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}
