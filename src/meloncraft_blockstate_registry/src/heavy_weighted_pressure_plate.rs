use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyWeightedPressurePlate {
    pub power: i32,
}

impl BlockState for HeavyWeightedPressurePlate {
    fn to_id(&self) -> i32 {
        if self.r#power == 7 {
            return 11052;
        }
        if self.r#power == 4 {
            return 11049;
        }
        if self.r#power == 12 {
            return 11057;
        }
        if self.r#power == 0 {
            return 11045;
        }
        if self.r#power == 6 {
            return 11051;
        }
        if self.r#power == 9 {
            return 11054;
        }
        if self.r#power == 3 {
            return 11048;
        }
        if self.r#power == 14 {
            return 11059;
        }
        if self.r#power == 2 {
            return 11047;
        }
        if self.r#power == 5 {
            return 11050;
        }
        if self.r#power == 13 {
            return 11058;
        }
        if self.r#power == 15 {
            return 11060;
        }
        if self.r#power == 1 {
            return 11046;
        }
        if self.r#power == 10 {
            return 11055;
        }
        if self.r#power == 8 {
            return 11053;
        }
        if self.r#power == 11 {
            return 11056;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11052 {
            return Some(HeavyWeightedPressurePlate { r#power: 7 });
        }
        if state_id == 11049 {
            return Some(HeavyWeightedPressurePlate { r#power: 4 });
        }
        if state_id == 11057 {
            return Some(HeavyWeightedPressurePlate { r#power: 12 });
        }
        if state_id == 11045 {
            return Some(HeavyWeightedPressurePlate { r#power: 0 });
        }
        if state_id == 11051 {
            return Some(HeavyWeightedPressurePlate { r#power: 6 });
        }
        if state_id == 11054 {
            return Some(HeavyWeightedPressurePlate { r#power: 9 });
        }
        if state_id == 11048 {
            return Some(HeavyWeightedPressurePlate { r#power: 3 });
        }
        if state_id == 11059 {
            return Some(HeavyWeightedPressurePlate { r#power: 14 });
        }
        if state_id == 11047 {
            return Some(HeavyWeightedPressurePlate { r#power: 2 });
        }
        if state_id == 11050 {
            return Some(HeavyWeightedPressurePlate { r#power: 5 });
        }
        if state_id == 11058 {
            return Some(HeavyWeightedPressurePlate { r#power: 13 });
        }
        if state_id == 11060 {
            return Some(HeavyWeightedPressurePlate { r#power: 15 });
        }
        if state_id == 11046 {
            return Some(HeavyWeightedPressurePlate { r#power: 1 });
        }
        if state_id == 11055 {
            return Some(HeavyWeightedPressurePlate { r#power: 10 });
        }
        if state_id == 11053 {
            return Some(HeavyWeightedPressurePlate { r#power: 8 });
        }
        if state_id == 11056 {
            return Some(HeavyWeightedPressurePlate { r#power: 11 });
        }
        return None;
    }
}
