use std::fs;

use clap::Parser;
use meowmeow_lang::{Config, Environment, parser::parse, scanner};

fn main() {
    let config = Config::try_parse().unwrap();

    let text = fs::read_to_string(config.file()).unwrap();
    let tokens = scanner::scan(&text).unwrap();

    for x in tokens.iter() {
        println!("{x:?}")
    }

    let (syntax_trees, _) = parse(&tokens).unwrap();
    println!("\nSYNTAX TREES:");

    for syntax_tree in syntax_trees.iter() {
        println!("{syntax_tree:?}")
    }

    // Code Evaluation
    let env = Environment::default().run(&syntax_trees);

    println!("{env:?}")
}
