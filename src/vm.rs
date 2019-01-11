// VM instructions, state and interpreter

pub enum Instruction {
  Push(i64),
  Pop,
  Add,
  Sub,
  Mul,
  Div
}

pub enum VmError {
  OutOfBounds,
  OutOfMemory
}

pub trait VM {
  fn run(&mut self);
  fn get(&self, index: i32) -> Result<i64, VmError>; // get i-th element from the stack, negative index counts from the bottom
}

pub struct VmState<'a> {
  pc: u32,
  code: &'a [Instruction],
  stack: Vec<i64>
}

impl<'a> VmState<'a> {
  pub fn new(code: &'a [Instruction]) -> VmState<'a> {
    VmState {pc: 0, code, stack: vec![]}
  }
}

impl<'a> VM for VmState<'a> {
  fn get(&self, index: i32) -> Result<i64, VmError> {
    if index < 0 {
      let idx = self.stack.len() as i32 + index;
      if idx < 0 {
        return Err(VmError::OutOfBounds)
      }
      return Ok(self.stack[idx as usize])
    }

    let idx = index as usize;
    if idx > self.stack.len() - 1 {
      return Err(VmError::OutOfBounds)
    }
    Ok(self.stack[idx])
  }

  fn run(&mut self) {
    //
  }
}
