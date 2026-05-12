use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for OrangeWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East { return 12988; }
        if self.r#facing == Facing::West { return 12987; }
        if self.r#facing == Facing::South { return 12986; }
        if self.r#facing == Facing::North { return 12985; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12988 {
            return Some(OrangeWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 12987 {
            return Some(OrangeWallBanner {
                r#facing: Facing::West,
            });
        }
        if state_id == 12986 {
            return Some(OrangeWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 12985 {
            return Some(OrangeWallBanner {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

