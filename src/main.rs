#![allow(unused)] // For beginning only.
use crate::prelude::*;
mod error;
mod prelude;
mod utils;

#[derive(Debug)]
enum Type {
	I32,
	F64,
	Str,
	Var,
	Null,
	Operator(Operators),
}

#[derive(Debug)]
enum Operators {
	Add,
	Sub,
	Mul,
	Div,
	Eq,
}

#[derive(Debug)]
struct Token {
	_type: Type,
	_value: String,
}

impl Token {
	fn new(_type: Type, _value: String) -> Option<Token> {
		match _type {
			Type::Null | Type::Str if _value.is_empty() => None,
			_ => Some(Token { _type, _value }),
		}
	}
}
fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars = input.chars().peekable();
	while let Some(&ch) = chars.peek() {
		match ch {
			'1'..='9' => {
				let mut value = String::new();
				while let Some(&digit) = chars.peek() {
					if digit.is_ascii_digit() {
						value.push(digit);
						chars.next();
					} else {
						break;
					}
				}
				tokens.push(Token::new(Type::I32, value).unwrap());
			}
			'a'..='z' => {
				let mut value = String::new();
				while let Some(&alphanumeric) = chars.peek() {
					if alphanumeric.is_ascii_alphabetic() {
						value.push(alphanumeric);
						chars.next();
					} else {
						break;
					}
				}
				tokens.push(Token::new(Type::Var, value).unwrap());
			}
			'.' => {
				chars.next();
				if let Some(&next_ch) = chars.peek() {
					if next_ch.is_ascii_digit() {
						let mut value = String::new();
						value.push('.');
						while let Some(&digit) = chars.peek() {
							if digit.is_ascii_digit() {
								value.push(digit);
								chars.next();
							} else {
								break;
							}
						}
						tokens.push(Token::new(Type::F64, value).unwrap());
					}
				}
			}
			'+' | '-' | '*' | '/' | '=' => {
				chars.next();
				tokens.push(Token::new(Type::Operator(get_operator(ch)), ch.to_string()).unwrap());
			}
			'"' => {
				chars.next();
				let mut value = String::new();
				while let Some(&quoted_char) = chars.peek() {
					chars.next();
					if quoted_char == '"' {
						break;
					}
					value.push(quoted_char);
				}
				tokens.push(Token::new(Type::Str, value).unwrap());
			}
			_ => {
				chars.next();
			}
		}
	}
	tokens
}
fn get_operator(ch: char) -> Operators {
	match ch {
		'+' => Operators::Add,
		'-' => Operators::Sub,
		'*' => Operators::Mul,
		'/' => Operators::Div,
		'=' => Operators::Eq,
		_ => panic!("Invalid operator!"),
	}
}
fn main() -> Result<()> {
	println!("is running!");
	let input = "var=\"Hello\"";
	let tokens = tokenize(input);
	println!("Tokens: {:?}\n", tokens[2]._type);
	Ok(())
}
