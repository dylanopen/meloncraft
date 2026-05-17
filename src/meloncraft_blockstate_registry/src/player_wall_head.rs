use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerWallHead {
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

impl BlockState for PlayerWallHead {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#powered == false { return 10872; }
        if self.r#powered == true && self.r#facing == Facing::West { return 10869; }
        if self.r#powered == true && self.r#facing == Facing::North { return 10865; }
        if self.r#facing == Facing::East && self.r#powered == true { return 10871; }
        if self.r#powered == false && self.r#facing == Facing::North { return 10866; }
        if self.r#facing == Facing::West && self.r#powered == false { return 10870; }
        if self.r#facing == Facing::South && self.r#powered == true { return 10867; }
        if self.r#facing == Facing::South && self.r#powered == false { return 10868; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10872 {
            return Some(PlayerWallHead {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10869 {
            return Some(PlayerWallHead {
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10865 {
            return Some(PlayerWallHead {
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10871 {
            return Some(PlayerWallHead {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10866 {
            return Some(PlayerWallHead {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10870 {
            return Some(PlayerWallHead {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10867 {
            return Some(PlayerWallHead {
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10868 {
            return Some(PlayerWallHead {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        return None;
    }
}

