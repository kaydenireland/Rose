use std::env;
use std::process;

use rose::Config;
use rose::grammar::{Derivation, DerivationStep, Grammar, Rule, Sentential};

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

    example_manual();
}

fn example_manual() {
    let rules = vec![Rule::new('E', "E+e"), Rule::new('E', "x")];
    let grammar = Grammar::from_rules(rules);

    println!("Grammar Valid: {:}", grammar.is_valid());
    println!("Grammar Regular: {:}", grammar.is_regular());

    let mut derivation = Derivation::new(&grammar);
    // TODO: Handle errors
    derivation.derive_leftmost(&grammar, 0);
    derivation.derive_leftmost(&grammar, 0);
    derivation.derive_leftmost(&grammar, 1);

    println!("Derivation Complete: {:}", derivation.is_complete());
    println!("Derivation Word: {:}", derivation.word());
}
