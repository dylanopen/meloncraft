use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BrownWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North { return 13029; }
        if self.r#facing == Facing::East { return 13032; }
        if self.r#facing == Facing::West { return 13031; }
        if self.r#facing == Facing::South { return 13030; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13029 {
            return Some(BrownWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13032 {
            return Some(BrownWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 13031 {
            return Some(BrownWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 13030 {
            return Some(BrownWallBanner {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

