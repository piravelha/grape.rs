use regex::Regex;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone)]
pub enum TokenKind {
    IntLiteral,
    Word,
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub file: String,
    pub line: i32,
    pub column: i32,
}

impl Display for Location {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "{}:{}:{}", self.file, self.line, self.column)?;
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: Location,
}

impl Display for Token {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "[{:?}:'{}']", self.kind, self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum LexingResult {
    Ok(Vec<Token>),
    Err {
        location: Location,
        message: String,
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

pub fn lex(file_path: &str, mut input: String) -> LexingResult {
    let patterns = vec![
        (new_pattern(r"\d+"), TokenKind::IntLiteral),
        (new_pattern(r"\+"), TokenKind::Plus),
        (new_pattern(r"-"), TokenKind::Minus),
        (new_pattern(r"\*"), TokenKind::Times),
        (new_pattern(r"/"), TokenKind::Div),
        (new_pattern(r"[a-zA-Z_][a-zA-Z_0-9]*"), TokenKind::Word),
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