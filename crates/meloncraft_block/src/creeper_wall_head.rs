use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreeperWallHead {
    pub powered: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CreeperWallHead {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#facing == Facing::North { return 10906; }
        if self.r#powered == true && self.r#facing == Facing::West { return 10909; }
        if self.r#powered == false && self.r#facing == Facing::South { return 10908; }
        if self.r#powered == false && self.r#facing == Facing::East { return 10912; }
        if self.r#facing == Facing::North && self.r#powered == true { return 10905; }
        if self.r#powered == true && self.r#facing == Facing::South { return 10907; }
        if self.r#facing == Facing::East && self.r#powered == true { return 10911; }
        if self.r#facing == Facing::West && self.r#powered == false { return 10910; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10906 {
            return Some(CreeperWallHead {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10909 {
            return Some(CreeperWallHead {
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10908 {
            return Some(CreeperWallHead {
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10912 {
            return Some(CreeperWallHead {
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10905 {
            return Some(CreeperWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10907 {
            return Some(CreeperWallHead {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10911 {
            return Some(CreeperWallHead {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10910 {
            return Some(CreeperWallHead {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        return None;
    }
}

