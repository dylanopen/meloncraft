use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeeNest {
    pub r#facing: Facing,
    pub honey_level: i32,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BeeNest {
    fn to_id(self) -> i32 {
        if block_state.r#honey_level == 3 && block_state.r#facing == Facing::South { return 21575; }
        if block_state.r#facing == Facing::North && block_state.r#honey_level == 1 { return 21567; }
        if block_state.r#facing == Facing::North && block_state.r#honey_level == 3 { return 21569; }
        if block_state.r#facing == Facing::South && block_state.r#honey_level == 0 { return 21572; }
        if block_state.r#honey_level == 5 && block_state.r#facing == Facing::South { return 21577; }
        if block_state.r#honey_level == 0 && block_state.r#facing == Facing::West { return 21578; }
        if block_state.r#facing == Facing::South && block_state.r#honey_level == 4 { return 21576; }
        if block_state.r#honey_level == 4 && block_state.r#facing == Facing::West { return 21582; }
        if block_state.r#facing == Facing::East && block_state.r#honey_level == 2 { return 21586; }
        if block_state.r#honey_level == 4 && block_state.r#facing == Facing::East { return 21588; }
        if block_state.r#honey_level == 2 && block_state.r#facing == Facing::South { return 21574; }
        if block_state.r#facing == Facing::East && block_state.r#honey_level == 3 { return 21587; }
        if block_state.r#honey_level == 0 && block_state.r#facing == Facing::North { return 21566; }
        if block_state.r#honey_level == 2 && block_state.r#facing == Facing::West { return 21580; }
        if block_state.r#honey_level == 1 && block_state.r#facing == Facing::East { return 21585; }
        if block_state.r#facing == Facing::East && block_state.r#honey_level == 5 { return 21589; }
        if block_state.r#honey_level == 1 && block_state.r#facing == Facing::West { return 21579; }
        if block_state.r#facing == Facing::North && block_state.r#honey_level == 5 { return 21571; }
        if block_state.r#facing == Facing::North && block_state.r#honey_level == 4 { return 21570; }
        if block_state.r#facing == Facing::West && block_state.r#honey_level == 3 { return 21581; }
        if block_state.r#honey_level == 5 && block_state.r#facing == Facing::West { return 21583; }
        if block_state.r#honey_level == 2 && block_state.r#facing == Facing::North { return 21568; }
        if block_state.r#facing == Facing::South && block_state.r#honey_level == 1 { return 21573; }
        if block_state.r#honey_level == 0 && block_state.r#facing == Facing::East { return 21584; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21575 {
            return Some(BeeNest {
                r#honey_level: 3,
                r#facing: Facing::South,
            });
        }
        if state_id == 21567 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 1,
            });
        }
        if state_id == 21569 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 3,
            });
        }
        if state_id == 21572 {
            return Some(BeeNest {
                r#facing: Facing::South,
                r#honey_level: 0,
            });
        }
        if state_id == 21577 {
            return Some(BeeNest {
                r#honey_level: 5,
                r#facing: Facing::South,
            });
        }
        if state_id == 21578 {
            return Some(BeeNest {
                r#honey_level: 0,
                r#facing: Facing::West,
            });
        }
        if state_id == 21576 {
            return Some(BeeNest {
                r#facing: Facing::South,
                r#honey_level: 4,
            });
        }
        if state_id == 21582 {
            return Some(BeeNest {
                r#honey_level: 4,
                r#facing: Facing::West,
            });
        }
        if state_id == 21586 {
            return Some(BeeNest {
                r#facing: Facing::East,
                r#honey_level: 2,
            });
        }
        if state_id == 21588 {
            return Some(BeeNest {
                r#honey_level: 4,
                r#facing: Facing::East,
            });
        }
        if state_id == 21574 {
            return Some(BeeNest {
                r#honey_level: 2,
                r#facing: Facing::South,
            });
        }
        if state_id == 21587 {
            return Some(BeeNest {
                r#facing: Facing::East,
                r#honey_level: 3,
            });
        }
        if state_id == 21566 {
            return Some(BeeNest {
                r#honey_level: 0,
                r#facing: Facing::North,
            });
        }
        if state_id == 21580 {
            return Some(BeeNest {
                r#honey_level: 2,
                r#facing: Facing::West,
            });
        }
        if state_id == 21585 {
            return Some(BeeNest {
                r#honey_level: 1,
                r#facing: Facing::East,
            });
        }
        if state_id == 21589 {
            return Some(BeeNest {
                r#facing: Facing::East,
                r#honey_level: 5,
            });
        }
        if state_id == 21579 {
            return Some(BeeNest {
                r#honey_level: 1,
                r#facing: Facing::West,
            });
        }
        if state_id == 21571 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 5,
            });
        }
        if state_id == 21570 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 4,
            });
        }
        if state_id == 21581 {
            return Some(BeeNest {
                r#facing: Facing::West,
                r#honey_level: 3,
            });
        }
        if state_id == 21583 {
            return Some(BeeNest {
                r#honey_level: 5,
                r#facing: Facing::West,
            });
        }
        if state_id == 21568 {
            return Some(BeeNest {
                r#honey_level: 2,
                r#facing: Facing::North,
            });
        }
        if state_id == 21573 {
            return Some(BeeNest {
                r#facing: Facing::South,
                r#honey_level: 1,
            });
        }
        if state_id == 21584 {
            return Some(BeeNest {
                r#honey_level: 0,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

