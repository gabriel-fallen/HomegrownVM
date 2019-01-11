mod vm;

#[cfg(test)]
mod tests {
    use crate::vm::*;
    
    #[test]
    fn it_works() {
        let code = [Instruction::Push(1)];
        let mut vm = VM_state::new(&code);
    }
}
