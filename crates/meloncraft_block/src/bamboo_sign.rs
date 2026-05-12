use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooSign {
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for BambooSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 5431; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 5443; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 8 { return 5438; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false { return 5439; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false { return 5449; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 1 { return 5424; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5434; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 5435; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5444; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 5433; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 5451; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 5430; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 5436; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 13 { return 5448; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 5426; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 14 { return 5450; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 5445; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 0 { return 5422; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5440; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5441; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 { return 5437; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5446; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5428; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5425; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5432; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 { return 5452; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 5429; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5423; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 5442; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 5453; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 12 { return 5447; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == false { return 5427; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5431 {
            return Some(BambooSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5443 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 5438 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 5439 {
            return Some(BambooSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5449 {
            return Some(BambooSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5424 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 5434 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5435 {
            return Some(BambooSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 5444 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5433 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5451 {
            return Some(BambooSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5430 {
            return Some(BambooSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5436 {
            return Some(BambooSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5448 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5426 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5450 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 5445 {
            return Some(BambooSign {
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5422 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 5440 {
            return Some(BambooSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5441 {
            return Some(BambooSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5437 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5446 {
            return Some(BambooSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5428 {
            return Some(BambooSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5425 {
            return Some(BambooSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5432 {
            return Some(BambooSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5452 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5429 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5423 {
            return Some(BambooSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5442 {
            return Some(BambooSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5453 {
            return Some(BambooSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5447 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 12,
            });
        }
        if state_id == 5427 {
            return Some(BambooSign {
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

