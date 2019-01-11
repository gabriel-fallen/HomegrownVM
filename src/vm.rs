// VM instructions, state and interpreter

pub enum Instruction {
  Push(i64),
  Pop,
  Add,
  Sub,
  Mul,
  Div
}

pub trait VM {
  fn run(&mut self);
  fn get(&self, index: i32) -> i64; // get i-th element from the stack, negative index counts from the bottom
}

pub struct VM_state<'a> {
  pc: u32,
  code: &'a [Instruction],
  stack: Vec<i64>
}

impl<'a> VM_state<'a> {
  pub fn new(code: &'a [Instruction]) -> VM_state<'a> {
    VM_state {pc: 0, code, stack: vec![]}
  }
}
