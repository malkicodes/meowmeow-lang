use std::{fs, process};

use clap::Parser;
use meowmeow_lang::{Config, Environment, parser::parse, scanner};

fn main() {
    let config = Config::try_parse().unwrap();

    let text = fs::read_to_string(config.file()).unwrap();
    let tokens = scanner::scan(&text).unwrap();

    if config.debug {
        eprint!("{} Token", tokens.len());

        if tokens.len() != 1 {
            eprint!("s");
        }

        eprintln!(":");

        for x in tokens.iter() {
            eprintln!("{x:?}")
        }

        eprintln!()
    }

    let (syntax_trees, _) = parse(&tokens).unwrap();

    if config.debug {
        let line_length = (syntax_trees.len() as f32).log10().floor() as usize + 1;

        eprint!("{} Instruction", syntax_trees.len());

        if syntax_trees.len() != 1 {
            eprint!("s");
        }

        eprintln!(":");

        for (i, syntax_tree) in syntax_trees.iter().enumerate() {
            eprintln!("{i:>line_length$} | {syntax_tree:?}")
        }
    }

    if config.no_run {
        process::exit(0)
    } else if config.debug {
        eprintln!("\n\n--------------- OUTPUT STARTS HERE ---------------")
    }

    // Code Evaluation
    let env = Environment::default().run(&syntax_trees);

    if config.debug {
        eprintln!("{env:?}")
    } else if let Err(err) = env {
        eprintln!("An error occured: {err}");
        process::exit(1);
    }
}
