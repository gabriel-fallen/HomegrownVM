// AST and compiler

#[derive(Debug)]
pub enum Expr {
  LitI(i64),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>)
}

use crate::vm::Instruction as Inst;
use self::Expr::*;

pub fn compile(expr: Expr) -> Vec<Inst> {
  match expr {
    LitI(val) => vec![Inst::Push(val)],
    Add(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(Inst::Add);
      a
    },
    Sub(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(Inst::Sub);
      a
    },
    Mul(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(Inst::Add);
      a
    },
    Div(e1, e2) => {
      let mut a = compile(*e1);
      let mut b = compile(*e2);
      a.append(&mut b);
      a.push(Inst::Add);
      a
    }
  }
}
