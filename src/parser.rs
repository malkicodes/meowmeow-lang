use crate::{SyntaxTree, Token, Value, get_function_argument_count, get_operator_argument_count};

pub fn parse(tokens: &[Token]) -> Result<(Vec<SyntaxTree>, usize), String> {
    let mut output = Vec::new();

    let mut i = 0;

    while i < tokens.len() {
        let (instruction, j) = parse_one(&tokens[i..])?;
        i += j;
        output.push(instruction);
    }

    Ok((output, i))
}

pub fn parse_one(tokens: &[Token]) -> Result<(SyntaxTree, usize), String> {
    let mut i = 1;

    if let Some(token) = tokens.first() {
        Ok((
            match token {
                Token::Number(n) => SyntaxTree::Atom(Value::Number(*n)),
                Token::String(s) => {
                    SyntaxTree::Atom(Value::Array(s.chars().map(|c| c as i64).collect()))
                }
                Token::Variable(var, n) => SyntaxTree::VariableId(var.clone(), *n),

                Token::Function(func) => {
                    let argument_count = get_function_argument_count(func);

                    let mut arguments = Vec::with_capacity(argument_count);

                    for _ in 0..argument_count {
                        let (arg, j) = parse_one(&tokens[i..])?;
                        i += j;
                        arguments.push(arg);
                    }

                    SyntaxTree::Function(func.to_owned(), arguments)
                }
                Token::Operator(op) => {
                    let argument_count = get_operator_argument_count(*op);

                    if argument_count == 1 {
                        let (lhs, j) = parse_one(&tokens[i..])?;
                        i += j;

                        SyntaxTree::UnaryOp(*op, Box::new(lhs))
                    } else if argument_count == 2 {
                        let (lhs, j) = parse_one(&tokens[i..])?;
                        i += j;
                        let (rhs, j) = parse_one(&tokens[i..])?;
                        i += j;

                        SyntaxTree::BinaryOp(*op, Box::new(lhs), Box::new(rhs))
                    } else {
                        return Err(format!(
                            "unimplemented operator argument count {argument_count}"
                        ));
                    }
                }
            },
            i,
        ))
    } else {
        Err("nothing to parse".to_owned())
    }
}
