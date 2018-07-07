extern crate rand;

use self::rand::{Rng, SeedableRng};

use inner_common::*;

use vm::VM;

impl GameState {
    pub fn new(seed: [u8; 16], logger: Option<fn(&str) -> ()>) -> GameState {
        let mut cells: [Vec<u8>; 16] = Default::default();

        let mut deck = Vec::with_capacity(3 * (START_OF_TABLEAU as usize + 4) + 1);

        for i in 1..=MAX_SUIT_NUM {
            deck.push(i);
            deck.push(i + 10);
            deck.push(i + 20);
        }

        for _ in 1..=4 {
            deck.push(0);
            deck.push(10);
            deck.push(20);
        }

        deck.push(30);

        let mut rng = rand::XorShiftRng::from_seed(seed);

        let mut deckpos = START_OF_TABLEAU;
        while deck.len() > 0 {
            let index = rng.gen_range(0, deck.len());
            cells[deckpos as usize].push(deck.swap_remove(index));

            deckpos = if deckpos >= CELLS_MAX_INDEX {
                START_OF_TABLEAU
            } else {
                deckpos + 1
            };
        }

        GameState {
            cells,
            wins: 0,
            win_done: false,
            selectdrop: false,
            selectpos: START_OF_TABLEAU,
            selectdepth: 0,
            grabpos: 1,
            grabdepth: 0,
            movetimer: 0,
            vm: VM::new(logger),
            rng,
        }
    }

    pub fn reset(&mut self) {
        let logger = self.vm.logger.take();

        let new_seed = self.rng.gen();

        *self = GameState::new(new_seed, logger);
    }
}
