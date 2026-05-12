use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lilac {
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for Lilac {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Upper { return 12715; }
        if self.r#half == Half::Lower { return 12716; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12715 {
            return Some(Lilac {
                r#half: Half::Upper,
            });
        }
        if state_id == 12716 {
            return Some(Lilac {
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

