use std::error::Error;
use std::fs;

pub enum Command{
    Help {help_command: Option<String>},
    Print {file_path: String, numbered: bool},
    List
}

pub struct Config {
    pub command: Command
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
                    numbered = args[3] == "--numbered";
                }else{
                    numbered = false;
                }
                Command::Print { file_path, numbered }
            }
            "list" => Command::List,
            _ => return Err("Unknown command"),
        };


        Ok(Config { command })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    match config.command {
        Command::Help { help_command} => help()?,
        Command::Print {file_path, numbered} => print(file_path, numbered)?,
        _ => {}
    }

    Ok(())
}

pub fn print(path: String, numbered: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    if numbered {
        let mut counter = 0;
        for line in contents.lines() {
            counter = counter + 1;
            println!("{counter} | {line}");
        }
    }else{
        println!("{contents}");
    }

    Ok(())
}

pub fn help() -> Result<(), Box<dyn Error>> {

    println!("
HELP\t\tProvides help information for Rose commands
PRINT\t\tPrints text from a specified file
"
    );

    Ok(())
}