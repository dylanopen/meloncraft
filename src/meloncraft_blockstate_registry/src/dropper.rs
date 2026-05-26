use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dropper {
    pub triggered: bool,
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

impl BlockState for Dropper {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::Up && self.r#triggered == false {
            return 11239;
        }
        if self.r#facing == Facing::Down && self.r#triggered == true {
            return 11240;
        }
        if self.r#facing == Facing::Down && self.r#triggered == false {
            return 11241;
        }
        if self.r#facing == Facing::South && self.r#triggered == true {
            return 11234;
        }
        if self.r#facing == Facing::West && self.r#triggered == true {
            return 11236;
        }
        if self.r#triggered == false && self.r#facing == Facing::North {
            return 11231;
        }
        if self.r#facing == Facing::South && self.r#triggered == false {
            return 11235;
        }
        if self.r#facing == Facing::East && self.r#triggered == true {
            return 11232;
        }
        if self.r#facing == Facing::East && self.r#triggered == false {
            return 11233;
        }
        if self.r#facing == Facing::West && self.r#triggered == false {
            return 11237;
        }
        if self.r#facing == Facing::Up && self.r#triggered == true {
            return 11238;
        }
        if self.r#facing == Facing::North && self.r#triggered == true {
            return 11230;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11239 {
            return Some(Dropper {
                r#facing: Facing::Up,
                r#triggered: false,
            });
        }
        if state_id == 11240 {
            return Some(Dropper {
                r#facing: Facing::Down,
                r#triggered: true,
            });
        }
        if state_id == 11241 {
            return Some(Dropper {
                r#facing: Facing::Down,
                r#triggered: false,
            });
        }
        if state_id == 11234 {
            return Some(Dropper {
                r#facing: Facing::South,
                r#triggered: true,
            });
        }
        if state_id == 11236 {
            return Some(Dropper {
                r#facing: Facing::West,
                r#triggered: true,
            });
        }
        if state_id == 11231 {
            return Some(Dropper {
                r#triggered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 11235 {
            return Some(Dropper {
                r#facing: Facing::South,
                r#triggered: false,
            });
        }
        if state_id == 11232 {
            return Some(Dropper {
                r#facing: Facing::East,
                r#triggered: true,
            });
        }
        if state_id == 11233 {
            return Some(Dropper {
                r#facing: Facing::East,
                r#triggered: false,
            });
        }
        if state_id == 11237 {
            return Some(Dropper {
                r#facing: Facing::West,
                r#triggered: false,
            });
        }
        if state_id == 11238 {
            return Some(Dropper {
                r#facing: Facing::Up,
                r#triggered: true,
            });
        }
        if state_id == 11230 {
            return Some(Dropper {
                r#facing: Facing::North,
                r#triggered: true,
            });
        }
        return None;
    }
}
