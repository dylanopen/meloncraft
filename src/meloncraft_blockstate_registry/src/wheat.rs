use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wheat {
    pub age: i32,
}

impl BlockState for Wheat {
    fn to_id(&self) -> i32 {
        if self.r#age == 6 {
            return 5116;
        }
        if self.r#age == 4 {
            return 5114;
        }
        if self.r#age == 7 {
            return 5117;
        }
        if self.r#age == 3 {
            return 5113;
        }
        if self.r#age == 2 {
            return 5112;
        }
        if self.r#age == 5 {
            return 5115;
        }
        if self.r#age == 0 {
            return 5110;
        }
        if self.r#age == 1 {
            return 5111;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5116 {
            return Some(Wheat { r#age: 6 });
        }
        if state_id == 5114 {
            return Some(Wheat { r#age: 4 });
        }
        if state_id == 5117 {
            return Some(Wheat { r#age: 7 });
        }
        if state_id == 5113 {
            return Some(Wheat { r#age: 3 });
        }
        if state_id == 5112 {
            return Some(Wheat { r#age: 2 });
        }
        if state_id == 5115 {
            return Some(Wheat { r#age: 5 });
        }
        if state_id == 5110 {
            return Some(Wheat { r#age: 0 });
        }
        if state_id == 5111 {
            return Some(Wheat { r#age: 1 });
        }
        return None;
    }
}
