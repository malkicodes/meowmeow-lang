use std::io::stdin;

use crate::{
    Environment, SyntaxTree, Value,
    scanner::{VALID_MEOW_REGEX, unescape_string},
};

pub fn eval(s: &SyntaxTree, env: &mut Environment) -> Result<Value, String> {
    Ok(match s {
        SyntaxTree::Atom(v) => v.clone(),
        SyntaxTree::VariableId(var, iter_count) => match *iter_count {
            0 => {
                return env
                    .get(var)
                    .cloned()
                    .ok_or_else(|| format!("undefined variable: {var}"));
            }
            _ => {
                let mut var = var.clone();

                for _ in 0..*iter_count {
                    let val = env
                        .get(&var)
                        .cloned()
                        .ok_or_else(|| format!("undefined variable: {var}"))?;

                    if let Value::Array(_) = &val {
                        var = val
                            .to_array_string()
                            .ok_or_else(|| format!("cannot use {val:?} as a variable"))?;

                        if !VALID_MEOW_REGEX.is_match(&var) {
                            return Err(format!("cannot use {var:?} as a variable"));
                        }
                    } else {
                        return Err(format!("cannot use {val:?} as a variable"));
                    }
                }

                env.get(&var)
                    .cloned()
                    .ok_or_else(|| format!("undefined variable: {var}"))?
            }
        },
        SyntaxTree::UnaryOp(op, s) => eval_unary_op(*op, s, env)?,
        SyntaxTree::BinaryOp(op, lhs, rhs) => eval_binary_op(*op, lhs, rhs, env)?,
        SyntaxTree::Function(func, args) => eval_function(func, args, env)?,
        SyntaxTree::Label(_) => Value::Null,
    })
}

fn eval_unary_op(op: char, s: &SyntaxTree, env: &mut Environment) -> Result<Value, String> {
    let value = eval(s, env)?;

    Ok(match op {
        'b' => {
            if match value {
                Value::Array(arr) => !arr.is_empty(),
                Value::Null => false,
                Value::Number(n) => n > 0,
            } {
                Value::Number(1)
            } else {
                Value::Number(0)
            }
        }
        '!' => match value {
            Value::Number(n) => {
                if n > 0 {
                    Value::Number(1)
                } else {
                    Value::Number(0)
                }
            }
            Value::Null => Value::Number(1),
            _ => return Err(format!("cannot do unary operator {op} with {value:?}")),
        },
        'l' => match value {
            Value::Array(arr) => Value::Number(arr.len() as i64),
            _ => return Err(format!("cannot do unary operator {op} with {value:?}")),
        },
        'a' => match &value {
            Value::Null => Value::Array(Vec::new()),
            Value::Number(i) => Value::Array(Vec::from([*i])),
            Value::Array(_) => value,
        },
        _ => return Err(format!("unknown unary operator: {op}")),
    })
}

