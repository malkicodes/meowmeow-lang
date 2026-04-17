use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::evaluator::eval;

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
    Label(String),
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
    Label(String),
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
        "nyan" => 2,
        _ => 1,
    }
}

#[derive(Debug, Default, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    labels: HashMap<String, usize>,
    instruction_counter: usize,
}

impl Environment {
    pub fn run(&mut self, code: &[SyntaxTree]) -> Result<(), String> {
        for (i, s) in code.iter().enumerate() {
            if let SyntaxTree::Label(l) = s {
                self.set_label(l, i)
                    .ok_or_else(|| format!("cannot override label {l}"))?;
            }
        }

        while let Some(s) = code.get(self.instruction_counter) {
            eval(s, self)?;
            self.instruction_counter += 1;
        }

        Ok(())
    }

    pub fn get(&self, variable_name: &str) -> Option<&Value> {
        self.variables.get(variable_name)
    }

    pub fn set(&mut self, variable_name: &str, value: Value) -> Option<Value> {
        self.variables
            .insert(variable_name.to_owned(), value.clone())
    }

    pub fn del(&mut self, variable_name: &str) -> Option<Value> {
        self.variables.remove(variable_name)
    }

    pub fn set_label(&mut self, label_name: &str, i: usize) -> Option<usize> {
        match self.labels.insert(label_name.to_owned(), i) {
            Some(_) => None,
            None => Some(i),
        }
    }

    pub fn jump_label(&mut self, label_name: &str) -> Option<()> {
        match self.labels.get(label_name).copied() {
            Some(target) => {
                self.instruction_counter = target - 1;
                Some(())
            }
            None => None,
        }
    }
}
