use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitcherPlant {
    pub r#half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for PitcherPlant {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Lower {
            return 14608;
        }
        if self.r#half == Half::Upper {
            return 14607;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14608 {
            return Some(PitcherPlant {
                r#half: Half::Lower,
            });
        }
        if state_id == 14607 {
            return Some(PitcherPlant {
                r#half: Half::Upper,
            });
        }
        return None;
    }
}
