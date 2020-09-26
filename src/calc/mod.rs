mod interpreter;
mod lexer;
mod parser;

use interpreter::interpret;
use lexer::Lexer;
use parser::Parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Plus,
  Minus,
  Times,
  Divide,
  Power,
  Mod,
  LParen,
  RParen,
  Int(u32),
  Float(f32),
}

pub fn calc(raw_data: &String) -> f32 {
  let mut lexer = Lexer::new(&raw_data);
  let tokens = lexer.tokenize();

  if tokens.len() >= 1 {
    let mut parser = Parser::new(&tokens);
    let parsed = parser.parse();

    interpret(&parsed)
  } else {
    0_f32
  }
}
