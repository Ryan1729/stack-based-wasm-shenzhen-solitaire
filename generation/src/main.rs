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
pub use self::instructions::*;

const INSTRUCTIONS: [u8; 48] = [
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

    let range = Uniform::from(0..INSTRUCTIONS.len());
    for _ in 0..100 {
        let index = range.sample(&mut rng);

        let instruction = INSTRUCTIONS[index];

        output.push_str(&format!("    {},\n", instruction));
    }

    output.push('\n');
    output.push(']');
    output.push('\n');

    println!("{}", output);
}
