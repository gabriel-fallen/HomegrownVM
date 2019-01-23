// AST and compiler

#[derive(Debug)]
pub enum Expr {
  LitI(i64),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>)
}

use crate::vm::Instruction::{self, *};
use self::Expr::*;

pub fn compile(expr: Expr) -> Vec<Instruction> {
  match expr {
    LitI(val) => vec![PushI(val)],
    Add(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(AddI);
      a
    },
    Sub(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(SubI);
      a
    },
    Mul(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(MulI);
      a
    },
    Div(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(DivI);
      a
    }
  }
}
