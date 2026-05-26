use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndRod {
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

impl BlockState for EndRod {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South {
            return 14436;
        }
        if self.r#facing == Facing::West {
            return 14437;
        }
        if self.r#facing == Facing::Up {
            return 14438;
        }
        if self.r#facing == Facing::Down {
            return 14439;
        }
        if self.r#facing == Facing::East {
            return 14435;
        }
        if self.r#facing == Facing::North {
            return 14434;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14436 {
            return Some(EndRod {
                r#facing: Facing::South,
            });
        }
        if state_id == 14437 {
            return Some(EndRod {
                r#facing: Facing::West,
            });
        }
        if state_id == 14438 {
            return Some(EndRod {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14439 {
            return Some(EndRod {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14435 {
            return Some(EndRod {
                r#facing: Facing::East,
            });
        }
        if state_id == 14434 {
            return Some(EndRod {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
