use regex::Regex;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone)]
enum TokenKind {
    IntLiteral,
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Debug, Clone)]
struct Location {
    file: String,
    line: i32,
    column: i32,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    value: String,
    location: Location,
}

#[derive(Debug, Clone)]
enum LexingResult {
    Ok(Vec<Token>),
    Err {
        location: Location,
        message: String,
    }
}

impl Display for LexingResult {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            LexingResult::Ok(tokens) => write!(formatter, "Ok({:?})", tokens)?,
            LexingResult::Err { location, message } => write!(
                formatter,
                "{}:{}:{}: {}",
                location.file,
                location.line,
                location.column,
                message,
            )?,
        }
        Ok(())
    }
}

fn new_pattern(re: &str) -> Regex {
    Regex::new(&format!("^({})", re)).unwrap()
}

fn update_location(mut line: i32, mut column: i32, input: &str) -> (i32, i32) {
    for char in input.chars() {
        if char == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn lex(file_path: &str, mut input: String) -> LexingResult {
    let patterns = vec![
        (new_pattern(r"\d+"), TokenKind::IntLiteral),
        (new_pattern(r"\+"), TokenKind::Plus),
        (new_pattern(r"-"), TokenKind::Minus),
        (new_pattern(r"\*"), TokenKind::Times),
        (new_pattern(r"/"), TokenKind::Div),
    ];
    let mut tokens = vec![];
    let (mut line, mut column) = (1, 1);
    while !input.is_empty() {
        (line, column) = update_location(
            line,
            column,
            &input[..input.len() - input.trim_start().len()]
        );
        input = input.trim_start().to_string();
        if input.is_empty() {
            break;
        }
        let mut matched = false;
        for (pattern, kind) in patterns.clone().into_iter() {
            if let Some(captures) = pattern.captures(&input) {
                let value = &captures[0];
                tokens.push(Token {
                    kind,
                    value: value.to_string(),
                    location: Location {
                        file: file_path.to_string(),
                        line,
                        column
                    },
                });
                (line, column) = update_location(
                    line,
                    column,
                    &input[..value.len()],
                );
                input = input[value.len()..].to_string();
                matched = true;
            }
        }
        if !matched {
            return LexingResult::Err {
                location: Location {
                    file: file_path.to_string(),
                    line,
                    column,
                },
                message: format!(
                    "SYNTAX ERROR: Invalid character: `{}`",
                    input.chars().next().unwrap(),
                ),
            }
        }
    }
    LexingResult::Ok(tokens)
}

fn main() {
    let program = "
        1 2 +
        error
    ".to_string();

    let result = lex("<stdin>", program);

    println!("{}", result);
}