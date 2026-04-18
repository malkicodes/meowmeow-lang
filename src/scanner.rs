use std::sync::LazyLock;

use regex::Regex;

use crate::Token;

pub static VALID_MEOW_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\Amr{0,3}[iye]?[aoe]*[wu]*r?~*\z").unwrap());
pub static VALID_NYA_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\Any{0,3}a+n?~?\z").unwrap());

fn scan_mml_number(s: &str) -> Option<i64> {
    let mut chars = s.chars();
    let mut output = 0;

    if chars.next() != Some('m') {
        return None;
    }

    while let Some(c) = chars.next() {
        match c {
            'r' => output += 1,
            'p' => {
                if chars.next().is_none() {
                    output = -output
                } else {
                    return None; // mrrprrp should be invalid
                }
            } // mrrp = -2
            _ => return None, // invalid number
        }
    }

    Some(output)
}

fn scan_mml_operator(s: &str) -> Option<(u8, u8)> {
    let mut u: u8 = 0;
    let mut r: u8 = 1;

    if !(s.starts_with('p') && s.ends_with('r')) {
        return None;
    }

    let mut chars = s.chars();
    chars.next();

    for c in chars.by_ref() {
        match c {
            'u' => u = u.wrapping_add(1),
            'r' => break,
            _ => return None,
        }
    }

    for c in chars {
        match c {
            'r' => r = r.wrapping_add(1),
            _ => return None,
        }
    }

    Some((u, r))
}

