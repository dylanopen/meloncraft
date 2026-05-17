use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteGlazedTerracotta {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WhiteGlazedTerracotta {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South { return 14765; }
        if self.r#facing == Facing::East { return 14767; }
        if self.r#facing == Facing::West { return 14766; }
        if self.r#facing == Facing::North { return 14764; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14765 {
            return Some(WhiteGlazedTerracotta {
                r#facing: Facing::South,
            });
        }
        if state_id == 14767 {
            return Some(WhiteGlazedTerracotta {
                r#facing: Facing::East,
            });
        }
        if state_id == 14766 {
            return Some(WhiteGlazedTerracotta {
                r#facing: Facing::West,
            });
        }
        if state_id == 14764 {
            return Some(WhiteGlazedTerracotta {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

