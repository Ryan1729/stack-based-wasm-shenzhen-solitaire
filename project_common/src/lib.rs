macro_rules! last_unchecked {
    ($vec:expr) => {
        $vec[$vec.len() - 1]
    };
}

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate bitflags;

pub mod inner_common;
pub use inner_common::*;

pub mod rendering;
pub use rendering::draw_winning_screen;
pub use rendering::Framebuffer;

pub mod game_state;
pub use game_state::*;

pub mod vm;
pub use vm::*;

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

pub fn canmovedragons(state: &GameState, suit: u8) -> bool {
    let mut count = 0;
    for i in 0..=CELLS_MAX_INDEX {
        let i = i as usize;
        if state.cells[i].len() > 0 && last_unchecked!(state.cells[i]) == suit * 10 {
            count += 1;
        }
    }

    if count < 4 {
        return false;
    }

    for i in 0..BUTTON_COLUMN {
        let i = i as usize;
        if state.cells[i].len() == 0 || last_unchecked!(state.cells[i]) == suit * 10 {
            return true;
        }
    }
    return false;
}

pub fn movedragons(state: &mut GameState) {
    let suit = state.selectdepth;
    let mut moveto = None;

    for i in 0..BUTTON_COLUMN {
        let i = i as usize;
        if state.cells[i].len() != 0
            && last_unchecked!(state.cells[i]) == suit * 10
            && moveto.is_none()
        {
            moveto = Some(i);
        }
    }
    if moveto.is_none() {
        for i in 0..BUTTON_COLUMN {
            let i = i as usize;
            if state.cells[i].len() == 0 {
                moveto = Some(i);
                break;
            }
        }
    }

    for i in 0..=CELLS_MAX_INDEX {
        let i = i as usize;
        if state.cells[i].len() != 0 && last_unchecked!(state.cells[i]) == suit * 10 {
            state.cells[i].pop();
        }
    }

    if let Some(moveto) = moveto {
        let moveto = moveto as usize;
        state.cells[moveto].push(CARD_BACK);
    }
}

pub fn cangrab(cells: &Cells, pos: u8, depth: u8) -> bool {
    let selection = getselection(cells, pos, depth);
    if selection.len() == 0 || (pos >= FLOWER_FOUNDATION && pos < START_OF_TABLEAU) {
        return false;
    }

    let mut lastsuit = 255;
    let mut lastnum = 255;
    let mut first = true;

    for &card in selection.iter() {
        if card == CARD_BACK {
            return false;
        }

        let suit = getsuit(card);
        let num = getcardnum(card);

        if !first {
            if suit == lastsuit || num == 0 || num != lastnum - 1 {
                return false;
            }
        }
        lastsuit = suit;
        lastnum = num;
        first = false;
    }

    return true;
}

pub fn getselection(cells: &Cells, pos: u8, depth: u8) -> Vec<u8> {
    let pos = pos as usize;
    let depth = depth as usize;

    let len = cells[pos].len();
    if len == 0 {
        return Vec::new();
    }

    let mut output = Vec::with_capacity(depth);

    for i in 1..=depth + 1 {
        let index = len - (depth + 1) + i - 1;
        output.push(cells[pos][index]);
    }

    return output;
}
