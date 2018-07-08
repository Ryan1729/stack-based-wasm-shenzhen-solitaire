extern crate project_common;
use project_common::vm::instructions::*;
use project_common::GameState;

extern crate rand;
use rand::{
    distributions::{Distribution, Standard, Uniform}, Rng, SeedableRng, XorShiftRng,
};

const INSTRUCTION_POOL: [u8; 48] = [
    NO_OP,
    FILL_MOVE_TIMER,
    GRAB,
    DROP,
    ADD,
    SUB,
    MUL,
    DIV,
    MAX,
    MIN,
    AND,
    OR,
    NOT,
    IF,
    EQ_BRANCH,
    NE_BRANCH,
    GT_BRANCH,
    GE_BRANCH,
    LT_BRANCH,
    LE_BRANCH,
    JUMP,
    EQ,
    NE,
    GT,
    GE,
    LT,
    LE,
    GET_SELECT_POS,
    SET_SELECT_POS,
    GET_SELECT_DEPTH,
    SET_SELECT_DEPTH,
    GET_GRAB_POS,
    SET_GRAB_POS,
    GET_GRAB_DEPTH,
    SET_GRAB_DEPTH,
    LITERAL,
    FORGET,
    CAN_GRAB,
    HANDLE_BUTTON_PRESS,
    ASSERT_EMPTY_STACK,
    GET_CARD_NUM,
    GET_CARD_SUIT,
    GET_GRAB_CARD_OR_255,
    GET_DROP_CARD_OR_255,
    MOVE_CARDS,
    GET_SELECT_DROP,
    GET_CELL_LEN,
    HALT,
];

const ZERO_PARAM_INSTRUCTION_POOL: [u8; 16] = [
    NO_OP,
    FILL_MOVE_TIMER,
    GRAB,
    DROP,
    JUMP,
    GET_SELECT_POS,
    GET_SELECT_DEPTH,
    GET_GRAB_POS,
    GET_GRAB_DEPTH,
    LITERAL,
    CAN_GRAB,
    GET_GRAB_CARD_OR_255,
    GET_DROP_CARD_OR_255,
    GET_SELECT_DROP,
    GET_CELL_LEN,
    HANDLE_BUTTON_PRESS
    //Leaving these out on purpose.
    //ASSERT_EMPTY_STACK,
    //HALT,
];

const ONE_PARAM_INSTRUCTION_POOL: [u8; 9] = [
    NOT,
    IF,
    FORGET,
    SET_SELECT_POS,
    SET_SELECT_DEPTH,
    SET_GRAB_POS,
    SET_GRAB_DEPTH,
    GET_CARD_NUM,
    GET_CARD_SUIT,
];

const TWO_PARAM_INSTRUCTION_POOL: [u8; 20] = [
    ADD, SUB, MUL, DIV, MAX, MIN, AND, OR, EQ_BRANCH, NE_BRANCH, GT_BRANCH, GE_BRANCH, LT_BRANCH,
    LE_BRANCH, EQ, NE, GT, GE, LT, LE,
];

const THREE_PARAM_INSTRUCTION_POOL: [u8; 1] = [MOVE_CARDS];

fn generate<R: Rng>(rng: &mut R, count: usize) -> Vec<u8> {
    let full_range = Uniform::from(0..INSTRUCTION_POOL.len());
    let zero_range = Uniform::from(0..ZERO_PARAM_INSTRUCTION_POOL.len());
    let one_range = Uniform::from(0..ONE_PARAM_INSTRUCTION_POOL.len());
    let two_range = Uniform::from(0..TWO_PARAM_INSTRUCTION_POOL.len());

    let zero_to_three = Uniform::from(0..4);

    let mut output = Vec::new();

    let mut stack_depth = 0;
    for _ in 0..count {
        let instruction = match stack_depth {
            0 => ZERO_PARAM_INSTRUCTION_POOL[zero_range.sample(rng)],
            1 => if rng.gen() {
                ONE_PARAM_INSTRUCTION_POOL[one_range.sample(rng)]
            } else {
                ZERO_PARAM_INSTRUCTION_POOL[zero_range.sample(rng)]
            },
            2 => match zero_to_three.sample(rng) {
                0 => ZERO_PARAM_INSTRUCTION_POOL[zero_range.sample(rng)],
                1 => ONE_PARAM_INSTRUCTION_POOL[one_range.sample(rng)],
                2 => TWO_PARAM_INSTRUCTION_POOL[two_range.sample(rng)],
                _ => MOVE_CARDS,
            },
            _ => INSTRUCTION_POOL[full_range.sample(rng)],
        };

        output.push(instruction);

        match instruction {
            GET_SELECT_POS | GET_SELECT_DEPTH | GET_GRAB_POS | GET_GRAB_DEPTH | LITERAL
            | CAN_GRAB | GET_GRAB_CARD_OR_255 | GET_DROP_CARD_OR_255 | GET_SELECT_DROP
            | GET_CELL_LEN => {
                stack_depth += 1;
            }
            IF | FORGET | SET_SELECT_POS | SET_SELECT_DEPTH | SET_GRAB_POS | SET_GRAB_DEPTH => {
                stack_depth -= 1;
            }
            ADD | SUB | MUL | DIV | MAX | MIN | AND | OR | EQ_BRANCH | NE_BRANCH | GT_BRANCH
            | GE_BRANCH | LT_BRANCH | LE_BRANCH | EQ | NE | GT | GE | LT | LE => {
                stack_depth -= 2;
            }
            _ => {}
        }
    }

    output
}

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    fn logger(s: &str) {
        println!("{}", s);
    }

    quickcheck!{
        fn does_not_over_or_underflow(pre_seed: (u64, u64)) -> bool {
            let seed = unsafe { std::mem::transmute(pre_seed) };

            let mut rng = XorShiftRng::from_seed(seed);

            let insructions = generate(&mut rng, 100);

            let mut game_state = GameState::new([0; 16], Some(logger));

            game_state.interpret(&insructions);

            //didn't panic
            true
        }
    }
}

use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let seed: [u8; 16] = {
        let mut args = std::env::args();
        //exe name
        args.next();

        args.next()
            .map(|s| {
                let mut result = [0u8; 16];
                let bytes = s.as_bytes();
                for i in 0..16 {
                    result[i] = bytes[i % bytes.len()];
                }
                result
            })
            .unwrap_or_else(|| {
                let since_the_epoch = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::new(42, 42));

                //The most significant 32 bits change too rarely to be useful.
                let seconds: u32 = since_the_epoch.as_secs() as u32;
                let nanos: u32 = since_the_epoch.subsec_nanos();

                let result = [seconds, nanos, seconds, nanos];

                unsafe { std::mem::transmute(result) }
            })
    };

    println!("\nUsing {:?} as a seed.\n", seed);

    let mut rng = XorShiftRng::from_seed(seed);

    let mut output = String::new();

    output.push('[');
    output.push('\n');

    let instructions = generate(&mut rng, 100);
    for instruction in instructions.iter() {
        output.push_str(&format!("    {},\n", instruction));
    }

    output.push('\n');
    output.push(']');
    output.push('\n');

    println!("{}", output);
}
