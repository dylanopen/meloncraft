use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Peony {
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for Peony {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Lower { return 12720; }
        if block_state.r#half == Half::Upper { return 12719; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12720 {
            return Some(Peony {
                r#half: Half::Lower,
            });
        }
        if state_id == 12719 {
            return Some(Peony {
                r#half: Half::Upper,
            });
        }
        return None;
    }
}