fn eval_binary_op(
    op: char,
    lhs: &SyntaxTree,
    rhs: &SyntaxTree,
    env: &mut Environment,
) -> Result<Value, String> {
    let lhv = eval(lhs, env)?;
    let rhv = eval(rhs, env)?;

    if matches!(lhv, Value::Number(_)) && matches!(rhv, Value::Number(_)) {
        let a = match lhv {
            Value::Number(v) => v,
            _ => unreachable!(),
        };
        let b = match rhv {
            Value::Number(v) => v,
            _ => unreachable!(),
        };

        Ok(Value::Number(match op {
            '=' => {
                if a == b {
                    1
                } else {
                    0
                }
            }
            '&' => b & a,
            '|' => b | a,
            '^' => b ^ a,

            '+' => b + a,
            '-' => b - a,
            '*' => b * a,
            '/' => b / a,
            '%' => b % a,
            _ => {
                return Err(format!(
                    "cannot do binary operator {op} with {lhv:?} {rhv:?}"
                ));
            }
        }))
    } else if matches!(lhv, Value::Array(_)) && matches!(rhv, Value::Array(_)) {
        match op {
            '+' => {
                if let Value::Array(mut x) = lhv {
                    if let Value::Array(mut y) = rhv {
                        x.append(&mut y);
                        Ok(Value::Array(x))
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            _ => Err(format!(
                "cannot do binary operator {op} with {lhv:?} {rhv:?}"
            )),
        }
    } else if matches!(lhv, Value::Number(_)) && matches!(rhv, Value::Array(_)) {
        let n = match lhv {
            Value::Number(n) => n,
            _ => unreachable!(),
        };
        let arr = match rhv {
            Value::Array(arr) => arr,
            _ => unreachable!(),
        };

        match op {
            'i' => {
                let arr_len = arr.len() as i64;

                if n < -arr_len || n >= arr_len {
                    Err(format!("index {n} out of bounds [0, {})", arr.len()))
                } else if n >= 0 {
                    Ok(Value::Number(arr[n as usize]))
                } else {
                    Ok(Value::Number(arr[(arr_len + n) as usize]))
                }
            }
            _ => Err(format!(
                "cannot do binary operator {op} with {n} {:?}",
                Value::Array(arr)
            )),
        }
    } else {
        Err(format!(
            "cannot do binary operator {op} with {lhv:?} {rhv:?}"
        ))
    }
}

fn eval_function(func: &str, args: &[SyntaxTree], env: &mut Environment) -> Result<Value, String> {
    if let Some(SyntaxTree::Label(l)) = args.last() {
        return match func {
            "nya" => match env.jump_label(l) {
                Some(_) => Ok(Value::Null),
                None => Err(format!("unknown label {l}")),
            },
            "nyan" => {
                let condition = args.first().unwrap();
                match eval(condition, env)? {
                    Value::Number(i) => {
                        if i > 0 {
                            match env.jump_label(l) {
                                Some(_) => Ok(Value::Null),
                                None => Err(format!("unknown label {l}")),
                            }
                        } else {
                            Ok(Value::Null)
                        }
                    }
                    _ => Err("invalid nyan condition".to_owned()),
                }
            }
            _ => Err(format!("unknown function: {func}")),
        };
    }

    match func {
        "meow" => {
            let arg = eval(args.first().unwrap(), env)?;

            match &arg {
                Value::Null => println!("nyull"),
                Value::Number(i) => match char::from_u32(
                    TryInto::<u32>::try_into(*i)
                        .map_err(|_| format!("could not convert to char: {i}"))?,
                ) {
                    Some(c) => println!("{c}"),
                    None => return Err(format!("could not convert to char: {i}")),
                },
                Value::Array(arr) => {
                    let mut possible_output = String::with_capacity(arr.len());

                    for n in arr.iter().copied() {
                        match TryInto::<u32>::try_into(n) {
                            Ok(i) => match char::from_u32(i) {
                                Some(c) => possible_output.push(c),
                                None => {
                                    return Err(format!(
                                        "could not convert arr to chars: {i} {arr:?}"
                                    ));
                                }
                            },
                            Err(_) => {
                                return Err(format!("could not convert arr to chars: {n} {arr:?}"));
                            }
                        }
                    }

                    println!("{possible_output}")
                }
            }

            Ok(arg)
        }
        "meowr" => {
            // same thing as "meow" but without newline
            let arg = eval(args.first().unwrap(), env)?;

            match &arg {
                Value::Null => print!("nyull"),
                Value::Number(i) => match char::from_u32(
                    TryInto::<u32>::try_into(*i)
                        .map_err(|_| format!("could not convert to char: {i}"))?,
                ) {
                    Some(c) => print!("{c}"),
                    None => return Err(format!("could not convert to char: {i}")),
                },
                Value::Array(arr) => {
                    let mut possible_output = String::with_capacity(arr.len());

                    for n in arr.iter().copied() {
                        match TryInto::<u32>::try_into(n) {
                            Ok(i) => match char::from_u32(i) {
                                Some(c) => possible_output.push(c),
                                None => {
                                    return Err(format!(
                                        "could not convert arr to chars: {i} {arr:?}"
                                    ));
                                }
                            },
                            Err(_) => {
                                return Err(format!("could not convert arr to chars: {n} {arr:?}"));
                            }
                        }
                    }

                    print!("{possible_output}")
                }
            }

            Ok(arg)
        }
        "mreow" => {
            let arg = eval(args.first().unwrap(), env)?;

            match &arg {
                Value::Null => println!("nyull"),
                Value::Number(v) => println!("{v}"),
                Value::Array(arr) => println!("{arr:?}"),
            }

            Ok(arg)
        }
        "mreowr" => {
            let arg = eval(args.first().unwrap(), env)?;

            match &arg {
                Value::Null => print!("nyull"),
                Value::Number(v) => print!("{v}"),
                Value::Array(arr) => print!("{arr:?}"),
            }

            Ok(arg)
        }

        "mew" => {
            let variable_name = get_variable_name(args.first().unwrap(), env)?;

            let value = eval(args.last().unwrap(), env)?;

            Ok(env.set(&variable_name, value).into())
        }
        "miaw" => {
            let variable_name = get_variable_name(args.first().unwrap(), env)?;

            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .map_err(|_| "error while input".to_owned())?;

            let data = input
                .chars()
                .next()
                .ok_or_else(|| "error while input".to_owned())?;

            Ok(env.set(&variable_name, Value::Number(data as i64)).into())
        }
        "miawr" => {
            let variable_name = get_variable_name(args.first().unwrap(), env)?;

            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .map_err(|_| "error while input".to_owned())?;

            let data = unescape_string(input.trim_end_matches('\n'))
                .ok_or_else(|| "invalid input".to_owned())?
                .iter()
                .map(|x| *x as i64)
                .collect::<Vec<_>>();

            Ok(env.set(&variable_name, Value::Array(data)).into())
        }
        "mriaw" => {
            let variable_name = get_variable_name(args.first().unwrap(), env)?;

            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .map_err(|_| "error while input".to_owned())?;

            let data = input
                .trim_end()
                .parse()
                .map_err(|_| "error while input".to_owned())?;

            Ok(env.set(&variable_name, Value::Number(data)).into())
        }
        "mriawr" => {
            let variable_name = get_variable_name(args.first().unwrap(), env)?;

            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .map_err(|_| "error while input".to_owned())?;

            let mut data = Vec::with_capacity(input.len() / 2 + 1);

            for n in input
                .split_ascii_whitespace()
                .map(|x| x.parse().map_err(|_| "invalid input".to_owned()))
            {
                data.push(n?);
            }

            Ok(env.set(&variable_name, Value::Array(data)).into())
        }

        "miao" => {
            let arr = args.last().unwrap();
            let n = if let Value::Number(n) = eval(args.first().unwrap(), env)? {
                n
            } else {
                return Err("trying to append non-number to array".to_owned());
            };

            match arr {
                SyntaxTree::VariableId(..) => {
                    let variable_name = get_variable_name(arr, env)?;

                    if let Some(Value::Array(arr)) = env.get_mut(&variable_name) {
                        arr.push(n);
                        Ok(Value::Number(n))
                    } else {
                        Err("trying to append to non-array or undefined variable".to_owned())
                    }
                }
                _ => Err(
                    "cannot append to non-variable array; use pur [array] puurrr [value] instead"
                        .to_owned(),
                ),
            }
        }
        "miaor" => {
            let arr = args.first().unwrap();

            match arr {
                SyntaxTree::VariableId(..) => {
                    let variable_name = get_variable_name(args.first().unwrap(), env)?;

                    if let Some(Value::Array(arr)) = env.get_mut(&variable_name) {
                        Ok(match arr.pop() {
                            Some(n) => Value::Number(n),
                            None => Value::Null,
                        })
                    } else {
                        Err("trying to pop from non-array or undefined variable".to_owned())
                    }
                }
                s => eval_binary_op('i', &SyntaxTree::Atom(Value::Number(-1)), s, env),
            }
        }
        _ => Err(format!("unknown function: {func}")),
    }
}

fn get_variable_name(variable: &SyntaxTree, env: &mut Environment) -> Result<String, String> {
    Ok(match variable {
        SyntaxTree::VariableId(name, iter_count) => match iter_count {
            0 => name.clone(),
            _ => {
                let mut var = name.clone();

                for _ in 0..*iter_count {
                    let val = env
                        .get(&var)
                        .ok_or_else(|| format!("undefined variable: {var}"))?;

                    if let Value::Array(_) = &val {
                        var = val
                            .to_array_string()
                            .ok_or_else(|| format!("cannot use {val:?} as a variable"))?;

                        if !VALID_MEOW_REGEX.is_match(&var) {
                            return Err(format!("cannot use {var:?} as a variable"));
                        }
                    } else {
                        return Err(format!("cannot use {val:?} as a variable"));
                    }
                }

                var
            }
        },
        _ => return Err(format!("cannot use {variable:?} as a variable")),
    })
}
