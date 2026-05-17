use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperWallTorch {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CopperWallTorch {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East { return 6814; }
        if self.r#facing == Facing::North { return 6811; }
        if self.r#facing == Facing::South { return 6812; }
        if self.r#facing == Facing::West { return 6813; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6814 {
            return Some(CopperWallTorch {
                r#facing: Facing::East,
            });
        }
        if state_id == 6811 {
            return Some(CopperWallTorch {
                r#facing: Facing::North,
            });
        }
        if state_id == 6812 {
            return Some(CopperWallTorch {
                r#facing: Facing::South,
            });
        }
        if state_id == 6813 {
            return Some(CopperWallTorch {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

