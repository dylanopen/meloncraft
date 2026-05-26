use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wildflowers {
    pub flower_amount: i32,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Wildflowers {
    fn to_id(&self) -> i32 {
        if self.r#flower_amount == 1 && self.r#facing == Facing::South {
            return 27632;
        }
        if self.r#facing == Facing::South && self.r#flower_amount == 2 {
            return 27633;
        }
        if self.r#facing == Facing::West && self.r#flower_amount == 3 {
            return 27638;
        }
        if self.r#facing == Facing::North && self.r#flower_amount == 4 {
            return 27631;
        }
        if self.r#flower_amount == 2 && self.r#facing == Facing::East {
            return 27641;
        }
        if self.r#flower_amount == 3 && self.r#facing == Facing::East {
            return 27642;
        }
        if self.r#flower_amount == 2 && self.r#facing == Facing::West {
            return 27637;
        }
        if self.r#flower_amount == 1 && self.r#facing == Facing::West {
            return 27636;
        }
        if self.r#facing == Facing::North && self.r#flower_amount == 3 {
            return 27630;
        }
        if self.r#facing == Facing::South && self.r#flower_amount == 3 {
            return 27634;
        }
        if self.r#facing == Facing::South && self.r#flower_amount == 4 {
            return 27635;
        }
        if self.r#flower_amount == 1 && self.r#facing == Facing::East {
            return 27640;
        }
        if self.r#flower_amount == 1 && self.r#facing == Facing::North {
            return 27628;
        }
        if self.r#facing == Facing::West && self.r#flower_amount == 4 {
            return 27639;
        }
        if self.r#facing == Facing::North && self.r#flower_amount == 2 {
            return 27629;
        }
        if self.r#flower_amount == 4 && self.r#facing == Facing::East {
            return 27643;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27632 {
            return Some(Wildflowers {
                r#flower_amount: 1,
                r#facing: Facing::South,
            });
        }
        if state_id == 27633 {
            return Some(Wildflowers {
                r#facing: Facing::South,
                r#flower_amount: 2,
            });
        }
        if state_id == 27638 {
            return Some(Wildflowers {
                r#facing: Facing::West,
                r#flower_amount: 3,
            });
        }
        if state_id == 27631 {
            return Some(Wildflowers {
                r#facing: Facing::North,
                r#flower_amount: 4,
            });
        }
        if state_id == 27641 {
            return Some(Wildflowers {
                r#flower_amount: 2,
                r#facing: Facing::East,
            });
        }
        if state_id == 27642 {
            return Some(Wildflowers {
                r#flower_amount: 3,
                r#facing: Facing::East,
            });
        }
        if state_id == 27637 {
            return Some(Wildflowers {
                r#flower_amount: 2,
                r#facing: Facing::West,
            });
        }
        if state_id == 27636 {
            return Some(Wildflowers {
                r#flower_amount: 1,
                r#facing: Facing::West,
            });
        }
        if state_id == 27630 {
            return Some(Wildflowers {
                r#facing: Facing::North,
                r#flower_amount: 3,
            });
        }
        if state_id == 27634 {
            return Some(Wildflowers {
                r#facing: Facing::South,
                r#flower_amount: 3,
            });
        }
        if state_id == 27635 {
            return Some(Wildflowers {
                r#facing: Facing::South,
                r#flower_amount: 4,
            });
        }
        if state_id == 27640 {
            return Some(Wildflowers {
                r#flower_amount: 1,
                r#facing: Facing::East,
            });
        }
        if state_id == 27628 {
            return Some(Wildflowers {
                r#flower_amount: 1,
                r#facing: Facing::North,
            });
        }
        if state_id == 27639 {
            return Some(Wildflowers {
                r#facing: Facing::West,
                r#flower_amount: 4,
            });
        }
        if state_id == 27629 {
            return Some(Wildflowers {
                r#facing: Facing::North,
                r#flower_amount: 2,
            });
        }
        if state_id == 27643 {
            return Some(Wildflowers {
                r#flower_amount: 4,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
