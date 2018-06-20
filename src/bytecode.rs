use common::GameState;

pub mod instructions {
    pub const DROP: u8 = 63;
}
pub use self::instructions::*;

impl GameState {
    pub fn interpret(&mut self, bytecode: &[u8]) {
        let len = bytecode.len();
        let mut i = 0;
        while i < len {
            let instruction = bytecode[i];

            self.interpret_instruction(bytecode, &mut i, instruction);

            i += 1;
        }
    }

    #[inline]
    pub fn interpret_instruction(
        &mut self,
        bytecode: &[u8],
        instruction_pointer: &mut usize,
        instruction: u8,
    ) {
        match instruction {
            DROP => {
                self.selectdrop = false;
            }
            _ => unimplemented!(),
        }
    }
}
