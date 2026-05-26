use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickyPiston {
    pub r#facing: Facing,
    pub extended: bool,
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

impl BlockState for StickyPiston {
    fn to_id(&self) -> i32 {
        if self.r#extended == true && self.r#facing == Facing::Up {
            return 2039;
        }
        if self.r#facing == Facing::West && self.r#extended == true {
            return 2038;
        }
        if self.r#extended == true && self.r#facing == Facing::North {
            return 2035;
        }
        if self.r#extended == true && self.r#facing == Facing::South {
            return 2037;
        }
        if self.r#extended == true && self.r#facing == Facing::East {
            return 2036;
        }
        if self.r#extended == false && self.r#facing == Facing::North {
            return 2041;
        }
        if self.r#extended == true && self.r#facing == Facing::Down {
            return 2040;
        }
        if self.r#facing == Facing::West && self.r#extended == false {
            return 2044;
        }
        if self.r#facing == Facing::East && self.r#extended == false {
            return 2042;
        }
        if self.r#facing == Facing::Down && self.r#extended == false {
            return 2046;
        }
        if self.r#extended == false && self.r#facing == Facing::Up {
            return 2045;
        }
        if self.r#facing == Facing::South && self.r#extended == false {
            return 2043;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2039 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 2038 {
            return Some(StickyPiston {
                r#facing: Facing::West,
                r#extended: true,
            });
        }
        if state_id == 2035 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2037 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2036 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2041 {
            return Some(StickyPiston {
                r#extended: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2040 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 2044 {
            return Some(StickyPiston {
                r#facing: Facing::West,
                r#extended: false,
            });
        }
        if state_id == 2042 {
            return Some(StickyPiston {
                r#facing: Facing::East,
                r#extended: false,
            });
        }
        if state_id == 2046 {
            return Some(StickyPiston {
                r#facing: Facing::Down,
                r#extended: false,
            });
        }
        if state_id == 2045 {
            return Some(StickyPiston {
                r#extended: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 2043 {
            return Some(StickyPiston {
                r#facing: Facing::South,
                r#extended: false,
            });
        }
        return None;
    }
}
