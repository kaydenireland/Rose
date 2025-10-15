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

    grammar_test();
}

fn grammar_test() {
    let r1 = rose::grammar::Rule::new('Q', "aA");
    let r2 = rose::grammar::Rule::new('A', "Aa");
    let r3 = Rule::new('A', "b");

    print!("Rule 2: {} | Valid: {}\n", r2.display(), r2.is_valid());

    println!("\nRule 1: Right Regular: {}, Left Regular: {}", r1.is_right_regular(), r1.is_left_regular());
    println!("Rule 2: Right Regular: {}, Left Regular: {}", r2.is_right_regular(), r2.is_left_regular());
    println!("Rule 3: Right Regular: {}, Left Regular: {}", r3.is_right_regular(), r3.is_left_regular());

    let rules = vec![r1, r2, r3];
    let grammar = Grammar::from_rules(rules);

    print!("{}", grammar.display());
    print!("Is grammar valid? {}\n", grammar.is_valid());
    print!("Is grammar regular? {}\n", grammar.is_regular());

    print!("Rules with A on LHS: {:?}\n", grammar.rule_idxs_from_nt('A'));
    print!("Rules with B on LHS: {:?}\n", grammar.rule_idxs_from_nt('B'));
    print!("Rules with Q on LHS: {:?}\n", grammar.rule_idxs_from_nt('Q'));
}
