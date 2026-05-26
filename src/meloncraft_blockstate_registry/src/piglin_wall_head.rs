use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiglinWallHead {
    pub r#facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PiglinWallHead {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#powered == false {
            return 10986;
        }
        if self.r#facing == Facing::West && self.r#powered == false {
            return 10990;
        }
        if self.r#powered == true && self.r#facing == Facing::South {
            return 10987;
        }
        if self.r#facing == Facing::West && self.r#powered == true {
            return 10989;
        }
        if self.r#powered == true && self.r#facing == Facing::East {
            return 10991;
        }
        if self.r#facing == Facing::East && self.r#powered == false {
            return 10992;
        }
        if self.r#facing == Facing::North && self.r#powered == true {
            return 10985;
        }
        if self.r#powered == false && self.r#facing == Facing::South {
            return 10988;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10986 {
            return Some(PiglinWallHead {
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10990 {
            return Some(PiglinWallHead {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10987 {
            return Some(PiglinWallHead {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10989 {
            return Some(PiglinWallHead {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10991 {
            return Some(PiglinWallHead {
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10992 {
            return Some(PiglinWallHead {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10985 {
            return Some(PiglinWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10988 {
            return Some(PiglinWallHead {
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
