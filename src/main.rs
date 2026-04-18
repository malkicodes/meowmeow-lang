use std::fs;

use clap::Parser;
use meowmeow_lang::{Config, Environment, parser::parse, scanner};

fn main() {
    let config = Config::try_parse().unwrap();

    let text = fs::read_to_string(config.file()).unwrap();
    let tokens = scanner::scan(&text).unwrap();

    if config.debug {
        for x in tokens.iter() {
            eprintln!("{x:?}")
        }

        eprintln!("\n\n")
    }

    let (syntax_trees, _) = parse(&tokens).unwrap();

    if config.debug {
        for (i, syntax_tree) in syntax_trees.iter().enumerate() {
            eprintln!("{i:>3} | {syntax_tree:?}")
        }

        eprintln!("\n\n--------------- OUTPUT STARTS HERE ---------------")
    }

    // Code Evaluation
    let env = Environment::default().run(&syntax_trees);

    if config.debug {
        println!("{env:?}")
    } else if let Err(err) = env {
        println!("An error occured: {err}")
    }
}
