use super::parser::OpValue;
use super::Token;

pub fn interpret(ops: &OpValue) -> f32 {
  match ops {
    OpValue::Int(n) => *n as f32,
    OpValue::Float(n) => *n,
    OpValue::UnaryOp(token, op) => {
      let op_result = interpret(op);
      match token {
        Token::Plus => op_result,
        Token::Minus => -op_result,
        _ => panic!("Invalid UnaryOp Token"),
      }
    }
    OpValue::BinOp(left, token, right) => {
      let left_result = interpret(left);
      let right_result = interpret(right);
      match token {
        Token::Plus => left_result + right_result,
        Token::Minus => left_result - right_result,
        Token::Times => left_result * right_result,
        Token::Divide => left_result / right_result,
        Token::Power => left_result.powf(right_result),
        Token::Mod => left_result % right_result,
        _ => panic!("Invalid BinOp Token"),
      }
    }
  }
}
