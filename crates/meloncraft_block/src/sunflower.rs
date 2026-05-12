use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sunflower {
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for Sunflower {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Lower { return 12714; }
        if self.r#half == Half::Upper { return 12713; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12714 {
            return Some(Sunflower {
                r#half: Half::Lower,
            });
        }
        if state_id == 12713 {
            return Some(Sunflower {
                r#half: Half::Upper,
            });
        }
        return None;
    }
}

