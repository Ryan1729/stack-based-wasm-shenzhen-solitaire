use *;

pub type Logger = Option<fn(&str) -> ()>;

pub struct VM {
    stack_pointer: usize,
    stack: [u8; VM::STACK_SIZE],
    pub logger: Logger,
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

impl VM {
    pub fn new(logger: Logger) -> Self {
        VM {
            stack_pointer: usize::max_value(),
            stack: [0; VM::STACK_SIZE],
            logger,
        }
    }
}

pub fn log(logger: Logger, s: &str) {
    if let Some(l) = logger {
        l(s);
    }
}

pub fn log_assert(logger: Logger, is_fine: bool, s: &str) {
    if !is_fine {
        log(logger, s);
    }
}

impl VM {
    pub const STACK_SIZE: usize = 512;

    pub fn pop(&mut self) -> u8 {
        log_assert(self.logger, !self.is_empty(), "stack underflow!");

        let output = self.stack[self.stack_pointer];

        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        return output;
    }

    pub fn push(&mut self, byte: u8) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        log_assert(self.logger, !self.is_empty(), "stack overflow!");

        self.stack[self.stack_pointer] = byte;
    }

    pub fn is_empty(&self) -> bool {
        self.stack_pointer == usize::max_value()
    }

    pub fn clear(&mut self) {
        self.stack_pointer = usize::max_value();
        for i in 0..VM::STACK_SIZE {
            self.stack[i] = 0;
        }
    }
}

macro_rules! consts_and_name_fn {
    ($name:ident, $const_type:ty, $($tokens:tt)*) => {
        $($tokens)*

        pub fn $name(c: $const_type) -> &'static str {
            consts_and_name_fn!(c, $($tokens)*);

            "unknown"
        }
    };
    ($c:ident, pub const $const_name:ident: $const_type:ty = $value:expr; $($tokens:tt)*) => {
        if $c == $value {
            return stringify!($const_name);
        }

        consts_and_name_fn!($c, $($tokens)*)
    };
    ($c:ident, const $const_name:ident: $const_type:ty = $value:expr; $($tokens:tt)*) => {
        consts_and_name_fn!($c, $($tokens)*)
    };
    ($c:ident,) => {}
}

pub mod instructions {
    consts_and_name_fn!{
        instruction_name, u8,
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

        pub const CAN_GRAB: u8 = 0b1110_0000;
        pub const HANDLE_BUTTON_PRESS: u8 = 0b1110_0001;

        pub const ASSERT_EMPTY_STACK: u8 = 0b1111_0000;

        pub const GET_CARD_NUM: u8 = 0b1111_1000;
        pub const GET_CARD_SUIT: u8 = 0b1111_1001;
        pub const GET_GRAB_CARD_OR_255: u8 = 0b1111_1010;
        pub const GET_DROP_CARD_OR_255: u8 = 0b1111_1011;
        pub const MOVE_CARDS: u8 = 0b1111_1100;
        pub const GET_SELECT_DROP: u8 = 0b1111_1101;
        pub const GET_CELL_LEN: u8 = 0b1111_1110;
        pub const HALT: u8 = 0b1111_1111;
    }
}
pub use self::instructions::*;

pub struct PrettyInstruction(pub u8);

impl fmt::Display for PrettyInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", instruction_name(self.0))
    }
}

impl fmt::Debug for PrettyInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", instruction_name(self.0))
    }
}

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
                if *instruction_pointer < bytecode.len() && $boolean {
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
            log(
                self.vm.logger,
                &format!(
                    "{:20} {} : {}",
                    *instruction_pointer,
                    self.vm,
                    PrettyInstruction(instruction),
                ),
            );
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
                if *instruction_pointer < bytecode.len() {
                    self.vm.push(bytecode[*instruction_pointer]);
                }
            }
            FORGET => {
                self.vm.pop();
            }
            ADD => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a.wrapping_add(b));
            }
            SUB => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a.wrapping_sub(b));
            }
            MUL => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(a.wrapping_mul(b));
            }
            DIV => {
                let b = self.vm.pop();
                let a = self.vm.pop();
                self.vm.push(if b != 0 {
                    a.wrapping_div(b)
                } else {
                    255
                });
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
            CAN_GRAB => {
                log(
                    self.vm.logger,
                    &format!(
                        "cells, pos, depth {:?} {:?} {:?}",
                        &self.cells, self.selectpos, self.selectdepth
                    ),
                );
                let output = if cangrab(&self.cells, self.selectpos, self.selectdepth) {
                    255
                } else {
                    0
                };

                self.vm.push(output);
            }
            HANDLE_BUTTON_PRESS => {
                if canmovedragons(self, self.selectdepth) {
                    movedragons(self);
                    self.interpret(&[DROP, FILL_MOVE_TIMER]);
                }
            }
            ASSERT_EMPTY_STACK => {
                assert!(self.vm.is_empty(), "ASSERT_EMPTY_STACK failed!");
            }
            GET_CARD_NUM => {
                let card = self.vm.pop();
                self.vm.push(getcardnum(card));
            }
            GET_CARD_SUIT => {
                let card = self.vm.pop();
                self.vm.push(getsuit(card));
            }
            GET_GRAB_CARD_OR_255 => {
                let grabpos = self.grabpos as usize & 15;
                let grabdepth = self.grabdepth as usize;
                let cells = &self.cells;

                let len = cells[grabpos].len();
                let output = if len == 0 || len - 1 < grabdepth {
                    255
                } else {
                    cells[grabpos][len - 1 - grabdepth]
                };

                self.vm.push(output);
            }
            GET_DROP_CARD_OR_255 => {
                let droppos = self.selectpos as usize & 15;
                let cells = &self.cells;

                let output = {
                    let len = cells[droppos].len();
                    if len == 0 {
                        255
                    } else {
                        last_unchecked!(cells[droppos])
                    }
                };

                self.vm.push(output);
            }
            MOVE_CARDS => {
                let droppos = self.vm.pop();
                let grabdepth = self.vm.pop();
                let grabpos = self.vm.pop();

                movecards(self, grabpos, grabdepth, droppos);
            }
            GET_SELECT_DROP => {
                self.vm.push(stack_bool!(self.selectdrop));
            }
            GET_CELL_LEN => {
                self.vm
                    .push(self.cells[self.selectpos as usize & 15].len() as u8);
            }
            HALT => *instruction_pointer = bytecode.len() - 1,
            _ => unimplemented!(),
        }
    }
}
