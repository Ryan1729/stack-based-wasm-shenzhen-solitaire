use common::{GameState, MOVE_TIMER_MAX};

pub mod instructions {
    pub const NO_OP: u8 = 0;

    pub const GRAB: u8 = 0b10;
    pub const DROP: u8 = GRAB | 1;

    pub const ADD: u8 = 0b100;
    pub const SUB: u8 = ADD | 1;
    pub const MUL: u8 = ADD | 0b10;
    pub const DIV: u8 = ADD | 0b11;

    const EQ: u8 = 0b001;
    const GT: u8 = 0b010;
    const LT: u8 = 0b100;
    const BRANCH: u8 = 0b1001_0000;

    pub const NE_BRANCH: u8 = BRANCH;
    pub const EQ_BRANCH: u8 = NE_BRANCH | EQ;
    pub const GT_BRANCH: u8 = BRANCH | GT;
    pub const GE_BRANCH: u8 = GT_BRANCH | EQ;
    pub const LT_BRANCH: u8 = BRANCH | LT;
    pub const LE_BRANCH: u8 = LT_BRANCH | EQ;
    pub const JUMP: u8 = BRANCH | EQ | GT | LT;

    const SET: u8 = 0b010;
    const DEPTH: u8 = 0b0000_1000;
    const GRAB_COORDS: u8 = 0b100;

    pub const GET_SELECT_POS: u8 = 0b0100_0000;
    pub const SET_SELECT_POS: u8 = GET_SELECT_POS | SET;
    pub const GET_SELECT_DEPTH: u8 = GET_SELECT_POS | DEPTH;
    pub const SET_SELECT_DEPTH: u8 = GET_SELECT_DEPTH | SET;

    pub const GET_GRAB_POS: u8 = GET_SELECT_POS | GRAB_COORDS;
    pub const SET_GRAB_POS: u8 = GET_GRAB_POS | SET;
    pub const GET_GRAB_DEPTH: u8 = GET_GRAB_POS | DEPTH;
    pub const SET_GRAB_DEPTH: u8 = GET_GRAB_DEPTH | SET;

    pub const LITERAL: u8 = 0b101010;
    pub const FORGET: u8 = LITERAL | 1;

    pub const FILL_MOVE_TIMER: u8 = 0b1100_0011;

    pub const HALT: u8 = 0b1111_1111;
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
        macro_rules! jump {
            () => {
                *instruction_pointer += 1;
                *instruction_pointer += bytecode[*instruction_pointer] as usize;
            };
        }

        if cfg!(debug_assertions) || true {
            console!(log, format!("{}", self.vm));
        }

        match instruction {
            NO_OP => {}
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
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a + b);
            }
            SUB => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a - b);
            }
            MUL => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a * b);
            }
            DIV => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a / b);
            }
            NE_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                if a != b {
                    jump!();
                }
            }
            EQ_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                if a == b {
                    jump!();
                }
            }
            GT_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                if a > b {
                    jump!();
                }
            }
            GE_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                if a >= b {
                    jump!();
                }
            }
            LT_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                if a < b {
                    jump!();
                }
            }
            LE_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                if a <= b {
                    jump!();
                }
            }
            JUMP => {
                jump!();
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
            HALT => *instruction_pointer = bytecode.len() - 1,
            _ => unimplemented!(),
        }
    }
}
