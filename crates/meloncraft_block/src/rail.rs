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
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::EastWest && self.r#waterlogged == true { return 5528; }
        if self.r#waterlogged == false && self.r#shape == Shape::AscendingWest { return 5533; }
        if self.r#shape == Shape::SouthEast && self.r#waterlogged == false { return 5539; }
        if self.r#shape == Shape::AscendingEast && self.r#waterlogged == true { return 5530; }
        if self.r#shape == Shape::NorthWest && self.r#waterlogged == false { return 5543; }
        if self.r#waterlogged == false && self.r#shape == Shape::AscendingSouth { return 5537; }
        if self.r#waterlogged == false && self.r#shape == Shape::NorthEast { return 5545; }
        if self.r#waterlogged == false && self.r#shape == Shape::AscendingNorth { return 5535; }
        if self.r#shape == Shape::SouthWest && self.r#waterlogged == true { return 5540; }
        if self.r#waterlogged == true && self.r#shape == Shape::NorthSouth { return 5526; }
        if self.r#waterlogged == false && self.r#shape == Shape::EastWest { return 5529; }
        if self.r#waterlogged == false && self.r#shape == Shape::AscendingEast { return 5531; }
        if self.r#waterlogged == false && self.r#shape == Shape::SouthWest { return 5541; }
        if self.r#waterlogged == true && self.r#shape == Shape::AscendingNorth { return 5534; }
        if self.r#waterlogged == true && self.r#shape == Shape::NorthWest { return 5542; }
        if self.r#shape == Shape::NorthSouth && self.r#waterlogged == false { return 5527; }
        if self.r#shape == Shape::AscendingSouth && self.r#waterlogged == true { return 5536; }
        if self.r#shape == Shape::NorthEast && self.r#waterlogged == true { return 5544; }
        if self.r#shape == Shape::SouthEast && self.r#waterlogged == true { return 5538; }
        if self.r#waterlogged == true && self.r#shape == Shape::AscendingWest { return 5532; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5528 {
            return Some(Rail {
                r#shape: Shape::EastWest,
                r#waterlogged: true,
            });
        }
        if state_id == 5533 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::AscendingWest,
            });
        }
        if state_id == 5539 {
            return Some(Rail {
                r#shape: Shape::SouthEast,
                r#waterlogged: false,
            });
        }
        if state_id == 5530 {
            return Some(Rail {
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
            });
        }
        if state_id == 5543 {
            return Some(Rail {
                r#shape: Shape::NorthWest,
                r#waterlogged: false,
            });
        }
        if state_id == 5537 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 5545 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::NorthEast,
            });
        }
        if state_id == 5535 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::AscendingNorth,
            });
        }
        if state_id == 5540 {
            return Some(Rail {
                r#shape: Shape::SouthWest,
                r#waterlogged: true,
            });
        }
        if state_id == 5526 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 5529 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 5531 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::AscendingEast,
            });
        }
        if state_id == 5541 {
            return Some(Rail {
                r#waterlogged: false,
                r#shape: Shape::SouthWest,
            });
        }
        if state_id == 5534 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::AscendingNorth,
            });
        }
        if state_id == 5542 {
            return Some(Rail {
                r#waterlogged: true,
                r#shape: Shape::NorthWest,
            });
        }
        if state_id == 5527 {
            return Some(Rail {
                r#shape: Shape::NorthSouth,
                r#waterlogged: false,
            });
        }
        if state_id == 5536 {
            return Some(Rail {
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 5544 {
            return Some(Rail {
                r#shape: Shape::NorthEast,
                r#waterlogged: true,
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
        return None;
    }
}

