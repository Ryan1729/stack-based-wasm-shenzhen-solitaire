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

#[macro_use]
extern crate lazy_static;

fn generate_instruction_compatible_with_stack_depth<R: Rng>(rng: &mut R, stack_depth: u8) -> u8 {
    lazy_static! {
        static ref FULL_RANGE: Uniform<usize> = Uniform::from(0..INSTRUCTION_POOL.len());
        static ref ZERO_RANGE: Uniform<usize> = Uniform::from(0..ZERO_PARAM_INSTRUCTION_POOL.len());
        static ref ONE_RANGE: Uniform<usize> = Uniform::from(0..ONE_PARAM_INSTRUCTION_POOL.len());
        static ref TWO_RANGE: Uniform<usize> = Uniform::from(0..TWO_PARAM_INSTRUCTION_POOL.len());
        static ref ZERO_TO_TWO: Uniform<usize> = Uniform::from(0..3);
    }

    match stack_depth {
        0 => ZERO_PARAM_INSTRUCTION_POOL[ZERO_RANGE.sample(rng)],
        1 => if rng.gen() {
            ONE_PARAM_INSTRUCTION_POOL[ONE_RANGE.sample(rng)]
        } else {
            ZERO_PARAM_INSTRUCTION_POOL[ZERO_RANGE.sample(rng)]
        },
        2 => match ZERO_TO_TWO.sample(rng) {
            0 => ZERO_PARAM_INSTRUCTION_POOL[ZERO_RANGE.sample(rng)],
            1 => ONE_PARAM_INSTRUCTION_POOL[ONE_RANGE.sample(rng)],
            _ => TWO_PARAM_INSTRUCTION_POOL[TWO_RANGE.sample(rng)],
        },
        _ => INSTRUCTION_POOL[FULL_RANGE.sample(rng)],
    }
}

const GRAB_ZERO_PARAM_INSTRUCTION_POOL: [u8; 3] = [
    GET_GRAB_CARD_OR_255,
    GET_DROP_CARD_OR_255,
    GET_SELECT_DROP,
];

const GRAB_ONE_PARAM_INSTRUCTION_POOL: [u8; 4] = [
    NOT,
    FORGET,
    GET_CARD_NUM,
    GET_CARD_SUIT,
];

const GRAB_TWO_PARAM_INSTRUCTION_POOL: [u8; 14] = [
    ADD, SUB, MUL, DIV, MAX, MIN, AND, OR, EQ, NE, GT, GE, LT, LE,
];

fn generate_A_button_instruction_compatible_with_stack_depth<R: Rng>(rng: &mut R, stack_depth: u8) -> u8 {
    lazy_static! {
        static ref ZERO_RANGE: Uniform<usize> = Uniform::from(0..GRAB_ZERO_PARAM_INSTRUCTION_POOL.len());
        static ref ONE_RANGE: Uniform<usize> = Uniform::from(0..GRAB_ONE_PARAM_INSTRUCTION_POOL.len());
        static ref TWO_RANGE: Uniform<usize> = Uniform::from(0..GRAB_TWO_PARAM_INSTRUCTION_POOL.len());
        static ref ZERO_TO_TWO: Uniform<usize> = Uniform::from(0..3);
    }

    match stack_depth {
        0 => GRAB_ZERO_PARAM_INSTRUCTION_POOL[ZERO_RANGE.sample(rng)],
        1 => if rng.gen() {
            GRAB_ONE_PARAM_INSTRUCTION_POOL[ONE_RANGE.sample(rng)]
        } else {
            GRAB_ZERO_PARAM_INSTRUCTION_POOL[ZERO_RANGE.sample(rng)]
        },
        _ => match ZERO_TO_TWO.sample(rng) {
            0 => GRAB_ZERO_PARAM_INSTRUCTION_POOL[ZERO_RANGE.sample(rng)],
            1 => GRAB_ONE_PARAM_INSTRUCTION_POOL[ONE_RANGE.sample(rng)],
            _ => GRAB_TWO_PARAM_INSTRUCTION_POOL[TWO_RANGE.sample(rng)],
        },
    }
}

