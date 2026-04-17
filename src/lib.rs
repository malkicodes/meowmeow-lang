use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

use clap::Parser;

pub mod evaluator;
pub mod parser;
pub mod scanner;

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

#[derive(Clone, PartialEq)]
pub enum Value {
    Null,
    Number(i64),
    Array(Vec<i64>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => f.write_str("Null"),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Array(arr) => {
                f.write_str("Array")?;

                let mut possible_output = String::with_capacity(arr.len());

                for n in arr.iter().copied() {
                    match TryInto::<u32>::try_into(n) {
                        Ok(i) => match char::from_u32(i) {
                            Some(c) => possible_output.push(c),
                            None => return write!(f, "{arr:?}"),
                        },
                        Err(_) => return write!(f, "{arr:?}"),
                    }
                }

                write!(f, "[{:?}]", possible_output.as_str())
            }
        }
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        match value {
            Some(v) => v,
            None => Value::Null,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
        "mew" => 2,
        _ => 1,
    }
}

#[derive(Debug, Default, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn get(&self, variable_name: &String) -> Option<&Value> {
        self.variables.get(variable_name)
    }

    pub fn set(&mut self, variable_name: &String, value: Value) -> Option<Value> {
        self.variables.insert(variable_name.clone(), value.clone())
    }

    pub fn del(&mut self, variable_name: &String) -> Option<Value> {
        self.variables.remove(variable_name)
    }
}
