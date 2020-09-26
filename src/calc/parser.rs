use super::Token;

#[derive(Debug)]
pub enum OpValue {
  Int(u32),
  Float(f32),
  UnaryOp(Token, Box<OpValue>),
  BinOp(Box<OpValue>, Token, Box<OpValue>),
}

pub struct Parser<'a> {
  tokens: &'a Vec<Token>,
  current_pos: usize,
}

impl Parser<'_> {
  pub fn new(tokens: &Vec<Token>) -> Parser {
    Parser {
      tokens,
      current_pos: 0,
    }
  }

  fn get_current(&self) -> Option<Token> {
    if self.current_pos < self.tokens.len() {
      if let Some(current) = self.tokens.get(self.current_pos) {
        return Some(current.clone());
      }
    }
    None
  }

  fn next(&mut self) {
    self.current_pos += 1;
  }

  pub fn parse(&mut self) -> OpValue {
    self.parse_expression()
  }

  fn parse_expression(&mut self) -> OpValue {
    let mut left = self.parse_term();
    loop {
      if let Some(current) = self.get_current() {
        if current == Token::Plus || current == Token::Minus {
          self.next();
          let right = self.parse_term();
          left = OpValue::BinOp(Box::new(left), current, Box::new(right))
        } else {
          break;
        }
      } else {
        break;
      }
    }
    left
  }

  fn parse_term(&mut self) -> OpValue {
    let mut left = self.parse_extra();
    loop {
      if let Some(current) = self.get_current() {
        if current == Token::Times || current == Token::Divide {
          self.next();
          let right = self.parse_extra();
          left = OpValue::BinOp(Box::new(left), current, Box::new(right))
        } else {
          break;
        }
      } else {
        break;
      }
    }
    left
  }

  fn parse_extra(&mut self) -> OpValue {
    let mut left = self.parse_factor();
    loop {
      if let Some(current) = self.get_current() {
        if current == Token::Power || current == Token::Mod {
          self.next();
          let right = self.parse_factor();
          left = OpValue::BinOp(Box::new(left), current, Box::new(right))
        } else {
          break;
        }
      } else {
        break;
      }
    }
    left
  }

  fn parse_factor(&mut self) -> OpValue {
    let error = "Error parsing: Missing factor";
    if let Some(current) = self.get_current() {
      self.next();

      return match current {
        Token::Int(n) => OpValue::Int(n),
        Token::Float(n) => OpValue::Float(n),
        Token::Plus | Token::Minus => {
          let next = self.parse_factor();
          OpValue::UnaryOp(current, Box::new(next))
        }
        Token::LParen => {
          let exp = self.parse_expression();
          self.next();
          exp
        }
        _ => panic!(error),
      };
    } else {
      panic!(error)
    }
  }
}
