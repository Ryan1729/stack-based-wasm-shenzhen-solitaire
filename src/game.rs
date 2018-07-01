use bytecode::instructions::*;
use common::*;

use std::cmp::{max, min};

fn update(state: &mut GameState, input: Input) {
    if haswon(state) {
        if state.win_done {
            if input.pressed_this_frame(Button::Start) {
                let wins = state.wins;

                *state = GameState::new();

                state.wins = wins;
            }
        } else {
            state.wins += 1;
            state.win_done = true;
        }

        return;
    }

    if state.movetimer > 0 {
        state.movetimer -= 1;
    }

    if state.movetimer == 0 {
        if automove(state) {
            state.interpret(&[FILL_MOVE_TIMER]);
        } else {
            if input.pressed_this_frame(Button::Left) {
                state.interpret(&[
                    GET_SELECT_POS,
                    LITERAL,
                    0,
                    EQ_BRANCH,
                    15,
                    GET_SELECT_POS,
                    LITERAL,
                    START_OF_TABLEAU,
                    EQ_BRANCH,
                    6,
                    GET_SELECT_POS,
                    LITERAL,
                    1,
                    SUB,
                    JUMP,
                    6,
                    LITERAL,
                    CELLS_MAX_INDEX,
                    JUMP,
                    2,
                    LITERAL,
                    START_OF_TABLEAU - 1,
                    SET_SELECT_POS,
                    ASSERT_EMPTY_STACK,
                    GET_SELECT_DROP,
                    IF,
                    11,
                    GET_CELL_LEN,
                    LITERAL,
                    1,
                    SUB,
                    GET_SELECT_DEPTH,
                    LITERAL,
                    0,
                    MAX,
                    MIN,
                    JUMP,
                    2,
                    LITERAL,
                    0,
                    SET_SELECT_DEPTH,
                ]);
            } else if input.pressed_this_frame(Button::Right) {
                state.interpret(&[
                    GET_SELECT_POS,
                    LITERAL,
                    START_OF_TABLEAU,
                    LITERAL,
                    1,
                    SUB,
                    EQ_BRANCH,
                    15,
                    GET_SELECT_POS,
                    LITERAL,
                    CELLS_MAX_INDEX,
                    GE_BRANCH,
                    6,
                    GET_SELECT_POS,
                    LITERAL,
                    1,
                    ADD,
                    JUMP,
                    6,
                    LITERAL,
                    START_OF_TABLEAU,
                    JUMP,
                    2,
                    LITERAL,
                    0,
                    SET_SELECT_POS,
                    ASSERT_EMPTY_STACK,
                    GET_SELECT_DROP,
                    IF,
                    11,
                    GET_CELL_LEN,
                    LITERAL,
                    1,
                    SUB,
                    GET_SELECT_DEPTH,
                    LITERAL,
                    0,
                    MAX,
                    MIN,
                    JUMP,
                    2,
                    LITERAL,
                    0,
                    SET_SELECT_DEPTH,
                ]);
            } else if input.pressed_this_frame(Button::Up) {
                state.interpret(&[
                    GET_SELECT_POS,
                    LITERAL,
                    BUTTON_COLUMN,
                    EQ_BRANCH,
                    15,
                    GET_CELL_LEN,
                    LITERAL,
                    0,
                    EQ,
                    GET_SELECT_DEPTH,
                    GET_CELL_LEN,
                    LITERAL,
                    1,
                    SUB,
                    GE,
                    GET_SELECT_DROP,
                    OR,
                    OR,
                    JUMP,
                    4,
                    GET_SELECT_DEPTH,
                    LITERAL,
                    2,
                    GE,
                    IF,
                    6,
                    GET_SELECT_DEPTH,
                    LITERAL,
                    1,
                    ADD,
                    SET_SELECT_DEPTH,
                    HALT,
                    GET_SELECT_POS,
                    LITERAL,
                    START_OF_TABLEAU,
                    GET_SELECT_POS,
                    LITERAL,
                    END_OF_FOUNDATIONS,
                    GT_BRANCH,
                    3,
                    ADD,
                    JUMP,
                    1,
                    SUB,
                    SET_SELECT_POS,
                    ASSERT_EMPTY_STACK,
                    LITERAL,
                    0,
                    SET_SELECT_DEPTH,
                ]);
            } else if input.pressed_this_frame(Button::Down) {
                state.interpret(&[
                    GET_SELECT_DEPTH,
                    LITERAL,
                    0,
                    EQ_BRANCH,
                    6,
                    GET_SELECT_DEPTH,
                    LITERAL,
                    1,
                    SUB,
                    SET_SELECT_DEPTH,
                    HALT,
                    GET_SELECT_POS,
                    LITERAL,
                    START_OF_TABLEAU,
                    GET_SELECT_POS,
                    LITERAL,
                    END_OF_FOUNDATIONS,
                    GT_BRANCH,
                    3,
                    ADD,
                    JUMP,
                    1,
                    SUB,
                    SET_SELECT_POS,
                    ASSERT_EMPTY_STACK,
                    GET_CELL_LEN,
                    GET_SELECT_DROP,
                    NOT,
                    AND,
                    IF,
                    13,
                    GET_SELECT_POS,
                    LITERAL,
                    BUTTON_COLUMN,
                    EQ_BRANCH,
                    4,
                    LITERAL,
                    0,
                    JUMP,
                    8,
                    LITERAL,
                    2,
                    JUMP,
                    4,
                    GET_CELL_LEN,
                    LITERAL,
                    1,
                    SUB,
                    SET_SELECT_DEPTH,
                ]);
            } else if input.pressed_this_frame(Button::A) {
                if state.selectpos == BUTTON_COLUMN {
                    if canmovedragons(state, state.selectdepth) {
                        movedragons(state);
                        state.interpret(&[DROP, FILL_MOVE_TIMER]);
                    }
                } else {
                    if state.selectdrop {
                        let monster_expression = if {
                            {
                                let cells = &state.cells;

                                let grabpos = state.grabpos as usize;
                                let grabdepth = state.grabdepth as usize;
                                let droppos = state.selectpos;

                                let grabcard = {
                                    let len = cells[grabpos].len();
                                    if len < grabdepth {
                                        255
                                    } else {
                                        cells[grabpos][len - 1 - grabdepth]
                                    }
                                };

                                if grabcard <= 128 {
                                     {
                                        let droppos = droppos as usize;
                                        if cells[droppos].len() == 0 {
                                            true
                                        } else {
                                            let dropcard = last_unchecked!(cells[droppos]);
                                            getsuit(grabcard) != getsuit(dropcard)
                                                && getcardnum(grabcard) != 0
                                                && getcardnum(grabcard) == getcardnum(dropcard) - 1
                                        }
                                    }
                                } else {
                                    false
                                }
                            }
                        }{ 255 } else {0};

                        state.interpret(&[
                            GET_GRAB_CARD_OR_255,
                            LITERAL,
                            255,
                            NE_BRANCH,
                            1,
                            HALT,
                            GET_SELECT_POS,
                            LITERAL,
                            BUTTON_COLUMN,
                            LT_BRANCH,
                            61, //A
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
                            40, //B
                            GET_GRAB_DEPTH,
                            NOT,
                            IF,
                            1,
                            HALT,
                            GET_CELL_LEN,
                            IF,
                            7,
                            GET_GRAB_CARD_OR_255,
                            GET_CARD_NUM,
                            LITERAL,
                            1,
                            EQ,
                            JUMP,
                            38, //END
                            GET_DROP_CARD_OR_255,
                            LITERAL,
                            255,
                            NE_BRANCH,
                            1,
                            HALT,
                            GET_GRAB_CARD_OR_255,
                            GET_CARD_SUIT,
                            GET_DROP_CARD_OR_255,
                            GET_CARD_SUIT,
                            EQ,
                            GET_GRAB_CARD_OR_255,
                            GET_CARD_NUM,
                            GET_GRAB_CARD_OR_255,
                            GET_CARD_NUM,
                            GET_DROP_CARD_OR_255,
                            GET_CARD_NUM,
                            LITERAL,
                            1,
                            ADD,
                            EQ,
                            AND,
                            AND,
                            JUMP,
                            13, //END
                            LITERAL, //B
                            monster_expression,
                            JUMP,
                            9,
                            GET_CELL_LEN, //A
                            LITERAL,
                            0,
                            EQ,
                            GET_GRAB_DEPTH,
                            LITERAL,
                            0,
                            EQ,
                            AND,
                            IF, //END
                            1,
                            HALT,
                            GET_GRAB_POS,
                            GET_GRAB_DEPTH,
                            GET_SELECT_POS,
                            MOVE_CARDS,
                            DROP,
                            FILL_MOVE_TIMER
                        ]);
                    } else if cangrab(&state.cells, state.selectpos, state.selectdepth) {
                        state.interpret(&[
                            GET_SELECT_POS,
                            SET_GRAB_POS,
                            GET_SELECT_DEPTH,
                            SET_GRAB_DEPTH,
                            GRAB,
                        ]);
                    }
                }
            } else if input.pressed_this_frame(Button::B) {
                state.interpret(&[DROP]);
            }
        }
    }
}

