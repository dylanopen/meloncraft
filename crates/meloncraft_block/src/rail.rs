use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rail {
    pub r#shape: Shape,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
}

impl BlockState for Rail {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::NorthSouth { return 5526; }
        if block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == false { return 5531; }
        if block_state.r#shape == Shape::AscendingNorth && block_state.r#waterlogged == true { return 5534; }
        if block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == true { return 5530; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::NorthWest { return 5542; }
        if block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == false { return 5529; }
        if block_state.r#shape == Shape::SouthWest && block_state.r#waterlogged == true { return 5540; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::AscendingNorth { return 5535; }
        if block_state.r#shape == Shape::NorthWest && block_state.r#waterlogged == false { return 5543; }
        if block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == true { return 5536; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::SouthEast { return 5539; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::NorthEast { return 5544; }
        if block_state.r#shape == Shape::SouthEast && block_state.r#waterlogged == true { return 5538; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingWest { return 5532; }
        if block_state.r#shape == Shape::NorthEast && block_state.r#waterlogged == false { return 5545; }
        if block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == false { return 5537; }
        if block_state.r#shape == Shape::AscendingWest && block_state.r#waterlogged == false { return 5533; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::SouthWest { return 5541; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::NorthSouth { return 5527; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::EastWest { return 5528; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5526 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 5531 {
            return Some(Rail {
                r#shape: Shape::AscendingEast,
                r#waterlogged: false,
            });
        }
        if state_id == 5534 {
            return Some(Rail {
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
            });
        }
        if state_id == 5530 {
            return Some(Rail {
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
            });
        }
        if state_id == 5542 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::NorthWest,
            });
        }
        if state_id == 5529 {
            return Some(Rail {
                r#shape: Shape::EastWest,
                r#waterlogged: false,
            });
        }
        if state_id == 5540 {
            return Some(Rail {
                r#shape: Shape::SouthWest,
                r#waterlogged: true,
            });
        }
        if state_id == 5535 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::AscendingNorth,
            });
        }
        if state_id == 5543 {
            return Some(Rail {
                r#shape: Shape::NorthWest,
                r#waterlogged: false,
            });
        }
        if state_id == 5536 {
            return Some(Rail {
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 5539 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::SouthEast,
            });
        }
        if state_id == 5544 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::NorthEast,
            });
        }
        if state_id == 5538 {
            return Some(Rail {
                r#shape: Shape::SouthEast,
                r#waterlogged: true,
            });
        }
        if state_id == 5532 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::AscendingWest,
            });
        }
        if state_id == 5545 {
            return Some(Rail {
                r#shape: Shape::NorthEast,
                r#waterlogged: false,
            });
        }
        if state_id == 5537 {
            return Some(Rail {
                r#shape: Shape::AscendingSouth,
                r#waterlogged: false,
            });
        }
        if state_id == 5533 {
            return Some(Rail {
                r#shape: Shape::AscendingWest,
                r#waterlogged: false,
            });
        }
        if state_id == 5541 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::SouthWest,
            });
        }
        if state_id == 5527 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 5528 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::EastWest,
            });
        }
        return None;
    }
}

