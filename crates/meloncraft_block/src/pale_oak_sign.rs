use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakSign {
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for PaleOakSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5361; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 11 { return 5381; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5376; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 5362; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5385; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 5367; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5370; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 5374; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 5378; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 5382; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 5372; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 5358; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 0 { return 5359; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false { return 5365; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 15 { return 5389; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5380; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 13 { return 5384; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5363; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 5383; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 { return 5388; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 5 { return 5368; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 5387; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5377; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 5379; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5360; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 5373; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 6 { return 5371; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 5369; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 5375; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 4 { return 5366; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 14 { return 5386; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5364; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5361 {
            return Some(PaleOakSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5381 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5376 {
            return Some(PaleOakSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5362 {
            return Some(PaleOakSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5385 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5367 {
            return Some(PaleOakSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5370 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5374 {
            return Some(PaleOakSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5378 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5382 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5372 {
            return Some(PaleOakSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5358 {
            return Some(PaleOakSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5359 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5365 {
            return Some(PaleOakSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5389 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5380 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5384 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5363 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5383 {
            return Some(PaleOakSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5388 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5368 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5387 {
            return Some(PaleOakSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5377 {
            return Some(PaleOakSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5379 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 5360 {
            return Some(PaleOakSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5373 {
            return Some(PaleOakSign {
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5371 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5369 {
            return Some(PaleOakSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5375 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5366 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 5386 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 5364 {
            return Some(PaleOakSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

