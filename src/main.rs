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

    // grammar_test();
}

fn grammar_test() {
    let r1 = rose::grammar::Rule::new('S', "aA");
    let r2 = rose::grammar::Rule::new('A', "AAa");
    let r3 = Rule::new('A', "b");

    print!("Rule 2: {} | Valid: {}\n", r2.display(), r2.is_valid());

    let rules = vec![r1, r2, r3];
    let grammar = Grammar::from_rules(rules);

    print!("{}", grammar.display());
    print!("Is grammar valid? {}\n", grammar.is_valid());
}
