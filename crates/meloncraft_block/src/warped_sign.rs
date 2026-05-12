use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedSign {
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for WarpedSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#rotation == 4 { return 21480; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 21494; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 21486; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 { return 21502; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 21493; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 21481; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 21472; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 21500; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 21490; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 21482; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false { return 21479; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 21491; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false { return 21499; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 { return 21487; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 21497; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 21477; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 21495; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 21496; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 13 { return 21498; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 21476; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == true { return 21484; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 21503; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 0 { return 21473; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 21489; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 21483; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 21475; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 1 { return 21474; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 21485; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 21501; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 21478; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 21488; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 21492; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21480 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 21494 {
            return Some(WarpedSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 21486 {
            return Some(WarpedSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 21502 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 21493 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 21481 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 21472 {
            return Some(WarpedSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 21500 {
            return Some(WarpedSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 21490 {
            return Some(WarpedSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 21482 {
            return Some(WarpedSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 21479 {
            return Some(WarpedSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 21491 {
            return Some(WarpedSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 21499 {
            return Some(WarpedSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 21487 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 21497 {
            return Some(WarpedSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 21477 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 21495 {
            return Some(WarpedSign {
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 21496 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 21498 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 21476 {
            return Some(WarpedSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 21484 {
            return Some(WarpedSign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 21503 {
            return Some(WarpedSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 21473 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 21489 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 21483 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 21475 {
            return Some(WarpedSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 21474 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 21485 {
            return Some(WarpedSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 21501 {
            return Some(WarpedSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 21478 {
            return Some(WarpedSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 21488 {
            return Some(WarpedSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 21492 {
            return Some(WarpedSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

