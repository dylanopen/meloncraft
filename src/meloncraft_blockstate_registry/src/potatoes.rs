use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Potatoes {
    pub age: i32,
}

impl BlockState for Potatoes {
    fn to_id(&self) -> i32 {
        if self.r#age == 3 {
            return 10468;
        }
        if self.r#age == 4 {
            return 10469;
        }
        if self.r#age == 6 {
            return 10471;
        }
        if self.r#age == 5 {
            return 10470;
        }
        if self.r#age == 0 {
            return 10465;
        }
        if self.r#age == 7 {
            return 10472;
        }
        if self.r#age == 1 {
            return 10466;
        }
        if self.r#age == 2 {
            return 10467;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10468 {
            return Some(Potatoes { r#age: 3 });
        }
        if state_id == 10469 {
            return Some(Potatoes { r#age: 4 });
        }
        if state_id == 10471 {
            return Some(Potatoes { r#age: 6 });
        }
        if state_id == 10470 {
            return Some(Potatoes { r#age: 5 });
        }
        if state_id == 10465 {
            return Some(Potatoes { r#age: 0 });
        }
        if state_id == 10472 {
            return Some(Potatoes { r#age: 7 });
        }
        if state_id == 10466 {
            return Some(Potatoes { r#age: 1 });
        }
        if state_id == 10467 {
            return Some(Potatoes { r#age: 2 });
        }
        return None;
    }
}
