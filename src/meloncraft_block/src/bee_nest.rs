use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeeNest {
    pub honey_level: i32,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BeeNest {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#honey_level == 4 { return 21570; }
        if self.r#facing == Facing::North && self.r#honey_level == 5 { return 21571; }
        if self.r#facing == Facing::West && self.r#honey_level == 1 { return 21579; }
        if self.r#facing == Facing::West && self.r#honey_level == 2 { return 21580; }
        if self.r#honey_level == 3 && self.r#facing == Facing::North { return 21569; }
        if self.r#honey_level == 5 && self.r#facing == Facing::South { return 21577; }
        if self.r#facing == Facing::South && self.r#honey_level == 0 { return 21572; }
        if self.r#honey_level == 0 && self.r#facing == Facing::North { return 21566; }
        if self.r#facing == Facing::West && self.r#honey_level == 0 { return 21578; }
        if self.r#facing == Facing::North && self.r#honey_level == 1 { return 21567; }
        if self.r#facing == Facing::North && self.r#honey_level == 2 { return 21568; }
        if self.r#facing == Facing::South && self.r#honey_level == 1 { return 21573; }
        if self.r#facing == Facing::East && self.r#honey_level == 0 { return 21584; }
        if self.r#facing == Facing::East && self.r#honey_level == 3 { return 21587; }
        if self.r#facing == Facing::East && self.r#honey_level == 4 { return 21588; }
        if self.r#facing == Facing::East && self.r#honey_level == 2 { return 21586; }
        if self.r#facing == Facing::South && self.r#honey_level == 2 { return 21574; }
        if self.r#honey_level == 4 && self.r#facing == Facing::South { return 21576; }
        if self.r#facing == Facing::West && self.r#honey_level == 4 { return 21582; }
        if self.r#honey_level == 3 && self.r#facing == Facing::South { return 21575; }
        if self.r#facing == Facing::East && self.r#honey_level == 5 { return 21589; }
        if self.r#honey_level == 1 && self.r#facing == Facing::East { return 21585; }
        if self.r#honey_level == 5 && self.r#facing == Facing::West { return 21583; }
        if self.r#honey_level == 3 && self.r#facing == Facing::West { return 21581; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21570 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 4,
            });
        }
        if state_id == 21571 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 5,
            });
        }
        if state_id == 21579 {
            return Some(BeeNest {
                r#facing: Facing::West,
                r#honey_level: 1,
            });
        }
        if state_id == 21580 {
            return Some(BeeNest {
                r#facing: Facing::West,
                r#honey_level: 2,
            });
        }
        if state_id == 21569 {
            return Some(BeeNest {
                r#honey_level: 3,
                r#facing: Facing::North,
            });
        }
        if state_id == 21577 {
            return Some(BeeNest {
                r#honey_level: 5,
                r#facing: Facing::South,
            });
        }
        if state_id == 21572 {
            return Some(BeeNest {
                r#facing: Facing::South,
                r#honey_level: 0,
            });
        }
        if state_id == 21566 {
            return Some(BeeNest {
                r#honey_level: 0,
                r#facing: Facing::North,
            });
        }
        if state_id == 21578 {
            return Some(BeeNest {
                r#facing: Facing::West,
                r#honey_level: 0,
            });
        }
        if state_id == 21567 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 1,
            });
        }
        if state_id == 21568 {
            return Some(BeeNest {
                r#facing: Facing::North,
                r#honey_level: 2,
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
                r#facing: Facing::East,
                r#honey_level: 0,
            });
        }
        if state_id == 21587 {
            return Some(BeeNest {
                r#facing: Facing::East,
                r#honey_level: 3,
            });
        }
        if state_id == 21588 {
            return Some(BeeNest {
                r#facing: Facing::East,
                r#honey_level: 4,
            });
        }
        if state_id == 21586 {
            return Some(BeeNest {
                r#facing: Facing::East,
                r#honey_level: 2,
            });
        }
        if state_id == 21574 {
            return Some(BeeNest {
                r#facing: Facing::South,
                r#honey_level: 2,
            });
        }
        if state_id == 21576 {
            return Some(BeeNest {
                r#honey_level: 4,
                r#facing: Facing::South,
            });
        }
        if state_id == 21582 {
            return Some(BeeNest {
                r#facing: Facing::West,
                r#honey_level: 4,
            });
        }
        if state_id == 21575 {
            return Some(BeeNest {
                r#honey_level: 3,
                r#facing: Facing::South,
            });
        }
        if state_id == 21589 {
            return Some(BeeNest {
                r#facing: Facing::East,
                r#honey_level: 5,
            });
        }
        if state_id == 21585 {
            return Some(BeeNest {
                r#honey_level: 1,
                r#facing: Facing::East,
            });
        }
        if state_id == 21583 {
            return Some(BeeNest {
                r#honey_level: 5,
                r#facing: Facing::West,
            });
        }
        if state_id == 21581 {
            return Some(BeeNest {
                r#honey_level: 3,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

