use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkWallBanner {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PinkWallBanner {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North { return 13005; }
        if self.r#facing == Facing::East { return 13008; }
        if self.r#facing == Facing::South { return 13006; }
        if self.r#facing == Facing::West { return 13007; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13005 {
            return Some(PinkWallBanner {
                r#facing: Facing::North,
            });
        }
        if state_id == 13008 {
            return Some(PinkWallBanner {
                r#facing: Facing::East,
            });
        }
        if state_id == 13006 {
            return Some(PinkWallBanner {
                r#facing: Facing::South,
            });
        }
        if state_id == 13007 {
            return Some(PinkWallBanner {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