fn getselection(cells: &Cells, pos: u8, depth: u8) -> Vec<u8> {
    let pos = pos as usize;
    let depth = depth as usize;

    let mut output = Vec::with_capacity(depth);
    for i in 1..=depth + 1 {
        let index = cells[pos].len() - (depth + 1) + i - 1;
        output.push(cells[pos][index]);
    }
    return output;
}

fn cangrab(cells: &Cells, pos: u8, depth: u8) -> bool {
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

fn canmovedragons(state: &GameState, suit: u8) -> bool {
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

fn movedragons(state: &mut GameState) {
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

fn haswon(state: &GameState) -> bool {
    for i in START_OF_TABLEAU..=CELLS_MAX_INDEX {
        let i = i as usize;
        if state.cells[i].len() > 0 {
            return false;
        }
    }
    return true;
}

fn automove(state: &mut GameState) -> bool {
    let min_free_card_num = {
        let mut min_foundation_card_num = None;

        for i in START_OF_FOUNDATIONS..START_OF_TABLEAU {
            let i = i as usize;
            let val = if state.cells[i].len() > 0 {
                let card = last_unchecked!(state.cells[i]);
                getcardnum(card)
            } else {
                0
            };
            if min_foundation_card_num.map(|v| val < v).unwrap_or(true) {
                min_foundation_card_num = Some(val);
            }
        }

        min_foundation_card_num.unwrap_or(255).wrapping_add(1)
    };

    for i in 0..=CELLS_MAX_INDEX {
        if (i < BUTTON_COLUMN || i >= START_OF_TABLEAU) && state.cells[i as usize].len() > 0 {
            let card = last_unchecked!(state.cells[i as usize]);
            if card == FLOWER_CARD {
                movecards(state, i, 0, FLOWER_FOUNDATION);
                return true;
            } else if getcardnum(card) == min_free_card_num && card != CARD_BACK {
                let suit = getsuit(card);
                for i2 in START_OF_FOUNDATIONS..START_OF_TABLEAU {
                    if state.cells[i2 as usize].len() > 0 {
                        let card2 = last_unchecked!(state.cells[i2 as usize]);
                        if getsuit(card2) == suit {
                            movecards(state, i, 0, i2);
                            return true;
                        }
                    }
                }
                for i2 in START_OF_FOUNDATIONS..START_OF_TABLEAU {
                    if state.cells[i2 as usize].len() == 0 {
                        movecards(state, i, 0, i2);
                        return true;
                    }
                }
            }
        }
    }

    return false;
}

fn draw(framebuffer: &mut Framebuffer, state: &GameState) {
    framebuffer.clear();
    framebuffer.draw_map();

    framebuffer.print("wins:", 11, 122, 7);
    framebuffer.print(&state.wins.to_string(), 35, 122, 7);

    if canmovedragons(state, 0) {
        framebuffer.spr(56, 48, 16);
    }
    if canmovedragons(state, 1) {
        framebuffer.spr(57, 48, 8);
    }
    if canmovedragons(state, 2) {
        framebuffer.spr(58, 48, 0);
    }

    for i in 0..=CELLS_MAX_INDEX {
        let (posx, posy) = get_card_pos(i);

        drawcell(framebuffer, &state.cells[i as usize], posx, posy);
    }

    let selectpos = state.selectpos;
    if state.selectdrop {
        drawselect(
            framebuffer,
            &state.cells,
            state.grabpos,
            state.grabdepth as i8,
            false,
        );
        if selectpos == BUTTON_COLUMN {
            drawselectbutton(framebuffer, state);
        } else if selectpos <= 8 {
            drawselect(
                framebuffer,
                &state.cells,
                selectpos,
                state.selectdepth as i8,
                true,
            );
        } else {
            drawselect(
                framebuffer,
                &state.cells,
                selectpos,
                -(state.grabdepth as i8) - 1,
                true,
            );
        }
    } else if selectpos == BUTTON_COLUMN {
        drawselectbutton(framebuffer, state);
    } else {
        drawselect(
            framebuffer,
            &state.cells,
            selectpos,
            state.selectdepth as i8,
            false,
        );
    }
}

fn drawcard(framebuffer: &mut Framebuffer, cardnum: u8, posx: u8, posy: u8) {
    if cardnum == CARD_BACK {
        framebuffer.sspr(0, 32, 16, 24, posx, posy);
        return;
    }

    framebuffer.sspr(0, 8, 16, 24, posx, posy);

    let suit = getsuit(cardnum);
    let num = getcardnum(cardnum);

    if num == 0 {
        let sprite = if suit == 1 {
            23
        } else if suit == 2 {
            39
        } else if suit == 3 {
            55
        } else {
            7
        };

        framebuffer.spr(sprite, posx, posy);
        framebuffer.spr_flip_both(sprite, posx + 8, posy + 16);
    } else {
        let (suitcolor, sprite) = if suit == 1 {
            (3, 22)
        } else if suit == 2 {
            (0, 38)
        } else {
            (8, 6)
        };

        framebuffer.print(&num.to_string(), posx + 3, posy + 3, suitcolor);
        framebuffer.spr(sprite, posx + 4, posy + 8);
        framebuffer.print(&num.to_string(), posx + 10, posy + 16, suitcolor);
    }
}

fn drawcell(framebuffer: &mut Framebuffer, cell: &Vec<u8>, posx: u8, posy: u8) {
    for (i, &card) in cell.iter().enumerate() {
        drawcard(framebuffer, card, posx, posy + (i as u8 * 8))
    }
}

fn drawselect(framebuffer: &mut Framebuffer, cells: &Cells, pos: u8, depth: i8, drop: bool) {
    let spritex = if drop { 32 } else { 16 };
    let spritey = 32;

    let (posx, mut posy) = get_card_pos(pos);

    let len = cells[pos as usize].len() as u8;
    if len > 0 {
        posy = (posy as i8 + ((len as i8 - max(depth, -1) - 1) * 8)) as u8;
    }

    framebuffer.sspr(spritex, spritey, 16, 8, posx, posy);

    let truedepth = if depth < 0 { i8::abs(depth) - 1 } else { depth };
    for _ in 0..=truedepth {
        posy = posy + 8;
        framebuffer.sspr(spritex, spritey + 8, 16, 8, posx, posy);
    }
    posy = posy + 8;
    framebuffer.sspr(spritex, spritey + 16, 16, 8, posx, posy);
}

fn drawselectbutton(framebuffer: &mut Framebuffer, state: &GameState) {
    let sprite = if state.selectdrop { 71 } else { 70 };

    framebuffer.spr(sprite, 48, 16 - (state.selectdepth * 8));
}

fn get_card_pos(posx: u8) -> (u8, u8) {
    let (mut posx, posy) = if posx > END_OF_FOUNDATIONS {
        (posx - START_OF_TABLEAU, 24)
    } else {
        (posx, 0)
    };

    posx = if posy == 0 && posx == FLOWER_FOUNDATION {
        56
    } else {
        posx * 16
    };

    (posx, posy)
}

#[inline]
pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
    update(state, input);

    draw(framebuffer, &state);
}