fn insert_instruction<R: Rng>(
    rng: &mut R,
    output: &mut Vec<u8>,
    stack_depth: &mut u8,
    restrictions: &mut Vec<u8>,
    instruction: u8,
    count: usize,
) {
    let len = output.len();

    match instruction {
        GET_SELECT_POS | GET_SELECT_DEPTH | GET_GRAB_POS | GET_GRAB_DEPTH | LITERAL | CAN_GRAB
        | GET_GRAB_CARD_OR_255 | GET_DROP_CARD_OR_255 | GET_SELECT_DROP | GET_CELL_LEN => {
            let previous_instruction = output.last().cloned().unwrap_or(NO_OP);
            if previous_instruction != LITERAL {
                *stack_depth += 1;
            }
        }
        IF | FORGET | SET_SELECT_POS | SET_SELECT_DEPTH | SET_GRAB_POS | SET_GRAB_DEPTH => {
            *stack_depth -= 1;
        }
        ADD | SUB | MUL | DIV | MAX | MIN | AND | OR | EQ_BRANCH | NE_BRANCH | GT_BRANCH
        | GE_BRANCH | LT_BRANCH | LE_BRANCH | EQ | NE | GT | GE | LT | LE => {
            *stack_depth -= 2;
        }
        MOVE_CARDS => {
            *stack_depth -= 3;
        }
        _ => {}
    }

    match instruction {
        IF | EQ_BRANCH | NE_BRANCH | GT_BRANCH | GE_BRANCH | LT_BRANCH | LE_BRANCH | JUMP => {
            if len == count - 1 {
                output.push(NO_OP);
                return;
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
                    *stack_depth = min(*stack_depth, restrictions[len]);
                }
                println!("+{:?} {:?}", len, stack_depth);
            }
            output.push(target);

            let absolute_target = min(len + 2 + target as usize, count - 1);

            add_restriction(restrictions, absolute_target, *stack_depth);
        }
        _ => {
            output.push(instruction);
        }
    }
}

fn add_restriction(restrictions: &mut Vec<u8>, index: usize, restriction: u8) {
    if index < restrictions.len() {
        println!(
            "restrictions[{:?}] = {:?}",
            index,
            min(restrictions[index], restriction)
        );
        restrictions[index] = min(restrictions[index], restriction);
    } else {
        println!("restrictions[{:?}] = Kaboom!", index,);
    }
}

fn generate<R: Rng>(rng: &mut R, count: usize) -> Vec<u8> {
    let mut output = Vec::new();

    //TODO:: If count turns out to get very high then this should be a hashmap instead.
    let mut restrictions = vec![255; count];

    let mut stack_depth: u8 = 0;

    while output.len() < count {
        let len = output.len();

        stack_depth = min(stack_depth, restrictions[len]);
        println!("{:?} {:?}", len, stack_depth);
        let instruction = generate_instruction_compatible_with_stack_depth(rng, stack_depth);

        insert_instruction(
            rng,
            &mut output,
            &mut stack_depth,
            &mut restrictions,
            instruction,
            count,
        );
    }

    output
}

type Restriction = u8;
const NON_RESTRICTION: Restriction = 255;

fn generate_containing_instructions<R: Rng>(
    rng: &mut R,
    count: usize,
    required_instructions: &[u8],
    instruction_restriction: Restriction,
) -> Vec<u8> {
    let required_len = required_instructions.len();
    if count <= required_len {
        return required_instructions.to_vec();
    }

    //TODO copy from `generate` into here and then look at `restrictions` to decide when to
    //skipp calling `generate_instruction_compatible_with_stack_depth` and instead start inserting
    //te required instructions
    let mut output = Vec::new();

    //TODO:: If count turns out to get very high then this should be a hashmap instead.
    let mut restrictions = vec![NON_RESTRICTION; count];

    let mut stack_depth: u8 = 0;

    let mut have_not_inserted = true;

    while output.len() < count {
        let len = output.len();

        stack_depth = min(stack_depth, restrictions[len]);

        let last_instruction_was_not_LITERAL = output.last().map(|&inst| inst != LITERAL).unwrap_or(true);

        let should_insert_instructions = have_not_inserted
        && last_instruction_was_not_LITERAL
        && count - len >= required_len
        && {
            let mut result = true;
            for &restriction in restrictions[len..len + required_len].iter() {
                let is_not_within_restrictions = (restriction != NON_RESTRICTION
                    && stack_depth < restriction)
                    || (instruction_restriction != NON_RESTRICTION
                        && stack_depth < instruction_restriction);

                if is_not_within_restrictions {
                    result = false;
                    break;
                }
            }
            println!("checking {}..{} {}", len, len + required_len, result);
            result
        }
        && (count - required_len == len || rng.gen_range(0, count - len) == 0);

        if should_insert_instructions {
            println!("stack_depth {} len {}", stack_depth, len);
            for i in 0..required_len {
                output.push(required_instructions[i]);
                let index = len + i + 1;
                println!(
                    "index {} {} required_instructions[i] {}",
                    index,
                    output.len(),
                    PrettyInstruction(required_instructions[i])
                );
                if index < count {
                    add_restriction(&mut restrictions, index, stack_depth);
                }
            }

            if instruction_restriction != NON_RESTRICTION {
                stack_depth = stack_depth.saturating_sub(instruction_restriction);
            }

            have_not_inserted = false;
        } else {
            let instruction = generate_A_button_instruction_compatible_with_stack_depth(rng, stack_depth);

            insert_instruction(
                rng,
                &mut output,
                &mut stack_depth,
                &mut restrictions,
                instruction,
                count,
            );
        }
    }

    output
}

