use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JackOLantern {
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for JackOLantern {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East { return 6825; }
        if self.r#facing == Facing::West { return 6824; }
        if self.r#facing == Facing::North { return 6822; }
        if self.r#facing == Facing::South { return 6823; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6825 {
            return Some(JackOLantern {
                r#facing: Facing::East,
            });
        }
        if state_id == 6824 {
            return Some(JackOLantern {
                r#facing: Facing::West,
            });
        }
        if state_id == 6822 {
            return Some(JackOLantern {
                r#facing: Facing::North,
            });
        }
        if state_id == 6823 {
            return Some(JackOLantern {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

