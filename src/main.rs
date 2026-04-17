use std::fs;

use clap::Parser;
use meowmeow_lang::{Config, scanner};

fn main() {
    let config = Config::try_parse().unwrap();

    let text = fs::read_to_string(config.file()).unwrap();
    let tokens = scanner::scan(&text).unwrap();

    for x in tokens.iter() {
        println!("{x:?}")
    }

    println!("{tokens:?}")
}