// We'll define a grab as a series of instructions, that at least some of the time, executes both
// halves of this conditional.
const GRAB_INSTRUCTIONS: [u8; 13] = [
    IF,
    6,
    GET_GRAB_POS,
    GET_GRAB_DEPTH,
    GET_SELECT_POS,
    MOVE_CARDS,
    DROP,
    HALT,
    GET_SELECT_POS,
    SET_GRAB_POS,
    GET_SELECT_DEPTH,
    SET_GRAB_DEPTH,
    GRAB,
];

const GRAB_INSTRUCTIONS_TRUE_RELATIVES_SIDE_INDICIES: [usize; 5] = [8, 9, 10, 11, 12];
const GRAB_INSTRUCTIONS_FALSE_RELATIVES_SIDE_INDICIES: [usize; 6] = [2, 3, 4, 5, 6, 7];

fn generate_grab<R: Rng>(rng: &mut R, count: usize) -> Vec<u8> {
    generate_containing_instructions(rng, count, &GRAB_INSTRUCTIONS, 1)
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

    const TEST_GENERATION_COUNT: usize = 100;

    use quickcheck::{Arbitrary, Gen};

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

        game_state.interpret(&[LITERAL, GET_SELECT_DEPTH]);
    }

    #[test]
    fn replicate() {
        let failed_input: (u64, u64) = (0, 0);

        let mut game_state = GameState::new([0; 16], Some(logger));

        game_state.cells = [
            vec![15, 18, 20, 22, 6],
            vec![20, 4, 12, 11, 27],
            vec![],
            vec![26, 0, 28, 0, 10],
            vec![3, 30, 5, 25, 21],
            vec![],
            vec![19, 23, 20, 10, 24],
            vec![13, 0, 14, 10, 7],
            vec![],
            vec![],
            vec![16, 9, 29, 8, 0],
            vec![],
            vec![17, 10, 2, 1, 20],
            vec![],
            vec![],
            vec![],
        ];

        let seed = unsafe { std::mem::transmute(failed_input) };

        let mut rng = XorShiftRng::from_seed(seed);

        let insructions = generate_grab(&mut rng, TEST_GENERATION_COUNT);

        let possible_base = find_subsequence(&insructions, &GRAB_INSTRUCTIONS);

        let inserted_instruction_base = if let Some(inserted_instruction_base) = possible_base {
            inserted_instruction_base
        } else {
            assert!(possible_base.is_none());
            0
        };

        use std::collections::HashSet;
        let mut visited: HashSet<usize> = HashSet::with_capacity(TEST_GENERATION_COUNT);

        for i in 0..16 {
            game_state.selectpos = i;

            for _ in 0..8 {
                let just_visited =
                    game_state.interpret_and_return_visited_instruction_pointers(&insructions);

                for &v in just_visited.iter() {
                    visited.insert(v);
                }

                game_state.vm.clear();
            }

        }

        let to_absolute = |&i| i + inserted_instruction_base;

        let was_visited = |i| visited.contains(&i);

        let true_side_executed = GRAB_INSTRUCTIONS_TRUE_RELATIVES_SIDE_INDICIES
            .iter()
            .map(to_absolute).all(was_visited);
        let false_side_executed = GRAB_INSTRUCTIONS_FALSE_RELATIVES_SIDE_INDICIES
            .iter()
            .map(to_absolute).all(was_visited);

        assert!(true_side_executed && false_side_executed)
    }

    quickcheck!{
        fn generate_grab_does_not_over_or_underflow(p: ((u64, u64), ArbGameState)) -> bool {
            let  (pre_seed, ArbGameState(mut game_state)) = p;
            let seed = unsafe { std::mem::transmute(pre_seed) };

            let mut rng = XorShiftRng::from_seed(seed);

            let insructions = generate_grab(&mut rng, TEST_GENERATION_COUNT);
            for _ in 0..8 {
                game_state.interpret(&insructions);

                game_state.vm.clear();
            }

            //didn't panic
            true
        }

        #[ignore]
        fn plain_generate_does_not_over_or_underflow(p: ((u64, u64), ArbGameState)) -> bool {
            let  (pre_seed, ArbGameState(mut game_state)) = p;
            let seed = unsafe { std::mem::transmute(pre_seed) };

            let mut rng = XorShiftRng::from_seed(seed);

            let insructions = generate(&mut rng, TEST_GENERATION_COUNT);
            for _ in 0..8 {
                game_state.interpret(&insructions);

                game_state.vm.clear();
            }

            //didn't panic
            true
        }

        fn generate_grab_executes_both_halves(p: ((u64, u64), ArbGameState)) -> bool {
            let (pre_seed, ArbGameState(mut game_state)) = p;

            let seed = unsafe { std::mem::transmute(pre_seed) };

            let mut rng = XorShiftRng::from_seed(seed);

            let insructions = generate_grab(&mut rng, TEST_GENERATION_COUNT);

            let possible_base = find_subsequence(&insructions, &GRAB_INSTRUCTIONS);

            let inserted_instruction_base = if let Some(inserted_instruction_base) = possible_base {
                inserted_instruction_base
            } else {
                return false;
            };

            use std::collections::HashSet;
            let mut visited: HashSet<usize> = HashSet::with_capacity(TEST_GENERATION_COUNT);

            for i in 0..16 {
                game_state.selectpos = i;

                for _ in 0..8 {
                    let just_visited =
                        game_state.interpret_and_return_visited_instruction_pointers(&insructions);

                    for &v in just_visited.iter() {
                        visited.insert(v);
                    }

                    game_state.vm.clear();
                }

            }

            let to_absolute = |&i| i + inserted_instruction_base;

            let was_visited = |i| visited.contains(&i);

            let true_side_executed = GRAB_INSTRUCTIONS_TRUE_RELATIVES_SIDE_INDICIES
                .iter()
                .map(to_absolute).all(was_visited);
            let false_side_executed = GRAB_INSTRUCTIONS_FALSE_RELATIVES_SIDE_INDICIES
                .iter()
                .map(to_absolute).all(was_visited);

            true_side_executed && false_side_executed
        }
    }

    // https://stackoverflow.com/a/35907071/4496839
    // O(mn)
    // if that's too slow theres a KMP impl at
    // https://www.nayuki.io/page/knuth-morris-pratt-string-matching
    fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
    where
        for<'a> &'a [T]: PartialEq,
    {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }

    fn scramble<R: Rng>(rng: &mut R, game_state: &mut GameState) {
        for column in game_state.cells.iter_mut() {
            rng.shuffle(column);
        }

        rng.shuffle(&mut game_state.cells);

        game_state.selectdrop = rng.gen();
        game_state.selectpos = rng.gen();
        game_state.selectdepth = rng.gen();
        game_state.grabpos = rng.gen();
        game_state.grabdepth = rng.gen();
    }

    fn max_out(game_state: &mut GameState) {
        game_state.selectdrop = true;
        game_state.selectpos = u8::max_value();
        game_state.selectdepth = u8::max_value();
        game_state.grabpos = u8::max_value();
        game_state.grabdepth = u8::max_value();
    }

    #[derive(Clone, Debug)]
    pub struct ArbGameState(pub GameState);

    impl Arbitrary for ArbGameState {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let seed: [u8; 16] = g.gen();

            let mut game_state = GameState::new(seed, Some(logger));

            for column in game_state.cells.iter_mut() {
                g.shuffle(column);
            }

            g.shuffle(&mut game_state.cells);

            game_state.selectdrop = g.gen();
            game_state.selectpos = g.gen();
            game_state.selectdepth = g.gen();
            game_state.grabpos = g.gen();
            game_state.grabdepth = g.gen();

            ArbGameState(game_state)
        }

        fn shrink(&self) -> Box<Iterator<Item = ArbGameState>> {
            let game_state = self.0.clone();
            let tuple = (
                game_state.selectdrop,
                game_state.selectpos,
                game_state.selectdepth,
                game_state.grabpos,
                game_state.grabdepth,
            );

            Box::new(tuple.shrink().map(
                move |(selectdrop, selectpos, selectdepth, grabpos, grabdepth)| {
                    ArbGameState(GameState {
                        cells: game_state.cells.clone(),
                        wins: game_state.wins,
                        win_done: game_state.win_done,
                        selectdrop,
                        selectpos,
                        selectdepth,
                        grabpos,
                        grabdepth,
                        movetimer: game_state.movetimer,
                        vm: game_state.vm.clone(),
                        rng: game_state.rng.clone(),
                    })
                },
            ))
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

    let instructions = generate_grab(&mut rng, 200);
    for &instruction in instructions.iter() {
        output.push_str(&format!("    {},\n", PrettyInstruction(instruction)));
    }

    output.push(']');
    output.push('\n');

    println!("{}", output);
}
