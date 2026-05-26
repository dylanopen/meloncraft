use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragonWallHead {
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

impl BlockState for DragonWallHead {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#facing == Facing::West {
            return 10950;
        }
        if self.r#facing == Facing::North && self.r#powered == false {
            return 10946;
        }
        if self.r#facing == Facing::North && self.r#powered == true {
            return 10945;
        }
        if self.r#facing == Facing::South && self.r#powered == false {
            return 10948;
        }
        if self.r#powered == true && self.r#facing == Facing::South {
            return 10947;
        }
        if self.r#powered == true && self.r#facing == Facing::West {
            return 10949;
        }
        if self.r#powered == true && self.r#facing == Facing::East {
            return 10951;
        }
        if self.r#powered == false && self.r#facing == Facing::East {
            return 10952;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10950 {
            return Some(DragonWallHead {
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10946 {
            return Some(DragonWallHead {
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10945 {
            return Some(DragonWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10948 {
            return Some(DragonWallHead {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10947 {
            return Some(DragonWallHead {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10949 {
            return Some(DragonWallHead {
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10951 {
            return Some(DragonWallHead {
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10952 {
            return Some(DragonWallHead {
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
