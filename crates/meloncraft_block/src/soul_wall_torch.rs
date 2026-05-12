use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulWallTorch {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for SoulWallTorch {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South { return 6807; }
        if self.r#facing == Facing::North { return 6806; }
        if self.r#facing == Facing::West { return 6808; }
        if self.r#facing == Facing::East { return 6809; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6807 {
            return Some(SoulWallTorch {
                r#facing: Facing::South,
            });
        }
        if state_id == 6806 {
            return Some(SoulWallTorch {
                r#facing: Facing::North,
            });
        }
        if state_id == 6808 {
            return Some(SoulWallTorch {
                r#facing: Facing::West,
            });
        }
        if state_id == 6809 {
            return Some(SoulWallTorch {
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

