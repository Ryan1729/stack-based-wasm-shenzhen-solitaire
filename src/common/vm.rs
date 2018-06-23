use inner_common::GameState;

pub struct VM {
    stack_pointer: usize,
    stack: [u8; VM::STACK_SIZE],
}

use std::fmt;

impl fmt::Display for VM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.stack_pointer == usize::max_value() {
            return write!(f, "[ ]");
        }
        write!(f, "[")?;

        let mut sep = "";
        for i in 0..=self.stack_pointer {
            let elem = self.stack[i];

            write!(f, "{}{}", sep, elem)?;
            sep = ", ";
        }

        write!(f, "]")
    }
}

impl Default for VM {
    fn default() -> Self {
        VM {
            stack_pointer: usize::max_value(),
            stack: [0; VM::STACK_SIZE],
        }
    }
}

impl VM {
    pub const STACK_SIZE: usize = 128;

    pub fn pop(&mut self) -> u8 {
        let output = self.stack[self.stack_pointer];

        assert!(self.stack_pointer != usize::max_value(), "stack underflow!");

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        return output;
    }

    pub fn push(&mut self, byte: u8) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        assert!(self.stack_pointer != usize::max_value(), "stack overflow!");

        self.stack[self.stack_pointer] = byte;
    }
}
