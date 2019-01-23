// VM instructions, state and interpreter

#[derive(Debug, PartialEq)]
pub enum Instruction {
  PushI(i64),
  PushF(f64),
  Pop,
  AddI,
  SubI,
  MulI,
  DivI,
  AddF,
  SubF,
  MulF,
  DivF
}

#[derive(Debug, PartialEq)]
pub enum VmError {
  OutOfBounds,
  OutOfMemory,
  DivByZero
}

pub trait VM {
  fn run(&mut self) -> Result<(), VmError>;
  
  fn get_int(&self, index: i32) -> Result<i64, VmError>; // get i-th element from the stack, negative index counts from the bottom
  fn get_float(&self, index: i32) -> Result<f64, VmError>;
  fn push_int(&mut self, val: i64) -> Result<(), VmError>;
  fn push_float(&mut self, val: f64) -> Result<(), VmError>;
  fn pop(&mut self) -> Result<(), VmError>;
  
  fn add_int(&mut self) -> Result<(), VmError>;
  fn sub_int(&mut self) -> Result<(), VmError>;
  fn mul_int(&mut self) -> Result<(), VmError>;
  fn div_int(&mut self) -> Result<(), VmError>;
  
  fn add_float(&mut self) -> Result<(), VmError>;
  fn sub_float(&mut self) -> Result<(), VmError>;
  fn mul_float(&mut self) -> Result<(), VmError>;
  fn div_float(&mut self) -> Result<(), VmError>;
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
  fn get_int(&self, index: i32) -> Result<i64, VmError> {
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

  fn get_float(&self, index: i32) -> Result<f64, VmError> {
    let ir = self.get_int(index)?;
    let r = unsafe { std::mem::transmute(ir) };
    Ok(r)
  }

  fn run(&mut self) -> Result<(), VmError> {
    use self::Instruction::*;

    while self.pc < self.code.len() {
      match self.code[self.pc] {
        PushI(val) => self.push_int(val)?,
        PushF(val) => self.push_float(val)?,
        Pop        => self.pop()?,
        AddI       => self.add_int()?,
        SubI       => self.sub_int()?,
        MulI       => self.mul_int()?,
        DivI       => self.div_int()?,
        AddF       => self.add_float()?,
        SubF       => self.sub_float()?,
        MulF       => self.mul_float()?,
        DivF       => self.div_float()?
      }
      self.pc += 1;
    }

    Ok(())
  }

  fn push_int(&mut self, val: i64) -> Result<(), VmError> {
    self.stack.push(val);
    Ok(())
  }

  fn push_float(&mut self, val: f64) -> Result<(), VmError> {
    let ival = unsafe { std::mem::transmute(val) };
    self.stack.push(ival);
    Ok(())
  }

  fn pop(&mut self) -> Result<(), VmError> {
    drop(self.stack.pop());
    Ok(())
  }

  fn add_int(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(a), Some(b)) => {
        self.stack.push(a + b);
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }

  fn sub_int(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(a), Some(b)) => {
        self.stack.push(b - a); // REVERSE ORDER
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }

  fn mul_int(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(a), Some(b)) => {
        self.stack.push(a * b);
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }

  fn div_int(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(a), Some(b)) => {
        if a == 0 {
          return Err(VmError::DivByZero)
        }
        self.stack.push(b / a); // REVERSE ORDER
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }

  fn add_float(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(ia), Some(ib)) => {
        let a: f64 = unsafe { std::mem::transmute(ia) };
        let b: f64 = unsafe { std::mem::transmute(ib) };
        let ir     = unsafe { std::mem::transmute(a + b) };
        self.stack.push(ir);
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }

  fn sub_float(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(ia), Some(ib)) => {
        let a: f64 = unsafe { std::mem::transmute(ia) };
        let b: f64 = unsafe { std::mem::transmute(ib) };
        let ir     = unsafe { std::mem::transmute(b - a) }; // REVERSE ORDER
        self.stack.push(ir);
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }

  fn mul_float(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(ia), Some(ib)) => {
        let a: f64 = unsafe { std::mem::transmute(ia) };
        let b: f64 = unsafe { std::mem::transmute(ib) };
        let ir     = unsafe { std::mem::transmute(a * b) };
        self.stack.push(ir);
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }

  fn div_float(&mut self) -> Result<(), VmError> {
    match (self.stack.pop(), self.stack.pop()) {
      (Some(ia), Some(ib)) => {
        let a: f64 = unsafe { std::mem::transmute(ia) };
        let b: f64 = unsafe { std::mem::transmute(ib) };
        if a == 0.0 {
          return Err(VmError::DivByZero)
        }
        let ir = unsafe { std::mem::transmute(b / a) };
        self.stack.push(ir); // REVERSE ORDER
        Ok(())
      },
      _ => Err(VmError::OutOfBounds)
    }
  }
}
