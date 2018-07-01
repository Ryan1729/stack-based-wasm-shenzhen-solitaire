pub mod rendering;
pub use rendering::draw_winning_screen;
pub use rendering::Framebuffer;

pub mod inner_common;
pub use inner_common::*;

pub mod game_state;
pub use game_state::*;

pub mod vm;
pub use vm::*;

pub struct State {
    pub game_state: GameState,
    pub framebuffer: Framebuffer,
    pub input: Input,
}

impl State {
    pub fn new() -> State {
        let framebuffer = Framebuffer::new();

        State {
            game_state: GameState::new(),
            framebuffer,
            input: Input::new(),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Input {
    pub gamepad: Button::Ty,
    pub previous_gamepad: Button::Ty,
}

impl Input {
    pub fn new() -> Self {
        Input {
            gamepad: Button::Ty::empty(),
            previous_gamepad: Button::Ty::empty(),
        }
    }

    pub fn pressed_this_frame(&self, buttons: Button::Ty) -> bool {
        !self.previous_gamepad.contains(buttons) && self.gamepad.contains(buttons)
    }
}

// These values are deliberately picked to be the same as the ones in NES' input registers.
pub mod Button {
    bitflags! {
        #[derive(Default)]
        pub flags Ty: u8 {
            const A          = 1 << 0,
            const B          = 1 << 1,
            const Select     = 1 << 2,
            const Start      = 1 << 3,
            const Up         = 1 << 4,
            const Down       = 1 << 5,
            const Left       = 1 << 6,
            const Right      = 1 << 7
        }
    }
}

pub fn movecards(state: &mut GameState, grabpos: u8, grabdepth: u8, droppos: u8) {
    let grabpos = grabpos as usize;
    let grabdepth = grabdepth as usize;
    let droppos = droppos as usize;
    if droppos <= END_OF_FOUNDATIONS as usize {
        if let Some(last) = state.cells[grabpos].pop() {
            if state.cells[droppos].len() > 0 {
                state.cells[droppos][0] = last;
            } else {
                state.cells[droppos].push(last);
            }
        }
    } else {
        let len = state.cells[grabpos].len();

        let temp: Vec<_> = state.cells[grabpos].drain(len - 1 - grabdepth..).collect();

        state.cells[droppos].extend(temp.into_iter());
    }
}

pub fn getsuit(card: u8) -> u8 {
    if card >= FLOWER_CARD {
        3
    } else if card >= FIRST_BLACK_CARD {
        2
    } else if card >= FIRST_GREEN_CARD {
        1
    } else {
        0
    }
}

pub fn getcardnum(card: u8) -> u8 {
    card - (getsuit(card) * 10)
}
