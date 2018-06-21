use common::{GameState, MOVE_TIMER_MAX};

pub mod instructions {
    pub const NO_OP: u8 = 0;

    pub const GRAB: u8 = 0b10;
    pub const DROP: u8 = GRAB | 1;

    const SET: u8 = 0b010;
    const GRAB_COORDS: u8 = 0b100;

    pub const GET_SELECT_COORDS: u8 = 0b0100_0000;
    pub const SET_SELECT_COORDS: u8 = GET_SELECT_COORDS | SET;
    pub const GET_GRAB_COORDS: u8 = GET_SELECT_COORDS | GRAB_COORDS;
    pub const SET_GRAB_COORDS: u8 = GET_SELECT_COORDS | SET | GRAB_COORDS;

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
            GET_SELECT_COORDS => {
                self.vm.push(self.selectdepth);
                self.vm.push(self.selectpos);
            }
            GET_GRAB_COORDS => {
                self.vm.push(self.grabdepth);
                self.vm.push(self.grabpos);
            }
            SET_SELECT_COORDS => {
                self.selectpos = self.vm.pop();
                self.selectdepth = self.vm.pop();
            }
            SET_GRAB_COORDS => {
                self.grabpos = self.vm.pop();
                self.grabdepth = self.vm.pop();
            }
            FILL_MOVE_TIMER => {
                self.movetimer = MOVE_TIMER_MAX;
            }
            _ => unimplemented!(),
        }
    }
}
