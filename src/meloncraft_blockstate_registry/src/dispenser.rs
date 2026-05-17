use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dispenser {
    pub r#facing: Facing,
    pub triggered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for Dispenser {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::Down && self.r#triggered == false { return 577; }
        if self.r#triggered == true && self.r#facing == Facing::East { return 568; }
        if self.r#facing == Facing::South && self.r#triggered == true { return 570; }
        if self.r#triggered == true && self.r#facing == Facing::West { return 572; }
        if self.r#facing == Facing::South && self.r#triggered == false { return 571; }
        if self.r#facing == Facing::East && self.r#triggered == false { return 569; }
        if self.r#triggered == true && self.r#facing == Facing::North { return 566; }
        if self.r#triggered == true && self.r#facing == Facing::Down { return 576; }
        if self.r#facing == Facing::Up && self.r#triggered == false { return 575; }
        if self.r#triggered == true && self.r#facing == Facing::Up { return 574; }
        if self.r#facing == Facing::West && self.r#triggered == false { return 573; }
        if self.r#triggered == false && self.r#facing == Facing::North { return 567; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 577 {
            return Some(Dispenser {
                r#facing: Facing::Down,
                r#triggered: false,
            });
        }
        if state_id == 568 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 570 {
            return Some(Dispenser {
                r#facing: Facing::South,
                r#triggered: true,
            });
        }
        if state_id == 572 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 571 {
            return Some(Dispenser {
                r#facing: Facing::South,
                r#triggered: false,
            });
        }
        if state_id == 569 {
            return Some(Dispenser {
                r#facing: Facing::East,
                r#triggered: false,
            });
        }
        if state_id == 566 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 576 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 575 {
            return Some(Dispenser {
                r#facing: Facing::Up,
                r#triggered: false,
            });
        }
        if state_id == 574 {
            return Some(Dispenser {
                r#triggered: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 573 {
            return Some(Dispenser {
                r#facing: Facing::West,
                r#triggered: false,
            });
        }
        if state_id == 567 {
            return Some(Dispenser {
                r#triggered: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

