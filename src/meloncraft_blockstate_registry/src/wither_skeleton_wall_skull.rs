use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitherSkeletonWallSkull {
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

impl BlockState for WitherSkeletonWallSkull {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#facing == Facing::East {
            return 10792;
        }
        if self.r#facing == Facing::West && self.r#powered == true {
            return 10789;
        }
        if self.r#facing == Facing::West && self.r#powered == false {
            return 10790;
        }
        if self.r#powered == false && self.r#facing == Facing::North {
            return 10786;
        }
        if self.r#powered == true && self.r#facing == Facing::East {
            return 10791;
        }
        if self.r#facing == Facing::South && self.r#powered == false {
            return 10788;
        }
        if self.r#facing == Facing::North && self.r#powered == true {
            return 10785;
        }
        if self.r#powered == true && self.r#facing == Facing::South {
            return 10787;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10792 {
            return Some(WitherSkeletonWallSkull {
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10789 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10790 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10786 {
            return Some(WitherSkeletonWallSkull {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10791 {
            return Some(WitherSkeletonWallSkull {
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10788 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10785 {
            return Some(WitherSkeletonWallSkull {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10787 {
            return Some(WitherSkeletonWallSkull {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
