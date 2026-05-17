use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitcherCrop {
    pub r#half: Half,
    pub age: i32,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for PitcherCrop {
    fn to_id(&self) -> i32 {
        if self.r#age == 2 && self.r#half == Half::Lower { return 14602; }
        if self.r#age == 4 && self.r#half == Half::Lower { return 14606; }
        if self.r#half == Half::Upper && self.r#age == 2 { return 14601; }
        if self.r#age == 0 && self.r#half == Half::Upper { return 14597; }
        if self.r#age == 3 && self.r#half == Half::Lower { return 14604; }
        if self.r#half == Half::Upper && self.r#age == 1 { return 14599; }
        if self.r#age == 4 && self.r#half == Half::Upper { return 14605; }
        if self.r#age == 1 && self.r#half == Half::Lower { return 14600; }
        if self.r#half == Half::Upper && self.r#age == 3 { return 14603; }
        if self.r#age == 0 && self.r#half == Half::Lower { return 14598; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14602 {
            return Some(PitcherCrop {
                r#age: 2,
                r#half: Half::Lower,
            });
        }
        if state_id == 14606 {
            return Some(PitcherCrop {
                r#age: 4,
                r#half: Half::Lower,
            });
        }
        if state_id == 14601 {
            return Some(PitcherCrop {
                r#half: Half::Upper,
                r#age: 2,
            });
        }
        if state_id == 14597 {
            return Some(PitcherCrop {
                r#age: 0,
                r#half: Half::Upper,
            });
        }
        if state_id == 14604 {
            return Some(PitcherCrop {
                r#age: 3,
                r#half: Half::Lower,
            });
        }
        if state_id == 14599 {
            return Some(PitcherCrop {
                r#half: Half::Upper,
                r#age: 1,
            });
        }
        if state_id == 14605 {
            return Some(PitcherCrop {
                r#age: 4,
                r#half: Half::Upper,
            });
        }
        if state_id == 14600 {
            return Some(PitcherCrop {
                r#age: 1,
                r#half: Half::Lower,
            });
        }
        if state_id == 14603 {
            return Some(PitcherCrop {
                r#half: Half::Upper,
                r#age: 3,
            });
        }
        if state_id == 14598 {
            return Some(PitcherCrop {
                r#age: 0,
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

