use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChorusFlower {
    pub age: i32,
}

impl BlockState for ChorusFlower {
    fn to_id(&self) -> i32 {
        if self.r#age == 1 {
            return 14505;
        }
        if self.r#age == 3 {
            return 14507;
        }
        if self.r#age == 5 {
            return 14509;
        }
        if self.r#age == 0 {
            return 14504;
        }
        if self.r#age == 2 {
            return 14506;
        }
        if self.r#age == 4 {
            return 14508;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14505 {
            return Some(ChorusFlower { r#age: 1 });
        }
        if state_id == 14507 {
            return Some(ChorusFlower { r#age: 3 });
        }
        if state_id == 14509 {
            return Some(ChorusFlower { r#age: 5 });
        }
        if state_id == 14504 {
            return Some(ChorusFlower { r#age: 0 });
        }
        if state_id == 14506 {
            return Some(ChorusFlower { r#age: 2 });
        }
        if state_id == 14508 {
            return Some(ChorusFlower { r#age: 4 });
        }
        return None;
    }
}
