use colored::*;
use std::error::Error;
use std::fs;
use strum::IntoEnumIterator;

use crate::{
    analyzer::{MTree as STree, SymbolTable, fold_constants, from_parse_tree}, grammar::{Derivation, Grammar, Rule}, lexer::Lexer, parser::Parser, token::Token
};

pub mod grammar;
pub mod lexer;
pub mod mtree;
pub mod parser;
pub mod pratt_parser;
pub mod token;
pub mod analyzer;
pub mod types;
pub mod codegen;

pub enum Command {
    Help { help_command: Option<String> },
    Print { file_path: String, numbered: bool },
    List { list_command: Option<String> },
    Derive { derive_command: String },
    Tokenize { file_path: String },
    Parse { file_path: String },
    Analyze { file_path: String },
    Compile { file_path: String }
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
                Command::Derive { derive_command }
            }
            "tokenize" => {
                if args.len() < 3 {
                    return Err("Enter File Path");
                }
                let file_path = args[2].clone();
                Command::Tokenize { file_path }
            }
            "parse" => {
                if args.len() < 3 {
                    return Err("Enter File Path");
                }
                let file_path = args[2].clone();
                Command::Parse { file_path }
            }
            "analyze" => {
                if args.len() < 3 {
                    return Err("Enter File Path");
                }
                let file_path = args[2].clone();
                Command::Analyze { file_path }
            }
            "compile" => {
                if args.len() < 3 {
                    return Err("Enter File Path");
                }
                let file_path = args[2].clone();
                Command::Compile { file_path }
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
        Command::Tokenize { file_path } => tokenize(file_path),
        Command::Parse { file_path } => parse(file_path),
        Command::Analyze { file_path } => analyze_tree(file_path),
        Command::Compile { file_path } => compile(file_path),
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
- Prints all grammar rules with {} keyword.
- Prints all tokens with {} keyword.
- Prints all commands when no keyword is given.
                ",
                "rules".yellow(),
                "tokens".yellow()
            );
        } else if help_command == "derive" {
            println!("Creates a word from the grammar.")
        } else if help_command == "tokenize" {
            println!("Creates tokens from user inputted file.")
        } else if help_command == "parse" {
            println!("Parses tokens from user inputted file into an MTree.")
        } else {
            println!("{}", "Command not found.".red());
        }
    } else {
        println!(
            "
{}\t\tProvides help information for Rose commands
{}\t\tPrints text from a specified file
{}\t\tPrints all commands
{}\t\tCreates word from grammar
{}\tCreates tokens from inputted language
{}\t\tParses tokens from inputted file into an MTree
",
            "HELP".yellow(),
            "PRINT".yellow(),
            "LIST".yellow(),
            "DERIVE".yellow(),
            "TOKENIZE".yellow(),
            "PARSE".yellow()
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
        } else if list_command == "tokens" {
            for token in Token::iter() {
                println!("{:?}", token);
            }
        }
    } else {
        println!(
            "
    {}
    {}
    {}
    {}
    {}
    {}
    ",
            "HELP".yellow(),
            "PRINT".yellow(),
            "LIST".yellow(),
            "DERIVE".yellow(),
            "TOKENIZE".yellow(),
            "PARSE".yellow()
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

pub fn tokenize(path: String) {
    let contents = fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.print_tokens();
}

pub fn parse(path: String) {
    let lexer = Lexer::new(fs::read_to_string(path).unwrap());
    let mut parser = Parser::new(lexer);
    let mtree = parser.analyze();

    println!("\nMTree:");
    mtree.print();
}

pub fn analyze_tree(path: String) {
    let lexer = Lexer::new(fs::read_to_string(path).unwrap());
    let mut parser = Parser::new(lexer);
    let mtree = parser.analyze();
    
    match from_parse_tree(&mtree) {
        Ok(mut ast) => {
            println!("\n=== Semantic AST ===\n{:#?}", ast);

            fold_constants(&mut ast);

            // symbol table
            let mut sym_table = SymbolTable::new();

            // run semantic analysis and report how many errors we found
            match analyzer::analyze(&ast, &mut sym_table) {
                Ok(_) => {
                    println!("\n✓ Semantic analysis completed with 0 error(s).");
                }
                Err(errors) => {
                    println!("\n✓ Semantic analysis completed with {} error(s):", errors.len());
                    for (i, error) in errors.iter().enumerate() {
                        println!("  {}. {}", i + 1, error);
                    }
                    println!("\n✗ Skipping execution due to semantic errors");
                }
            }
        }
        Err(e) => {
            panic!("Semantic conversion failed: {}", e);
        }
    }
}

use inkwell::context::Context;
use codegen::LlvmCodegen;

fn compile_to_ir(semantic: &STree) -> Result<(), String> {
    let context = Context::create();
    let mut cg = LlvmCodegen::new(&context, "rose_module");
    cg.emit_program(semantic)?;
    cg.verify()?;
    cg.write_ir_to_file("out.ll")?;
    Ok(())
}

pub fn compile(path: String) {
    let lexer = Lexer::new(fs::read_to_string(path).unwrap());
    let mut parser = Parser::new(lexer);
    let mtree = parser.analyze();
    
    match from_parse_tree(&mtree) {
        Ok(mut ast) => {
            println!("\n=== Semantic AST ===\n{:#?}", ast);

            fold_constants(&mut ast);

            // symbol table
            let mut sym_table = SymbolTable::new();

            // run semantic analysis and report how many errors we found
            match analyzer::analyze(&ast, &mut sym_table) {
                Ok(_) => {
                    println!("\n✓ Semantic analysis completed with 0 error(s).");
                    match compile_to_ir(&ast) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                Err(errors) => {
                    println!("\n✓ Semantic analysis completed with {} error(s):", errors.len());
                    for (i, error) in errors.iter().enumerate() {
                        println!("  {}. {}", i + 1, error);
                    }
                    println!("\n✗ Skipping execution due to semantic errors");
                }
            }
        }
        Err(e) => {
            panic!("Semantic conversion failed: {}", e);
        }
    }
}