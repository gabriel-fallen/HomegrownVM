mod vm;
mod compiler;

#[cfg(test)]
mod tests {
  use crate::vm::{*, Instruction::*};

  #[test]
  fn vm_works() {
    let code = [PushI(1)];
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get_int(0), Ok(1));
  }

  #[test]
  fn vm_adds() {
    let code = [PushI(1), PushI(4), AddI];
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get_int(0), Ok(5));
  }

  #[test]
  fn vm_subs() {
    let code = [PushI(1), PushI(4), SubI];
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get_int(0), Ok(-3));
  }

  #[test]
  fn vm_adds_float() {
    let code = [PushF(1.1), PushF(4.5), AddF];
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get_float(0), Ok(5.6));
  }

  use crate::compiler::{*, Expr::*};

  #[test]
  fn compiles_lit() {
    let expr = LitI(42);
    let code = compile(expr);
    assert_eq!(code, vec![PushI(42)]);
  }

  #[test]
  fn compiles_add() {
    let expr = Add(Box::new(LitI(2)), Box::new(LitI(3)));
    let code = compile(expr);
    assert_eq!(code, vec![PushI(2), PushI(3), AddI]);
  }

  #[test]
  fn vm_adds_compiled() {
    let expr = Add(Box::new(LitI(2)), Box::new(LitI(3)));
    let code = compile(expr);
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get_int(0), Ok(5));
  }

  #[test]
  fn vm_subs_compiled() {
    let expr = Sub(Box::new(LitI(2)), Box::new(LitI(3)));
    let code = compile(expr);
    let mut vm = VmState::new(&code);
    let res = vm.run();
    assert_eq!(res, Ok(()));
    assert_eq!(vm.get_int(0), Ok(-1));
  }
}
