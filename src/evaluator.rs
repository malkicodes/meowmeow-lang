use crate::{Environment, SyntaxTree, Value};

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
            _ => return Err("variable variables are unimplemented".to_owned()),
        },
        SyntaxTree::UnaryOp(op, s) => eval_unary_op(*op, *s.clone(), env)?,
        SyntaxTree::BinaryOp(op, lhs, rhs) => eval_binary_op(*op, *lhs.clone(), *rhs.clone(), env)?,
        SyntaxTree::Function(func, args) => eval_function(func, args, env)?,
        SyntaxTree::Label(_) => Value::Null,
    })
}

fn eval_unary_op(op: char, s: SyntaxTree, env: &mut Environment) -> Result<Value, String> {
    let value = eval(&s, env)?;

    Ok(match op {
        '!' => match value {
            Value::Number(n) => {
                if n > 0 {
                    Value::Number(1)
                } else {
                    Value::Number(0)
                }
            }
            _ => return Err(format!("cannot do unary operator {op} with {value:?}")),
        },
        _ => return Err(format!("unknown unary operator: {op}")),
    })
}

fn eval_binary_op(
    op: char,
    lhs: SyntaxTree,
    rhs: SyntaxTree,
    env: &mut Environment,
) -> Result<Value, String> {
    let lhv = eval(&lhs, env)?;
    let rhv = eval(&rhs, env)?;

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
            '+' => b + a,
            '-' => b - a,
            '*' => b * a,
            '/' => b - a,
            '%' => b % a,
            _ => return Err(format!("unimplemented binary operator {op}")),
        }))
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
        "mew" => {
            let variable_name = match args.first().unwrap().clone() {
                SyntaxTree::VariableId(name, iter) => match iter {
                    0 => name,
                    _ => return Err("variable variables are not implemented yet".to_owned()),
                },
                _ => return Err("assigning to something that is not a variable".to_owned()),
            };

            let value = eval(args.last().unwrap(), env)?;

            Ok(env.set(&variable_name, value).into())
        }
        _ => Err(format!("unknown function: {func}")),
    }
}
