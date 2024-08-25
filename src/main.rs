use std::{fmt::{Display, Error, Formatter}, process::exit};
mod lexer;

#[derive(Debug, Clone)]
enum Value {
    Int(i32),
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Int(int) => {
                write!(formatter, "{}", int)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Type {
    Int,
}

struct TypeErr {
    location: lexer::Location,
    message: String,
}

fn check_word(stack: &mut Vec<Type>, ip: &mut usize, token: lexer::Token) -> Result<(), TypeErr> {
    match token.value.as_str() {
        "print" => {
            if stack.is_empty() {
                Err(TypeErr {
                    location: token.location,
                    message: format!("TYPE ERROR: Attempting to print with an empty stack")
                })
            } else {
                stack.pop();
                *ip += 1;
                Ok(())
            }
        },
        _ => todo!(),
    }
}

fn check_program(program: Vec<lexer::Token>) -> Result<(), TypeErr> {
    let mut stack = vec![];
    let mut ip = 0;

    while ip < program.len() {
        let token = program[ip].clone();
        match token.kind {
            lexer::TokenKind::IntLiteral => {
                stack.push(Type::Int);
                ip += 1;
            },
            lexer::TokenKind::Word => {
                check_word(&mut stack, &mut ip, token)?;
            },
            lexer::TokenKind::Plus => {
                if stack.len() < 2 {
                    return Err(TypeErr {
                        location: token.location,
                        message: format!(
                            "Not enough arguments for `+` (plus) operation, expected 2, got {}",
                            stack.len()
                        ),
                    });
                }
                stack.pop();
                ip += 1;
            },
            lexer::TokenKind::Minus => {
                if stack.len() < 2 {
                    return Err(TypeErr {
                        location: token.location,
                        message: format!(
                            "Not enough arguments for `-` (minus) operation, expected 2, got {}",
                            stack.len()
                        ),
                    });
                }
                stack.pop();
                ip += 1;
            },
            lexer::TokenKind::Times => {
                if stack.len() < 2 {
                    return Err(TypeErr {
                        location: token.location,
                        message: format!(
                            "Not enough arguments for `*` (times) operation, expected 2, got {}",
                            stack.len()
                        ),
                    });
                }
                stack.pop();
                ip += 1;
            },
            lexer::TokenKind::Div => {
                if stack.len() < 2 {
                    return Err(TypeErr {
                        location: token.location,
                        message: format!(
                            "Not enough arguments for `/` (div) operation, expected 2, got {}",
                            stack.len()
                        ),
                    });
                }
                stack.pop();
                ip += 1;
            }
        }
    }

    Ok(())
}

fn interpret_word(stack: &mut Vec<Value>, ip: &mut usize, token: lexer::Token) {
    match token.value.as_str() {
        "print" => {
            let top = stack.pop().unwrap();
            println!("{}", top);
            *ip += 1;
        },
        _ => todo!(),
    }
}

fn interpret_program(program: Vec<lexer::Token>) {
    let mut stack = vec![];
    let mut ip = 0;

    while ip < program.len() {
        let token = program[ip].clone();
        match token.kind {
            lexer::TokenKind::IntLiteral => {
                stack.push(Value::Int(str::parse(&token.value).unwrap()));
                ip += 1;
            },
            lexer::TokenKind::Word => {
                interpret_word(&mut stack, &mut ip, token);
            },
            lexer::TokenKind::Plus => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        stack.push(Value::Int(left + right));
                    },
                }
                ip += 1;
            },
            lexer::TokenKind::Minus => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        stack.push(Value::Int(left - right));
                    },
                }
                ip += 1;
            },
            lexer::TokenKind::Times => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        stack.push(Value::Int(left * right));
                    },
                }
                ip += 1;
            },
            lexer::TokenKind::Div => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => {
                        stack.push(Value::Int(left / right));
                    },
                }
                ip += 1;
            }
        }
    }
}

fn main() {
    let program = "
        1 2 +
        print
        3 4 +
        print
        print
    ".to_string();

    let result = lexer::lex("<stdin>", program);

    match result {
        lexer::LexingResult::Err { location, message } => {
            eprintln!("{}: {}", location, message);
            exit(1);
        },
        lexer::LexingResult::Ok(tokens) => {
            match check_program(tokens.clone()) {
                Ok(()) => {},
                Err(TypeErr { location, message }) => {
                    eprintln!("{}: {}", location, message);
                    exit(1);
                }
            };
            interpret_program(tokens);
        }
    }
}