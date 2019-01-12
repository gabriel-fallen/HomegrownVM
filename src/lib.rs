mod vm;
mod compiler;

#[cfg(test)]
mod tests {
  use crate::vm::{*, Instruction::*};

  #[test]
  fn vm_works() {
    let code = [Push(1)];
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get(0), Ok(1));
  }

  #[test]
  fn vm_adds() {
    let code = [Push(1), Push(4), Add];
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get(0), Ok(5));
  }

  use crate::compiler::*;

  #[test]
  fn compiles_lit() {
    let expr = Expr::LitI(42);
    let code = compile(expr);
    assert_eq!(code, vec![Push(42)]);
  }

  #[test]
  fn compiles_add() {
    let expr = Expr::Add(Box::new(Expr::LitI(2)), Box::new(Expr::LitI(3)));
    let code = compile(expr);
    assert_eq!(code, vec![Push(2), Push(3), Add]);
  }

  #[test]
  fn vm_adds_compiled() {
    let expr = Expr::Add(Box::new(Expr::LitI(2)), Box::new(Expr::LitI(3)));
    let code = compile(expr);
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get(0), Ok(5));
  }
}
