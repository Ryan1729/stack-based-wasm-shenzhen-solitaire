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
        62, //A
        GET_SELECT_POS,
        LITERAL,
        FLOWER_FOUNDATION,
        GT,
        HALT_UNLESS,
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
        10,
        EQ,
        JUMP,
        41, //END
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
        24,                   //END
        GET_DROP_CARD_OR_255, //B
        LITERAL,
        255,
        EQ_BRANCH,
        20, //PAST END CHECK
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
        5,            //END
        GET_CELL_LEN, //A
        NOT,
        GET_GRAB_DEPTH,
        NOT,
        AND,
        HALT_UNLESS,  //END
        GET_GRAB_POS, //PAST END CHECK
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
