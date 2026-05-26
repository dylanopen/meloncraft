use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cactus {
    pub age: i32,
}

impl BlockState for Cactus {
    fn to_id(&self) -> i32 {
        if self.r#age == 10 {
            return 6738;
        }
        if self.r#age == 3 {
            return 6731;
        }
        if self.r#age == 9 {
            return 6737;
        }
        if self.r#age == 6 {
            return 6734;
        }
        if self.r#age == 1 {
            return 6729;
        }
        if self.r#age == 15 {
            return 6743;
        }
        if self.r#age == 5 {
            return 6733;
        }
        if self.r#age == 14 {
            return 6742;
        }
        if self.r#age == 13 {
            return 6741;
        }
        if self.r#age == 0 {
            return 6728;
        }
        if self.r#age == 4 {
            return 6732;
        }
        if self.r#age == 7 {
            return 6735;
        }
        if self.r#age == 8 {
            return 6736;
        }
        if self.r#age == 11 {
            return 6739;
        }
        if self.r#age == 2 {
            return 6730;
        }
        if self.r#age == 12 {
            return 6740;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6738 {
            return Some(Cactus { r#age: 10 });
        }
        if state_id == 6731 {
            return Some(Cactus { r#age: 3 });
        }
        if state_id == 6737 {
            return Some(Cactus { r#age: 9 });
        }
        if state_id == 6734 {
            return Some(Cactus { r#age: 6 });
        }
        if state_id == 6729 {
            return Some(Cactus { r#age: 1 });
        }
        if state_id == 6743 {
            return Some(Cactus { r#age: 15 });
        }
        if state_id == 6733 {
            return Some(Cactus { r#age: 5 });
        }
        if state_id == 6742 {
            return Some(Cactus { r#age: 14 });
        }
        if state_id == 6741 {
            return Some(Cactus { r#age: 13 });
        }
        if state_id == 6728 {
            return Some(Cactus { r#age: 0 });
        }
        if state_id == 6732 {
            return Some(Cactus { r#age: 4 });
        }
        if state_id == 6735 {
            return Some(Cactus { r#age: 7 });
        }
        if state_id == 6736 {
            return Some(Cactus { r#age: 8 });
        }
        if state_id == 6739 {
            return Some(Cactus { r#age: 11 });
        }
        if state_id == 6730 {
            return Some(Cactus { r#age: 2 });
        }
        if state_id == 6740 {
            return Some(Cactus { r#age: 12 });
        }
        return None;
    }
}
