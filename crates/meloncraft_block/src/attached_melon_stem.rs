use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachedMelonStem {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for AttachedMelonStem {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East { return 8140; }
        if self.r#facing == Facing::West { return 8139; }
        if self.r#facing == Facing::South { return 8138; }
        if self.r#facing == Facing::North { return 8137; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8140 {
            return Some(AttachedMelonStem {
                r#facing: Facing::East,
            });
        }
        if state_id == 8139 {
            return Some(AttachedMelonStem {
                r#facing: Facing::West,
            });
        }
        if state_id == 8138 {
            return Some(AttachedMelonStem {
                r#facing: Facing::South,
            });
        }
        if state_id == 8137 {
            return Some(AttachedMelonStem {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

