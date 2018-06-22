use common::{GameState, MOVE_TIMER_MAX};

pub mod instructions {
    pub const NO_OP: u8 = 0;

    pub const GRAB: u8 = 0b10;
    pub const DROP: u8 = GRAB | 1;

    pub const ADD: u8 = 0b100;
    pub const SUB: u8 = ADD | 1;
    pub const MUL: u8 = ADD | 0b10;
    pub const DIV: u8 = ADD | 0b11;

    pub const LITERAL: u8 = 0b101010;
    pub const FORGET: u8 = LITERAL | 1;

    const SET: u8 = 0b010;
    const DEPTH: u8 = 0b0001_0000;
    const GRAB_COORDS: u8 = 0b100;

    pub const GET_SELECT_POS: u8 = 0b0100_0000;
    pub const SET_SELECT_POS: u8 = GET_SELECT_POS | SET;
    pub const GET_SELECT_DEPTH: u8 = GET_SELECT_POS | DEPTH;
    pub const SET_SELECT_DEPTH: u8 = GET_SELECT_DEPTH | SET;

    pub const GET_GRAB_POS: u8 = GET_SELECT_POS | GRAB_COORDS;
    pub const SET_GRAB_POS: u8 = GET_GRAB_POS | SET;
    pub const GET_GRAB_DEPTH: u8 = GET_GRAB_POS | DEPTH;
    pub const SET_GRAB_DEPTH: u8 = GET_GRAB_DEPTH | SET;

    pub const FILL_MOVE_TIMER: u8 = 0b1100_0011;

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

        //TODO clear the stack?
    }

    #[inline]
    pub fn interpret_instruction(
        &mut self,
        bytecode: &[u8],
        instruction_pointer: &mut usize,
        instruction: u8,
    ) {
        match instruction {
            GRAB => {
                self.selectdrop = true;
            }
            DROP => {
                self.selectdrop = false;
            }
            LITERAL => {
                *instruction_pointer += 1;
                self.vm.push(bytecode[*instruction_pointer]);
            }
            FORGET => {
                self.vm.pop();
            }
            ADD => {
                let a = self.vm.pop();
                let b = self.vm.pop();
                self.vm.push(a + b);
            }
            SUB => {
                let a = self.vm.pop();
                let b = self.vm.pop();
                self.vm.push(a - b);
            }
            MUL => {
                let a = self.vm.pop();
                let b = self.vm.pop();
                self.vm.push(a * b);
            }
            DIV => {
                let a = self.vm.pop();
                let b = self.vm.pop();
                self.vm.push(a / b);
            }
            GET_SELECT_POS => self.vm.push(self.selectpos),
            SET_SELECT_POS => {
                self.selectpos = self.vm.pop();
            }
            GET_SELECT_DEPTH => self.vm.push(self.selectdepth),
            SET_SELECT_DEPTH => {
                self.selectdepth = self.vm.pop();
            }
            GET_GRAB_POS => self.vm.push(self.grabpos),
            SET_GRAB_POS => {
                self.grabpos = self.vm.pop();
            }
            GET_GRAB_DEPTH => self.vm.push(self.grabdepth),
            SET_GRAB_DEPTH => {
                self.grabdepth = self.vm.pop();
            }
            FILL_MOVE_TIMER => {
                self.movetimer = MOVE_TIMER_MAX;
            }

            _ => unimplemented!(),
        }
    }
}
