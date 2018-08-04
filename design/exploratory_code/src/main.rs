use std::collections::HashMap;

extern crate project_common;

use project_common::*;

fn main() {
    let instructions = [
        GET_SELECT_POS,
        LITERAL,
        BUTTON_COLUMN,
        NE_BRANCH,
        2,
        HANDLE_BUTTON_PRESS,
        HALT,
        GET_SELECT_DROP,
        IF,
        8,
        CAN_GRAB,
        HALT_UNLESS,
        GET_SELECT_POS,
        SET_GRAB_POS,
        GET_SELECT_DEPTH,
        SET_GRAB_DEPTH,
        GRAB,
        HALT,
        GET_GRAB_CARD_OR_HALT,
        GET_SELECT_POS,
        LITERAL,
        BUTTON_COLUMN,
        LT_BRANCH,
        67, //A
        GET_SELECT_POS,
        LITERAL,
        FLOWER_FOUNDATION,
        GT_BRANCH,
        1,
        HALT,
        GET_SELECT_POS,
        LITERAL,
        START_OF_FOUNDATIONS,
        LT,
        GET_SELECT_POS,
        LITERAL,
        START_OF_TABLEAU,
        GE,
        OR,
        IF,
        27, //B
        GET_GRAB_DEPTH,
        NOT,
        HALT_UNLESS,
        GET_CELL_LEN,
        IF,
        6,
        GET_GRAB_CARD_NUM_OR_255,
        LITERAL,
        1,
        EQ,
        JUMP,
        49, //END
        GET_DROP_CARD_OR_HALT,
        GET_GRAB_CARD_SUIT_OR_255,
        GET_DROP_CARD_SUIT_OR_255,
        EQ,
        GET_GRAB_CARD_NUM_OR_255,
        GET_GRAB_CARD_NUM_OR_255,
        GET_DROP_CARD_NUM_OR_255,
        LITERAL,
        1,
        ADD,
        EQ,
        AND,
        AND,
        JUMP,
        32,                   //END
        GET_DROP_CARD_OR_255, //B
        LITERAL,
        255,
        NE_BRANCH,
        4,
        LITERAL,
        255,
        JUMP,
        23, // END
        GET_GRAB_CARD_SUIT_OR_255,
        GET_DROP_CARD_SUIT_OR_255,
        NE,
        GET_GRAB_CARD_NUM_OR_255,
        GET_GRAB_CARD_NUM_OR_255,
        GET_DROP_CARD_NUM_OR_255,
        LITERAL,
        1,
        SUB,
        EQ,
        AND,
        AND,
        JUMP,
        9,            //END
        GET_CELL_LEN, //A
        LITERAL,
        0,
        EQ,
        GET_GRAB_DEPTH,
        LITERAL,
        0,
        EQ,
        AND,
        HALT_UNLESS, //END
        GET_GRAB_POS,
        GET_GRAB_DEPTH,
        GET_SELECT_POS,
        MOVE_CARDS,
        DROP,
        FILL_MOVE_TIMER,
    ];

    let mut counts = HashMap::with_capacity(instructions.len());

    for &instruction in instructions.iter() {
        let pretty = PrettyInstruction(instruction);

        let count = counts.entry(pretty).or_insert(0);
        *count += 1;
    }

    let mut count_vec: Vec<_> = counts.iter().collect();

    count_vec.sort_unstable_by_key(|(_, &c)| c);

    print!("{:#?}", count_vec);
}
