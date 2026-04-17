use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[arg()]
    file: PathBuf,
}

impl Config {
    pub fn file(&self) -> &Path {
        self.file.as_path()
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Token {
    Number(i64),
    String(String),
    Variable(String, u8),
    Function(String),
    Operator(char),
}

pub mod scanner;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Array(Vec<i64>),
}

#[derive(Debug)]
pub enum SyntaxTree {
    Atom(Value),
    VariableId(String, u8),
    UnaryOp(char, Box<SyntaxTree>),
    BinaryOp(char, Box<SyntaxTree>, Box<SyntaxTree>),
    Function(String, Vec<SyntaxTree>),
}

pub fn get_operator_argument_count(op: char) -> usize {
    match op {
        '!' => 1,
        _ => 2,
    }
}

pub fn get_function_argument_count(function_name: &str) -> usize {
    match function_name {
        "eq" => 2,
        _ => 1,
    }
}
