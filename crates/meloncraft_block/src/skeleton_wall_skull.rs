use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonWallSkull {
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

impl BlockState for SkeletonWallSkull {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#powered == false { return 10752; }
        if self.r#powered == false && self.r#facing == Facing::West { return 10750; }
        if self.r#facing == Facing::West && self.r#powered == true { return 10749; }
        if self.r#facing == Facing::East && self.r#powered == true { return 10751; }
        if self.r#powered == false && self.r#facing == Facing::North { return 10746; }
        if self.r#facing == Facing::South && self.r#powered == true { return 10747; }
        if self.r#facing == Facing::South && self.r#powered == false { return 10748; }
        if self.r#powered == true && self.r#facing == Facing::North { return 10745; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10752 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10750 {
            return Some(SkeletonWallSkull {
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10749 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10751 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10746 {
            return Some(SkeletonWallSkull {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10747 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10748 {
            return Some(SkeletonWallSkull {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10745 {
            return Some(SkeletonWallSkull {
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

