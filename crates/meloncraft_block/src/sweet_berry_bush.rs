use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SweetBerryBush {
    pub age: i32,
}


impl BlockState for SweetBerryBush {
    fn to_id(&self) -> i32 {
        if self.r#age == 2 { return 20741; }
        if self.r#age == 1 { return 20740; }
        if self.r#age == 3 { return 20742; }
        if self.r#age == 0 { return 20739; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20741 {
            return Some(SweetBerryBush {
                r#age: 2,
            });
        }
        if state_id == 20740 {
            return Some(SweetBerryBush {
                r#age: 1,
            });
        }
        if state_id == 20742 {
            return Some(SweetBerryBush {
                r#age: 3,
            });
        }
        if state_id == 20739 {
            return Some(SweetBerryBush {
                r#age: 0,
            });
        }
        return None;
    }
}

