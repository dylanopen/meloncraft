use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TallGrass {
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for TallGrass {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Lower { return 12722; }
        if self.r#half == Half::Upper { return 12721; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12722 {
            return Some(TallGrass {
                r#half: Half::Lower,
            });
        }
        if state_id == 12721 {
            return Some(TallGrass {
                r#half: Half::Upper,
            });
        }
        return None;
    }
}

