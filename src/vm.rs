// VM instructions, state and interpreter

#[derive(Debug)]
pub enum Instruction {
  Push(i64),
  Pop,
  Add,
  Sub,
  Mul,
  Div
}

#[derive(Debug, PartialEq)]
pub enum VmError {
  OutOfBounds,
  OutOfMemory,
  DivByZero
}

pub trait VM {
  fn run(&mut self) -> Result<(), VmError>;
  fn get(&self, index: i32) -> Result<i64, VmError>; // get i-th element from the stack, negative index counts from the bottom
}

pub struct VmState<'a> {
  pc: usize,
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

  fn run(&mut self) -> Result<(), VmError> {
    use self::Instruction::*;

    while self.pc < self.code.len() {
      match self.code[self.pc] {
        Push(val) => self.stack.push(val),
        Pop => drop(self.stack.pop()),
        Add => {
          let res = add(self.stack.pop(), self.stack.pop())?;
          self.stack.push(res)
        },
        Sub => {
          let res = sub(self.stack.pop(), self.stack.pop())?;
          self.stack.push(res)
        },
        Mul => {
          let res = mul(self.stack.pop(), self.stack.pop())?;
          self.stack.push(res)
        },
        Div => {
          let res = div(self.stack.pop(), self.stack.pop())?;
          self.stack.push(res)
        },
      }
      self.pc += 1;
    }

    Ok(())
  }
}

fn add(val1: Option<i64>, val2: Option<i64>) -> Result<i64, VmError> {
  match (val1, val2) {
    (Some(a), Some(b)) => Ok(a + b),
    _ => Err(VmError::OutOfBounds)
  }
}

fn sub(val1: Option<i64>, val2: Option<i64>) -> Result<i64, VmError> {
  match (val1, val2) {
    (Some(a), Some(b)) => Ok(a - b),
    _ => Err(VmError::OutOfBounds)
  }
}

fn mul(val1: Option<i64>, val2: Option<i64>) -> Result<i64, VmError> {
  match (val1, val2) {
    (Some(a), Some(b)) => Ok(a * b),
    _ => Err(VmError::OutOfBounds)
  }
}

fn div(val1: Option<i64>, val2: Option<i64>) -> Result<i64, VmError> {
  match (val1, val2) {
    (Some(a), Some(b)) => {
      if b == 0 {
        return Err(VmError::DivByZero)
      }
      Ok(a / b)
    },
    _ => Err(VmError::OutOfBounds)
  }
}
