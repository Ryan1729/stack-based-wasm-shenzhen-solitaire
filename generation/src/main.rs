extern crate project_common;
use project_common::vm::instructions::*;
use project_common::vm::PrettyInstruction;
use project_common::GameState;

extern crate rand;
use rand::{
    distributions::{Distribution, Uniform}, Rng, SeedableRng, XorShiftRng,
};

use std::cmp::min;

const INSTRUCTION_POOL: [u8; 47] = [
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
    //Leaving this out on purpose.
    //ASSERT_EMPTY_STACK,
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

#[allow(dead_code)]
const THREE_PARAM_INSTRUCTION_POOL: [u8; 1] = [MOVE_CARDS];

fn generate<R: Rng>(rng: &mut R, count: usize) -> Vec<u8> {
    let full_range = Uniform::from(0..INSTRUCTION_POOL.len());
    let zero_range = Uniform::from(0..ZERO_PARAM_INSTRUCTION_POOL.len());
    let one_range = Uniform::from(0..ONE_PARAM_INSTRUCTION_POOL.len());
    let two_range = Uniform::from(0..TWO_PARAM_INSTRUCTION_POOL.len());

    let zero_to_two = Uniform::from(0..3);

    let mut output = Vec::new();

    //TODO:: If count turns out to get very high then this should be a hashmap instead.
    let mut restrictions = vec![255; count];

    let mut stack_depth: u8 = 0;

    while output.len() < count {
        let len = output.len();

        stack_depth = min(stack_depth, restrictions[len]);
        println!("{:?} {:?}", len, stack_depth);
        let instruction = match stack_depth {
            0 => ZERO_PARAM_INSTRUCTION_POOL[zero_range.sample(rng)],
            1 => if rng.gen() {
                ONE_PARAM_INSTRUCTION_POOL[one_range.sample(rng)]
            } else {
                ZERO_PARAM_INSTRUCTION_POOL[zero_range.sample(rng)]
            },
            2 => match zero_to_two.sample(rng) {
                0 => ZERO_PARAM_INSTRUCTION_POOL[zero_range.sample(rng)],
                1 => ONE_PARAM_INSTRUCTION_POOL[one_range.sample(rng)],
                _ => TWO_PARAM_INSTRUCTION_POOL[two_range.sample(rng)],
            },
            _ => INSTRUCTION_POOL[full_range.sample(rng)],
        };

        match instruction {
            GET_SELECT_POS | GET_SELECT_DEPTH | GET_GRAB_POS | GET_GRAB_DEPTH | LITERAL
            | CAN_GRAB | GET_GRAB_CARD_OR_255 | GET_DROP_CARD_OR_255 | GET_SELECT_DROP
            | GET_CELL_LEN => {
                let previous_instruction = output.last().cloned().unwrap_or(NO_OP);
                if previous_instruction != LITERAL {
                    stack_depth += 1;
                }
            }
            IF | FORGET | SET_SELECT_POS | SET_SELECT_DEPTH | SET_GRAB_POS | SET_GRAB_DEPTH => {
                stack_depth -= 1;
            }
            ADD | SUB | MUL | DIV | MAX | MIN | AND | OR | EQ_BRANCH | NE_BRANCH | GT_BRANCH
            | GE_BRANCH | LT_BRANCH | LE_BRANCH | EQ | NE | GT | GE | LT | LE => {
                stack_depth -= 2;
            }
            MOVE_CARDS => {
                stack_depth -= 3;
            }
            _ => {}
        }

        match instruction {
            IF | EQ_BRANCH | NE_BRANCH | GT_BRANCH | GE_BRANCH | LT_BRANCH | LE_BRANCH | JUMP => {
                if len == count - 1 {
                    output.push(NO_OP);
                    break;
                }

                let maximum_valid_target = min((count - 1) - (len + 1), 255) as u8;

                // Choosing target this way is a bit of a cop-out, which reduces the range of the generator.
                let target =
                    rng.choose(
                        &ZERO_PARAM_INSTRUCTION_POOL
                            .iter()
                            .cloned()
                            .filter(|&inst| inst < maximum_valid_target && inst != JUMP)
                            .collect::<Vec<_>>(),
                    ).cloned()
                        .unwrap_or(0);
                // It could be something like this:
                // `min(gen_range_or_0(rng, 0, maximum_valid_target), 255) as u8;`
                // but then we'd need to prevent jumping into the generated instruction.

                output.push(instruction);
                {
                    let len = output.len();
                    if len < count {
                        stack_depth = min(stack_depth, restrictions[len]);
                    }
                    println!("+{:?} {:?}", len, stack_depth);
                }
                output.push(target);

                let absolute_target = min(len + 2 + target as usize, count - 1);

                println!(
                    "restrictions[{:?}] = {:?}",
                    absolute_target,
                    min(restrictions[absolute_target], stack_depth)
                );
                restrictions[absolute_target] = min(restrictions[absolute_target], stack_depth);
            }
            _ => {
                output.push(instruction);
            }
        }
    }

    output
}

#[allow(dead_code)]
fn gen_range_or_0<R: Rng>(rng: &mut R, low: usize, high: usize) -> usize {
    if low >= high {
        0
    } else {
        rng.gen_range(low, high)
    }
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

    #[test]
    fn jump_at_the_end_does_not_crash() {
        let mut game_state = GameState::new([0; 16], None);

        game_state.interpret(&[JUMP]);
    }

    #[test]
    fn jump_past_the_end_does_not_crash() {
        let mut game_state = GameState::new([0; 16], None);

        game_state.interpret(&[JUMP, 255]);
    }

    #[test]
    fn dissect() {
        let mut game_state = GameState::new([0; 16], Some(logger));

        game_state.interpret(&[
            GET_SELECT_DEPTH,
            LITERAL,
            AND,
            HANDLE_BUTTON_PRESS,
            NO_OP,
            GET_SELECT_POS,
            LITERAL,
            NOT,
            JUMP,
            GET_SELECT_POS,
            JUMP,
            NO_OP,
            GET_CARD_NUM,
            GET_GRAB_DEPTH,
            LT,
            SET_SELECT_DEPTH,
            GET_GRAB_DEPTH,
            GET_GRAB_CARD_OR_255,
            GET_DROP_CARD_OR_255,
            GET_CARD_SUIT,
            ADD,
            GET_CARD_SUIT,
            SET_SELECT_POS,
            GET_GRAB_CARD_OR_255,
            CAN_GRAB,
            GET_CARD_SUIT,
            LT,
            GET_GRAB_POS,
            LITERAL,
            GET_CARD_SUIT,
            GET_SELECT_DEPTH,
            GET_GRAB_DEPTH,
            EQ_BRANCH,
            FILL_MOVE_TIMER,
            GET_CARD_SUIT,
            GET_CARD_NUM,
            FILL_MOVE_TIMER,
            FORGET,
            GET_SELECT_DEPTH,
            NE,
            LITERAL,
            GRAB,
            GET_SELECT_POS,
            SET_GRAB_POS,
            SET_GRAB_DEPTH,
            GET_DROP_CARD_OR_255,
            GET_CARD_NUM,
            FILL_MOVE_TIMER,
            GET_GRAB_POS,
            HANDLE_BUTTON_PRESS,
            MIN,
            HANDLE_BUTTON_PRESS,
            GET_DROP_CARD_OR_255,
            SET_SELECT_DEPTH,
            GET_SELECT_DROP,
            GET_GRAB_DEPTH,
            HANDLE_BUTTON_PRESS,
            SET_GRAB_POS,
            NOT,
            NOT,
            SET_SELECT_DEPTH,
            CAN_GRAB,
            JUMP,
            GET_SELECT_DEPTH,
            NO_OP,
            GET_SELECT_DROP,
            GET_CARD_NUM,
            FORGET,
            SET_SELECT_POS,
            GET_SELECT_DEPTH,
            LITERAL,
            GRAB,
            LITERAL,
            SET_GRAB_DEPTH,
            GET_SELECT_POS,
            GET_GRAB_CARD_OR_255,
            SET_GRAB_POS,
            FILL_MOVE_TIMER,
            IF,
            GET_SELECT_DEPTH,
            GET_CELL_LEN,
            GET_GRAB_DEPTH,
            LT_BRANCH,
            GET_GRAB_DEPTH,
            SET_SELECT_DEPTH,
            SET_SELECT_POS,
            HANDLE_BUTTON_PRESS,
            GET_SELECT_POS,
            NOT,
            GET_CARD_NUM,
            DROP,
            NO_OP,
            SET_SELECT_POS,
            GET_CELL_LEN,
            NOT,
            GET_GRAB_DEPTH,
            GET_GRAB_CARD_OR_255,
            GET_GRAB_DEPTH,
            GET_GRAB_DEPTH,
            GET_GRAB_DEPTH,
            LITERAL,
            ADD,
            GET_GRAB_DEPTH,
            OR,
            NE_BRANCH,
            GET_GRAB_DEPTH,
            GE_BRANCH,
            GET_SELECT_DEPTH,
            NO_OP,
            HANDLE_BUTTON_PRESS,
            GET_GRAB_DEPTH,
            GET_SELECT_DEPTH,
            SET_GRAB_POS,
            GET_GRAB_CARD_OR_255,
            SUB,
            GET_SELECT_POS,
            CAN_GRAB,
            EQ,
            GET_GRAB_POS,
            DROP,
            SET_SELECT_DEPTH,
            GET_GRAB_DEPTH,
            SET_GRAB_DEPTH,
            GET_GRAB_DEPTH,
            GET_GRAB_CARD_OR_255,
            JUMP,
            GRAB,
            DROP,
            SET_SELECT_DEPTH,
            GET_DROP_CARD_OR_255,
            DROP,
            DROP,
            JUMP,
            GRAB,
            CAN_GRAB,
            NOT,
            GET_CARD_NUM,
            DROP,
            GET_GRAB_DEPTH,
            GE,
            CAN_GRAB,
            LITERAL,
            NOT,
            MAX,
            GET_CELL_LEN,
            GET_CARD_SUIT,
            GET_GRAB_CARD_OR_255,
            GET_CARD_NUM,
            FORGET,
            SET_SELECT_POS,
            GET_CELL_LEN,
            GRAB,
            SET_SELECT_POS,
            GET_CELL_LEN,
            NOT,
            GET_SELECT_DROP,
            SET_GRAB_DEPTH,
            SET_SELECT_POS,
            JUMP,
            GRAB,
            JUMP,
            GRAB,
            GET_CELL_LEN,
            DROP,
            FILL_MOVE_TIMER,
            NO_OP,
            GET_DROP_CARD_OR_255,
            FORGET,
            LITERAL,
            GET_GRAB_CARD_OR_255,
            GET_CARD_NUM,
            NO_OP,
            FORGET,
            GET_SELECT_DEPTH,
            JUMP,
            FILL_MOVE_TIMER,
            SET_SELECT_POS,
            FILL_MOVE_TIMER,
            GET_GRAB_CARD_OR_255,
            GET_CARD_SUIT,
            DROP,
            GET_GRAB_DEPTH,
            GET_GRAB_CARD_OR_255,
            SET_SELECT_POS,
            GET_CELL_LEN,
            GE_BRANCH,
            NO_OP,
            GET_SELECT_POS,
            JUMP,
            NO_OP,
            GET_CARD_SUIT,
            IF,
            NO_OP,
            HANDLE_BUTTON_PRESS,
            HANDLE_BUTTON_PRESS,
            CAN_GRAB,
            GET_GRAB_CARD_OR_255,
            ADD,
            CAN_GRAB,
            SET_SELECT_DEPTH,
        ]);
    }

    #[test]
    fn replicate() {
        let failed_input: (u64, u64) = (46, 82);

        let mut game_state = GameState::new([0; 16], Some(logger));

        let seed = unsafe { std::mem::transmute(failed_input) };

        let mut rng = XorShiftRng::from_seed(seed);

        {
            let insructions = generate(&mut rng, 512);

            let pretty_instructions: Vec<_> =
                insructions.iter().map(|&b| PrettyInstruction(b)).collect();
            logger(&format!("insructions: {:?}", &pretty_instructions));

            game_state.interpret(&insructions);
        }

        {
            let insructions = generate(&mut rng, 512);

            let pretty_instructions: Vec<_> =
                insructions.iter().map(|&b| PrettyInstruction(b)).collect();
            logger(&format!("insructions: {:?}", &pretty_instructions));

            game_state.interpret(&insructions);
        }

        {
            let insructions = generate(&mut rng, 512);

            let pretty_instructions: Vec<_> =
                insructions.iter().map(|&b| PrettyInstruction(b)).collect();
            logger(&format!("insructions: {:?}", &pretty_instructions));

            game_state.interpret(&insructions);
        }
    }

    quickcheck!{
        fn does_not_over_or_underflow(pre_seed: (u64, u64)) -> bool {

            let seed = unsafe { std::mem::transmute(pre_seed) };
            let mut game_state = GameState::new(seed, Some(logger));

            let mut rng = XorShiftRng::from_seed(seed);

            for i in 0..16 {
                game_state.grabpos = i;

                let insructions = generate(&mut rng, 512);
                for _ in 0..8 {
                    game_state.interpret(&insructions);
                }
            }

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

    let instructions = generate(&mut rng, 200);
    for &instruction in instructions.iter() {
        output.push_str(&format!("    {},\n", PrettyInstruction(instruction)));
    }

    output.push(']');
    output.push('\n');

    println!("{}", output);
}
