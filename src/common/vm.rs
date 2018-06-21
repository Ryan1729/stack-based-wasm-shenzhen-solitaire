use inner_common::GameState;

pub struct VM {
    stack_pointer: usize,
    stack: [u8; VM::STACK_SIZE],
}

impl Default for VM {
    fn default() -> Self {
        VM {
            stack_pointer: 0,
            stack: [0; VM::STACK_SIZE],
        }
    }
}

impl VM {
    pub const STACK_SIZE: usize = 128;

    pub fn pop(&mut self) -> u8 {
        let output = self.stack[self.stack_pointer];

        self.stack_pointer = self.stack_pointer.saturating_sub(1);

        return output;
    }

    pub fn push(&mut self, byte: u8) {
        self.stack_pointer += 1;

        self.stack[self.stack_pointer] = byte;
    }
}
