use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piston {
    pub extended: bool,
    pub r#facing: Facing,
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

impl BlockState for Piston {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#extended == true {
            return 2060;
        }
        if self.r#facing == Facing::Up && self.r#extended == false {
            return 2067;
        }
        if self.r#facing == Facing::Up && self.r#extended == true {
            return 2061;
        }
        if self.r#extended == false && self.r#facing == Facing::North {
            return 2063;
        }
        if self.r#extended == false && self.r#facing == Facing::South {
            return 2065;
        }
        if self.r#extended == false && self.r#facing == Facing::West {
            return 2066;
        }
        if self.r#facing == Facing::East && self.r#extended == false {
            return 2064;
        }
        if self.r#facing == Facing::East && self.r#extended == true {
            return 2058;
        }
        if self.r#extended == true && self.r#facing == Facing::South {
            return 2059;
        }
        if self.r#extended == true && self.r#facing == Facing::North {
            return 2057;
        }
        if self.r#extended == true && self.r#facing == Facing::Down {
            return 2062;
        }
        if self.r#extended == false && self.r#facing == Facing::Down {
            return 2068;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2060 {
            return Some(Piston {
                r#facing: Facing::West,
                r#extended: true,
            });
        }
        if state_id == 2067 {
            return Some(Piston {
                r#facing: Facing::Up,
                r#extended: false,
            });
        }
        if state_id == 2061 {
            return Some(Piston {
                r#facing: Facing::Up,
                r#extended: true,
            });
        }
        if state_id == 2063 {
            return Some(Piston {
                r#extended: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2065 {
            return Some(Piston {
                r#extended: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2066 {
            return Some(Piston {
                r#extended: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2064 {
            return Some(Piston {
                r#facing: Facing::East,
                r#extended: false,
            });
        }
        if state_id == 2058 {
            return Some(Piston {
                r#facing: Facing::East,
                r#extended: true,
            });
        }
        if state_id == 2059 {
            return Some(Piston {
                r#extended: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2057 {
            return Some(Piston {
                r#extended: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2062 {
            return Some(Piston {
                r#extended: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 2068 {
            return Some(Piston {
                r#extended: false,
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}
