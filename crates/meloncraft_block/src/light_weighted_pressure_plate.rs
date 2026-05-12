use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightWeightedPressurePlate {
    pub power: i32,
}


impl BlockState for LightWeightedPressurePlate {
    fn to_id(self) -> i32 {
        if block_state.r#power == 2 { return 11031; }
        if block_state.r#power == 1 { return 11030; }
        if block_state.r#power == 3 { return 11032; }
        if block_state.r#power == 11 { return 11040; }
        if block_state.r#power == 9 { return 11038; }
        if block_state.r#power == 12 { return 11041; }
        if block_state.r#power == 8 { return 11037; }
        if block_state.r#power == 0 { return 11029; }
        if block_state.r#power == 5 { return 11034; }
        if block_state.r#power == 10 { return 11039; }
        if block_state.r#power == 14 { return 11043; }
        if block_state.r#power == 4 { return 11033; }
        if block_state.r#power == 6 { return 11035; }
        if block_state.r#power == 7 { return 11036; }
        if block_state.r#power == 13 { return 11042; }
        if block_state.r#power == 15 { return 11044; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11031 {
            return Some(LightWeightedPressurePlate {
                r#power: 2,
            });
        }
        if state_id == 11030 {
            return Some(LightWeightedPressurePlate {
                r#power: 1,
            });
        }
        if state_id == 11032 {
            return Some(LightWeightedPressurePlate {
                r#power: 3,
            });
        }
        if state_id == 11040 {
            return Some(LightWeightedPressurePlate {
                r#power: 11,
            });
        }
        if state_id == 11038 {
            return Some(LightWeightedPressurePlate {
                r#power: 9,
            });
        }
        if state_id == 11041 {
            return Some(LightWeightedPressurePlate {
                r#power: 12,
            });
        }
        if state_id == 11037 {
            return Some(LightWeightedPressurePlate {
                r#power: 8,
            });
        }
        if state_id == 11029 {
            return Some(LightWeightedPressurePlate {
                r#power: 0,
            });
        }
        if state_id == 11034 {
            return Some(LightWeightedPressurePlate {
                r#power: 5,
            });
        }
        if state_id == 11039 {
            return Some(LightWeightedPressurePlate {
                r#power: 10,
            });
        }
        if state_id == 11043 {
            return Some(LightWeightedPressurePlate {
                r#power: 14,
            });
        }
        if state_id == 11033 {
            return Some(LightWeightedPressurePlate {
                r#power: 4,
            });
        }
        if state_id == 11035 {
            return Some(LightWeightedPressurePlate {
                r#power: 6,
            });
        }
        if state_id == 11036 {
            return Some(LightWeightedPressurePlate {
                r#power: 7,
            });
        }
        if state_id == 11042 {
            return Some(LightWeightedPressurePlate {
                r#power: 13,
            });
        }
        if state_id == 11044 {
            return Some(LightWeightedPressurePlate {
                r#power: 15,
            });
        }
        return None;
    }
}

