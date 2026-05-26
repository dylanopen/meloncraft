use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrostedIce {
    pub age: i32,
}

impl BlockState for FrostedIce {
    fn to_id(&self) -> i32 {
        if self.r#age == 2 {
            return 14641;
        }
        if self.r#age == 1 {
            return 14640;
        }
        if self.r#age == 3 {
            return 14642;
        }
        if self.r#age == 0 {
            return 14639;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14641 {
            return Some(FrostedIce { r#age: 2 });
        }
        if state_id == 14640 {
            return Some(FrostedIce { r#age: 1 });
        }
        if state_id == 14642 {
            return Some(FrostedIce { r#age: 3 });
        }
        if state_id == 14639 {
            return Some(FrostedIce { r#age: 0 });
        }
        return None;
    }
}
