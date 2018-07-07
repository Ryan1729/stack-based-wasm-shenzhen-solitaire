extern crate project_common;
use project_common::vm::instructions::*;

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

extern crate rand;
use rand::{
    distributions::{Distribution, Standard, Uniform}, Rng, SeedableRng, XorShiftRng,
};

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

                let seconds: [u32; 2] = unsafe { std::mem::transmute(since_the_epoch.as_secs()) };

                let result = [seconds[0], seconds[1], seconds[0], seconds[1]];

                unsafe { std::mem::transmute(result) }
            })
    };

    println!("\nUsing {:?} as a seed.\n", seed);

    let mut rng = XorShiftRng::from_seed(seed);

    let mut output = String::new();

    output.push('[');
    output.push('\n');

    let range = Uniform::from(0..INSTRUCTION_POOL.len());
    for _ in 0..100 {
        let index = range.sample(&mut rng);

        let instruction = INSTRUCTION_POOL[index];

        output.push_str(&format!("    {},\n", instruction));
    }

    output.push('\n');
    output.push(']');
    output.push('\n');

    println!("{}", output);
}
