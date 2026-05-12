use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitcherCrop {
    pub age: i32,
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for PitcherCrop {
    fn to_id(self) -> i32 {
        if block_state.r#age == 0 && block_state.r#half == Half::Lower { return 14598; }
        if block_state.r#age == 1 && block_state.r#half == Half::Lower { return 14600; }
        if block_state.r#half == Half::Upper && block_state.r#age == 0 { return 14597; }
        if block_state.r#age == 3 && block_state.r#half == Half::Lower { return 14604; }
        if block_state.r#half == Half::Upper && block_state.r#age == 4 { return 14605; }
        if block_state.r#age == 1 && block_state.r#half == Half::Upper { return 14599; }
        if block_state.r#half == Half::Upper && block_state.r#age == 2 { return 14601; }
        if block_state.r#half == Half::Lower && block_state.r#age == 4 { return 14606; }
        if block_state.r#half == Half::Upper && block_state.r#age == 3 { return 14603; }
        if block_state.r#half == Half::Lower && block_state.r#age == 2 { return 14602; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14598 {
            return Some(PitcherCrop {
                r#age: 0,
                r#half: Half::Lower,
            });
        }
        if state_id == 14600 {
            return Some(PitcherCrop {
                r#age: 1,
                r#half: Half::Lower,
            });
        }
        if state_id == 14597 {
            return Some(PitcherCrop {
                r#half: Half::Upper,
                r#age: 0,
            });
        }
        if state_id == 14604 {
            return Some(PitcherCrop {
                r#age: 3,
                r#half: Half::Lower,
            });
        }
        if state_id == 14605 {
            return Some(PitcherCrop {
                r#half: Half::Upper,
                r#age: 4,
            });
        }
        if state_id == 14599 {
            return Some(PitcherCrop {
                r#age: 1,
                r#half: Half::Upper,
            });
        }
        if state_id == 14601 {
            return Some(PitcherCrop {
                r#half: Half::Upper,
                r#age: 2,
            });
        }
        if state_id == 14606 {
            return Some(PitcherCrop {
                r#half: Half::Lower,
                r#age: 4,
            });
        }
        if state_id == 14603 {
            return Some(PitcherCrop {
                r#half: Half::Upper,
                r#age: 3,
            });
        }
        if state_id == 14602 {
            return Some(PitcherCrop {
                r#half: Half::Lower,
                r#age: 2,
            });
        }
        return None;
    }
}

