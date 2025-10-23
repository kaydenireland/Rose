use std::env;
use std::process;

use rose::Config;
use rose::grammar::{Derivation, Grammar, Rule};
use rose::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rose::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // example_manual();
    test_lexer();
}

#[allow(dead_code)]
fn test_lexer() {
    let mut lexer = Lexer::new("///".to_string());
    lexer.print_tokens();
}

#[allow(dead_code)]
fn example_manual() {
    let rules = vec![
        Rule::new('E', "E+e"),
        Rule::new('E', "Ee"),
        Rule::new('E', "eeE"),
        Rule::new('E', "Gp"),
        Rule::new('G', "s"),
        Rule::new('E', "x"),
    ];
    let grammar = Grammar::from_rules(rules);

    println!("Grammar Valid: {:}", grammar.is_valid());
    println!("Grammar Regular: {:}", grammar.is_regular());

    let mut derivation = Derivation::new(&grammar);
    println!(
        "Random Derived Word (5 Step Limit): {}",
        derivation
            .print_random(&grammar, Some(5))
            .unwrap_or("No Word Generated".to_string())
    );
    derivation = Derivation::new(&grammar);
    println!(
        "Random Derived Word (No Step Limit): {}",
        derivation
            .print_random(&grammar, None)
            .unwrap_or("No Word Generated".to_string())
    );
}
