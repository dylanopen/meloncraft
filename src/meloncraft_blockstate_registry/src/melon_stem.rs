use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MelonStem {
    pub age: i32,
}

impl BlockState for MelonStem {
    fn to_id(&self) -> i32 {
        if self.r#age == 5 {
            return 8154;
        }
        if self.r#age == 1 {
            return 8150;
        }
        if self.r#age == 6 {
            return 8155;
        }
        if self.r#age == 4 {
            return 8153;
        }
        if self.r#age == 3 {
            return 8152;
        }
        if self.r#age == 0 {
            return 8149;
        }
        if self.r#age == 2 {
            return 8151;
        }
        if self.r#age == 7 {
            return 8156;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8154 {
            return Some(MelonStem { r#age: 5 });
        }
        if state_id == 8150 {
            return Some(MelonStem { r#age: 1 });
        }
        if state_id == 8155 {
            return Some(MelonStem { r#age: 6 });
        }
        if state_id == 8153 {
            return Some(MelonStem { r#age: 4 });
        }
        if state_id == 8152 {
            return Some(MelonStem { r#age: 3 });
        }
        if state_id == 8149 {
            return Some(MelonStem { r#age: 0 });
        }
        if state_id == 8151 {
            return Some(MelonStem { r#age: 2 });
        }
        if state_id == 8156 {
            return Some(MelonStem { r#age: 7 });
        }
        return None;
    }
}
