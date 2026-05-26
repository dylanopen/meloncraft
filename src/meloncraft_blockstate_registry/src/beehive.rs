use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Beehive {
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

impl BlockState for Beehive {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#honey_level == 0 {
            return 21602;
        }
        if self.r#honey_level == 5 && self.r#facing == Facing::East {
            return 21613;
        }
        if self.r#facing == Facing::South && self.r#honey_level == 2 {
            return 21598;
        }
        if self.r#honey_level == 5 && self.r#facing == Facing::North {
            return 21595;
        }
        if self.r#honey_level == 1 && self.r#facing == Facing::South {
            return 21597;
        }
        if self.r#facing == Facing::South && self.r#honey_level == 5 {
            return 21601;
        }
        if self.r#honey_level == 4 && self.r#facing == Facing::West {
            return 21606;
        }
        if self.r#honey_level == 3 && self.r#facing == Facing::South {
            return 21599;
        }
        if self.r#honey_level == 5 && self.r#facing == Facing::West {
            return 21607;
        }
        if self.r#facing == Facing::East && self.r#honey_level == 3 {
            return 21611;
        }
        if self.r#facing == Facing::East && self.r#honey_level == 0 {
            return 21608;
        }
        if self.r#honey_level == 2 && self.r#facing == Facing::North {
            return 21592;
        }
        if self.r#honey_level == 3 && self.r#facing == Facing::West {
            return 21605;
        }
        if self.r#honey_level == 1 && self.r#facing == Facing::East {
            return 21609;
        }
        if self.r#facing == Facing::East && self.r#honey_level == 4 {
            return 21612;
        }
        if self.r#honey_level == 0 && self.r#facing == Facing::North {
            return 21590;
        }
        if self.r#honey_level == 4 && self.r#facing == Facing::South {
            return 21600;
        }
        if self.r#honey_level == 2 && self.r#facing == Facing::West {
            return 21604;
        }
        if self.r#honey_level == 1 && self.r#facing == Facing::North {
            return 21591;
        }
        if self.r#facing == Facing::North && self.r#honey_level == 3 {
            return 21593;
        }
        if self.r#facing == Facing::West && self.r#honey_level == 1 {
            return 21603;
        }
        if self.r#honey_level == 0 && self.r#facing == Facing::South {
            return 21596;
        }
        if self.r#facing == Facing::East && self.r#honey_level == 2 {
            return 21610;
        }
        if self.r#honey_level == 4 && self.r#facing == Facing::North {
            return 21594;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21602 {
            return Some(Beehive {
                r#facing: Facing::West,
                r#honey_level: 0,
            });
        }
        if state_id == 21613 {
            return Some(Beehive {
                r#honey_level: 5,
                r#facing: Facing::East,
            });
        }
        if state_id == 21598 {
            return Some(Beehive {
                r#facing: Facing::South,
                r#honey_level: 2,
            });
        }
        if state_id == 21595 {
            return Some(Beehive {
                r#honey_level: 5,
                r#facing: Facing::North,
            });
        }
        if state_id == 21597 {
            return Some(Beehive {
                r#honey_level: 1,
                r#facing: Facing::South,
            });
        }
        if state_id == 21601 {
            return Some(Beehive {
                r#facing: Facing::South,
                r#honey_level: 5,
            });
        }
        if state_id == 21606 {
            return Some(Beehive {
                r#honey_level: 4,
                r#facing: Facing::West,
            });
        }
        if state_id == 21599 {
            return Some(Beehive {
                r#honey_level: 3,
                r#facing: Facing::South,
            });
        }
        if state_id == 21607 {
            return Some(Beehive {
                r#honey_level: 5,
                r#facing: Facing::West,
            });
        }
        if state_id == 21611 {
            return Some(Beehive {
                r#facing: Facing::East,
                r#honey_level: 3,
            });
        }
        if state_id == 21608 {
            return Some(Beehive {
                r#facing: Facing::East,
                r#honey_level: 0,
            });
        }
        if state_id == 21592 {
            return Some(Beehive {
                r#honey_level: 2,
                r#facing: Facing::North,
            });
        }
        if state_id == 21605 {
            return Some(Beehive {
                r#honey_level: 3,
                r#facing: Facing::West,
            });
        }
        if state_id == 21609 {
            return Some(Beehive {
                r#honey_level: 1,
                r#facing: Facing::East,
            });
        }
        if state_id == 21612 {
            return Some(Beehive {
                r#facing: Facing::East,
                r#honey_level: 4,
            });
        }
        if state_id == 21590 {
            return Some(Beehive {
                r#honey_level: 0,
                r#facing: Facing::North,
            });
        }
        if state_id == 21600 {
            return Some(Beehive {
                r#honey_level: 4,
                r#facing: Facing::South,
            });
        }
        if state_id == 21604 {
            return Some(Beehive {
                r#honey_level: 2,
                r#facing: Facing::West,
            });
        }
        if state_id == 21591 {
            return Some(Beehive {
                r#honey_level: 1,
                r#facing: Facing::North,
            });
        }
        if state_id == 21593 {
            return Some(Beehive {
                r#facing: Facing::North,
                r#honey_level: 3,
            });
        }
        if state_id == 21603 {
            return Some(Beehive {
                r#facing: Facing::West,
                r#honey_level: 1,
            });
        }
        if state_id == 21596 {
            return Some(Beehive {
                r#honey_level: 0,
                r#facing: Facing::South,
            });
        }
        if state_id == 21610 {
            return Some(Beehive {
                r#facing: Facing::East,
                r#honey_level: 2,
            });
        }
        if state_id == 21594 {
            return Some(Beehive {
                r#honey_level: 4,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
