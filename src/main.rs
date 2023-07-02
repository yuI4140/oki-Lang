#![allow(unused)] // For beginning only.
use crate::prelude::*;
mod error;
mod prelude;
mod utils;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Type {
    I32,
    F64,
    Str,
    Var,
    Null,
    Operator(Operators),
}

impl Default for Type {
    fn default() -> Self {
        Type::Null
    }
}
#[derive(Debug,PartialEq, Clone, Copy)]
enum Operators {
	Add,
	Sub,
	Mul,
	Div,
	Eq,
}
#[derive(Debug, PartialEq, Clone, Default)]
struct Token {
	_type: Type,
	_value: String,
}
fn string_to_str(s: String) -> &'static str {
	let boxed_str: Box<str> = s.into_boxed_str();
	Box::leak(boxed_str)
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
#[derive(Debug, Clone, Default)]
struct Ast {
	root: Option<Node>,
	right: Option<Box<Node>>,
	left: Option<Box<Node>>,
}
impl Ast {
	fn new(tkl: &(Token, Token, Token)) -> Option<Ast> {
		let root = Node::new(tkl.0.clone())?;
		let right = Box::new(Node::new(tkl.1.clone())?);
		let left = Box::new(Node::new(tkl.2.clone())?);
		Some(Ast {
			root: Some(root),
			right: Some(right),
			left: Some(left),
		})
	}
}
#[derive(Debug, Clone)]
struct Node {
	token: Token,
}
impl Node {
	fn new(tk: Token) -> Option<Node> {
		match tk._type {
			Type::Null => None,
			_ => Some(Node { token: tk }),
		}
	}
}
fn convert<T: FromStr<Err = Error>>(string: &str) -> Result<T> {
	string.parse::<T>()
}
fn get_cloned_value<T: Clone + Default>(option: Option<T>) -> T {
    match option.as_ref() {
        Some(value) => value.clone(),
        None => T::default(),
    }
}
fn parser_op(ast: &Ast) -> i32 {
	if let (Some(root), Some(right), Some(left)) =
		(ast.root.as_ref(), ast.right.as_ref(), ast.left.as_ref())
	{
		let _t = Token::new(Type::I32, String::from(" "));
		let a = root.token.clone();
		let op = right.token.clone();
		let b = left.token.clone();
		if (a._type != get_cloned_value(_t.clone())._type && b._type != get_cloned_value(_t.clone())._type) {
			return 0;
		}
	}
	return 0;
}
fn main() -> Result<()> {
	println!("is running!");

	let input = "var=2+2";
	let tokens = tokenize(input);
	let ast = Ast::new(&(tokens[2].clone(), tokens[3].clone(), tokens[4].clone()));
	parser_op(&get_cloned_value(ast));
	Ok(())
}
