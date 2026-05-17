use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for RedGlazedTerracotta {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South { return 14821; }
        if self.r#facing == Facing::North { return 14820; }
        if self.r#facing == Facing::East { return 14823; }
        if self.r#facing == Facing::West { return 14822; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14821 {
            return Some(RedGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14820 {
            return Some(RedGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        if state_id == 14823 {
            return Some(RedGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14822 {
            return Some(RedGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

