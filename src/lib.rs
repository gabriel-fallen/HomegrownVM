mod vm;

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
}
