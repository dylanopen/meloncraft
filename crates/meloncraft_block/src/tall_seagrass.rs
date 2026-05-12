use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TallSeagrass {
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for TallSeagrass {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Upper { return 2055; }
        if block_state.r#half == Half::Lower { return 2056; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2055 {
            return Some(TallSeagrass {
                r#half: Half::Upper,
            });
        }
        if state_id == 2056 {
            return Some(TallSeagrass {
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