fn get_operator_from_ur(ur: (u8, u8)) -> Option<char> {
    Some(match ur {
        (0, 1) => 'b', // pr        = convert to 0 or 1
        (0, 2) => '!', // prr       = not
        (0, 3) => '=', // prrr      = is equivalent to (==)
        (0, 4) => '&', // prrrr     = and
        (0, 5) => '|', // prrrrr    = or
        (0, 6) => '^', // prrrrrr   = xor

        (1, 1) => '+', // pur       = addition
        (1, 2) => '-', // purr      = subtraction
        (1, 3) => '*', // purrr     = multiplication
        (1, 4) => '/', // purrrr    = division
        (1, 5) => '%', // purrrrr   = modulo

        (2, 1) => 'i', // puur      = index
        (2, 2) => 'l', // puurr     = length
        (2, 3) => 'a', // puurrr    = convert to array

        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SplitState {
    Awaiting,
    Identifier(usize),
    String(usize),
}

impl SplitState {
    fn unwrap(self) -> usize {
        match self {
            SplitState::Awaiting => panic!("unwrapped awaiting"),
            SplitState::Identifier(v) => v,
            SplitState::String(v) => v,
        }
    }
}

fn split(text: &str) -> Result<Vec<&str>, (String, usize)> {
    let mut output = Vec::new();

    let mut state: SplitState = SplitState::Awaiting;
    let mut escape: bool = false;
    let mut comment: bool = false;

    for (i, c) in text.char_indices() {
        println!("{state:?}: {i} {c:?} | {escape} {comment}");
        if comment {
            if c == '\n' {
                comment = false;
            }
            continue;
        }

        if c == '#' && !escape && !matches!(state, SplitState::String(_)) {
            comment = true;

            match state {
                SplitState::Identifier(start_i) => output.push(&text[start_i..]),
                SplitState::String(_) => return Err(("unclosed string".to_owned(), i)),
                SplitState::Awaiting => (),
            }
            continue;
        }

        if c.is_ascii_whitespace() {
            if let SplitState::Identifier(_) = state {
                output.push(&text[state.unwrap()..i]);
                state = SplitState::Awaiting;
            }

            escape = false;
        } else if c == '"' {
            match state {
                SplitState::Identifier(_) => {
                    return Err(("identifier too close to string".to_owned(), i));
                }
                SplitState::String(start_i) => {
                    if !escape {
                        output.push(&text[start_i..i + 1]);
                        state = SplitState::Awaiting;
                    }
                }
                SplitState::Awaiting => state = SplitState::String(i),
            }

            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if let SplitState::Awaiting = state {
            state = SplitState::Identifier(i);
            escape = false;
        } else {
            escape = false;
        }
    }

    match state {
        SplitState::Identifier(start_i) => output.push(&text[start_i..]),
        SplitState::String(start_i) => return Err(("unclosed string".to_owned(), start_i)),
        SplitState::Awaiting => (),
    }

    Ok(output)
}

pub fn scan(text: &str) -> Result<Vec<Token>, String> {
    let split_words = split(text).map_err(|(s, i)| format!("splitting error at {i}: {s}"))?;

    let mut output = Vec::with_capacity(split_words.len());

    for word in split_words {
        if word == "nyull" {
            output.push(Token::Null);
            continue;
        }

        if let Some(string) = word.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            output.push(Token::String(
                unescape_string(string)
                    .ok_or_else(|| "invalid array".to_owned())
                    .unwrap(),
            ));
            continue;
        }

        if let Some(n) = scan_mml_number(word) {
            output.push(Token::Number(n));
            continue;
        }

        if let Some(ur) = scan_mml_operator(word) {
            if let Some(op) = get_operator_from_ur(ur) {
                output.push(Token::Operator(op));
                continue;
            } else {
                return Err(format!("invalid opurrator: {word} {ur:?}"));
            }
        }

        if VALID_MEOW_REGEX.is_match(word) {
            if word.ends_with('~') {
                let (word, recursion_string) = match word.split_once('~') {
                    Some(v) => v,
                    None => unreachable!(),
                };
                output.push(Token::Variable(
                    word.to_owned(),
                    recursion_string.len().try_into().map_err(|_| {
                        format!(
                            "variable variable level too deep: {} > 255",
                            recursion_string.len()
                        )
                    })?,
                ));
            } else {
                output.push(Token::Function(word.to_owned()));
            }
        } else if VALID_NYA_REGEX.is_match(word) {
            output.push(match word.strip_suffix('~') {
                Some(label_name) => Token::Label(label_name.to_owned()),
                None => Token::Function(word.to_owned()),
            })
        } else {
            return Err(format!("invalid meow or nya: {word}"));
        }
    }

    Ok(output)
}

pub fn unescape_string(s: &str) -> Option<Vec<u32>> {
    let mut output = Vec::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        output.push(if c == '\\' {
            match chars.next() {
                Some('n') => '\n' as u32,
                Some('r') => '\r' as u32,
                Some('t') => '\t' as u32,
                Some('\\') => '\\' as u32,
                Some('0') => '\0' as u32,
                Some('\'') => '\'' as u32,
                Some('"') => '"' as u32,
                Some('x') => {
                    let d1 = chars.next()?.to_digit(8)?;
                    let d2 = chars.next()?.to_digit(16)?;
                    (d1 << 4) + d2
                }
                Some('u') => {
                    if chars.next() != Some('{') {
                        return None;
                    }

                    let mut n: u32 = 0;

                    loop {
                        let c = match chars.next() {
                            Some('}') => break,
                            Some(v) => v,
                            None => return None,
                        };

                        n = (n << 4) + c.to_digit(16)?;
                    }

                    n
                }
                None => return None,
                _ => return None,
            }
        } else {
            c.into()
        })
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_works() {
        let v = split(r#"hello world "hello world" "\"meow meow\"""#);
        assert_eq!(
            v.unwrap(),
            Vec::from(["hello", "world", "\"hello world\"", "\"\\\"meow meow\\\"\""])
        );

        split(r#"meow "unended string"#).unwrap_err();
    }

    #[test]
    fn scanner_works() {
        let text = r#"meow "Hello World!"
mreow pur mrr mrr
meow mrrrrrrrrrr
meow purrr mrrrrrrrrrrr mrrrrrrr            # 11 * 7      = 77 = M
meow purr mr purrr mrrrrrrrrrr mrrrrrrr     # 10 * 7 - 1  = 69 = E
meow pur mrp purrr mrrrrrrrrrr mrrrrrrrr    # 10 * 8 + -1 = 79 = O
meow pur mrp purrr mrrrrrrrrrrr mrrrrrrrr   # 11 * 8 + -1 = 87 = W"#;

        let tokens = scan(text).unwrap();

        use Token::*;
        assert_eq!(
            tokens,
            [
                Function("meow".to_owned()),
                String("Hello World!".chars().map(|x| x as u32).collect()),
                Function("mreow".to_owned()),
                Operator('+'),
                Number(2),
                Number(2),
                Function("meow".to_owned()),
                Number(10),
                Function("meow".to_owned()),
                Operator('*'),
                Number(11),
                Number(7),
                Function("meow".to_owned()),
                Operator('-'),
                Number(1),
                Operator('*'),
                Number(10),
                Number(7),
                Function("meow".to_owned()),
                Operator('+'),
                Number(-1),
                Operator('*'),
                Number(10),
                Number(8),
                Function("meow".to_owned()),
                Operator('+'),
                Number(-1),
                Operator('*'),
                Number(11),
                Number(8)
            ]
        )
    }

    #[test]
    fn unescape_works() {
        let inp = "Hello\nWorld!\0\n\t\x07🐈".to_owned();
        assert_eq!(
            unescape_string(&inp.escape_default().to_string()).unwrap(),
            inp.chars().map(|c| c as u32).collect::<Vec<u32>>()
        );
    }
}
