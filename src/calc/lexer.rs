use super::Token;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct Lexer {
  raw_data: String,
  current_pos: usize,
}

impl Lexer {
  pub fn new(raw_data: &String) -> Lexer {
    Lexer {
      raw_data: raw_data.clone(),
      current_pos: 0,
    }
  }

  fn throw_error(&self, pos: (usize, usize), msg: &str) {
    println!("ERROR TOKENIZING");
    println!("{}", &self.raw_data);
    println!("{}{} {}", " ".repeat(pos.0), "^".repeat(pos.1 - pos.0), msg);
    std::process::exit(1);
  }

  fn get_current(&self) -> char {
    self
      .raw_data
      .chars()
      .nth(self.current_pos)
      .expect("Error tokenizing: Invalid position")
  }

  fn tokenize_number(&mut self) -> Token {
    let mut number = String::from("");
    let mut dots = 0;

    while self.current_pos < self.raw_data.len() {
      let c = self.get_current();

      if DIGITS.contains(&c) {
        number.push(c);
        self.current_pos += 1;
      } else if c == '.' {
        if dots == 0 {
          dots += 1;
          number.push(c);
        } else {
          self.throw_error((self.current_pos, self.current_pos + 1), "Extra dot");
        }
        self.current_pos += 1;
      } else {
        break;
      }
    }

    if dots == 0 {
      match number.parse() {
        Ok(n) => Token::Int(n),
        Err(_) => {
          self.throw_error(
            (self.current_pos - number.len(), self.current_pos),
            "Error parsing integer",
          );
          Token::Int(0)
        }
      }
    } else {
      match number.parse() {
        Ok(n) => Token::Float(n),
        Err(_) => {
          self.throw_error(
            (self.current_pos - number.len(), self.current_pos),
            "Error parsing float",
          );
          Token::Float(0.0)
        }
      }
    }
  }

  pub fn tokenize(&mut self) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    while self.current_pos < self.raw_data.len() {
      let c = self.get_current();

      if DIGITS.contains(&c) || c == '.' {
        tokens.push(self.tokenize_number());
      } else {
        match c {
          '+' => tokens.push(Token::Plus),
          '-' => tokens.push(Token::Minus),
          '*' => tokens.push(Token::Times),
          '/' => tokens.push(Token::Divide),
          '^' => tokens.push(Token::Power),
          '%' => tokens.push(Token::Mod),
          '(' => tokens.push(Token::LParen),
          ')' => tokens.push(Token::RParen),
          ' ' => (),
          _ => self.throw_error((self.current_pos, self.current_pos + 1), "Unknown token"),
        }
        self.current_pos += 1;
      }
    }

    tokens
  }
}
