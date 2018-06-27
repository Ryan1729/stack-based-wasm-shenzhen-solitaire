use common::{GameState, MOVE_TIMER_MAX};

pub mod instructions {
    pub const NO_OP: u8 = 0;

    pub const FILL_MOVE_TIMER: u8 = 1;

    pub const GRAB: u8 = 0b10;
    pub const DROP: u8 = GRAB | 1;

    pub const ADD: u8 = 0b100;
    pub const SUB: u8 = ADD | 1;
    pub const MUL: u8 = ADD | 0b10;
    pub const DIV: u8 = ADD | 0b11;

    pub const MAX: u8 = 0b1001;
    pub const MIN: u8 = MAX | 0b10;

    pub const AND: u8 = 0b1000;
    pub const OR: u8 = 0b1110;
    pub const NOT: u8 = 0b1111;

    const EQ_FLAG: u8 = 0b001;
    const GT_FLAG: u8 = 0b010;
    const LT_FLAG: u8 = 0b100;
    const BRANCH: u8 = 0b1001_0000;

    pub const IF: u8 = BRANCH;
    pub const EQ_BRANCH: u8 = BRANCH | EQ_FLAG;
    pub const NE_BRANCH: u8 = BRANCH | GT_FLAG | LT_FLAG;
    pub const GT_BRANCH: u8 = BRANCH | GT_FLAG;
    pub const GE_BRANCH: u8 = GT_BRANCH | EQ_FLAG;
    pub const LT_BRANCH: u8 = BRANCH | LT_FLAG;
    pub const LE_BRANCH: u8 = LT_BRANCH | EQ_FLAG;
    pub const JUMP: u8 = BRANCH | EQ_FLAG | GT_FLAG | LT_FLAG;

    const BOOLEAN: u8 = 0b1010_0000;

    pub const EQ: u8 = BOOLEAN | EQ_FLAG;
    pub const NE: u8 = BOOLEAN | GT_FLAG | LT_FLAG;
    pub const GT: u8 = BOOLEAN | GT_FLAG;
    pub const GE: u8 = GT | EQ_FLAG;
    pub const LT: u8 = BOOLEAN | LT_FLAG;
    pub const LE: u8 = LT | EQ_FLAG;

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

    pub const ASSERT_EMPTY_STACK: u8 = 0b1111_0000;

    pub const GET_SELECT_DROP: u8 = 0b1111_1101;
    pub const GET_CELL_LEN: u8 = 0b1111_1110;
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
        macro_rules! jump_if {
            ($boolean:expr) => {
                *instruction_pointer += 1;
                if $boolean {
                    *instruction_pointer += bytecode[*instruction_pointer] as usize;
                }
            };
        }

        macro_rules! stack_bool {
            ($boolean:expr) => {
                if $boolean {
                    255
                } else {
                    0
                }
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
            MAX => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(if a > b { a } else { b });
            }
            MIN => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(if a < b { a } else { b });
            }
            AND => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a & b);
            }
            OR => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a | b);
            }
            NOT => {
                let a = self.vm.pop();
                self.vm.push(if a == 0 { 255 } else { 0 });
            }
            IF => {
                let a = self.vm.pop();

                jump_if!(a != 0);
            }
            EQ_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                jump_if!(a == b);
            }
            NE_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                jump_if!(a != b);
            }
            GT_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                jump_if!(a > b);
            }
            GE_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                jump_if!(a >= b);
            }
            LT_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                jump_if!(a < b);
            }
            LE_BRANCH => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                jump_if!(a <= b);
            }
            JUMP => {
                jump_if!(true);
            }
            EQ => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(stack_bool!(a == b));
            }
            NE => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(stack_bool!(a != b));
            }
            GT => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(stack_bool!(a > b));
            }
            GE => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(stack_bool!(a >= b));
            }
            LT => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(stack_bool!(a < b));
            }
            LE => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(stack_bool!(a <= b));
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
            ASSERT_EMPTY_STACK => {
                assert!(self.vm.is_empty(), "ASSERT_EMPTY_STACK failed!");
            }
            GET_SELECT_DROP => {
                self.vm.push(stack_bool!(self.selectdrop));
            }
            GET_CELL_LEN => {
                self.vm
                    .push(self.cells[self.selectpos as usize].len() as u8);
            }
            HALT => *instruction_pointer = bytecode.len() - 1,
            _ => unimplemented!(),
        }
    }
}
