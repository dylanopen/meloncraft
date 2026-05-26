use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SugarCane {
    pub age: i32,
}

impl BlockState for SugarCane {
    fn to_id(&self) -> i32 {
        if self.r#age == 6 {
            return 6752;
        }
        if self.r#age == 4 {
            return 6750;
        }
        if self.r#age == 0 {
            return 6746;
        }
        if self.r#age == 5 {
            return 6751;
        }
        if self.r#age == 9 {
            return 6755;
        }
        if self.r#age == 10 {
            return 6756;
        }
        if self.r#age == 7 {
            return 6753;
        }
        if self.r#age == 12 {
            return 6758;
        }
        if self.r#age == 2 {
            return 6748;
        }
        if self.r#age == 11 {
            return 6757;
        }
        if self.r#age == 3 {
            return 6749;
        }
        if self.r#age == 1 {
            return 6747;
        }
        if self.r#age == 14 {
            return 6760;
        }
        if self.r#age == 15 {
            return 6761;
        }
        if self.r#age == 8 {
            return 6754;
        }
        if self.r#age == 13 {
            return 6759;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6752 {
            return Some(SugarCane { r#age: 6 });
        }
        if state_id == 6750 {
            return Some(SugarCane { r#age: 4 });
        }
        if state_id == 6746 {
            return Some(SugarCane { r#age: 0 });
        }
        if state_id == 6751 {
            return Some(SugarCane { r#age: 5 });
        }
        if state_id == 6755 {
            return Some(SugarCane { r#age: 9 });
        }
        if state_id == 6756 {
            return Some(SugarCane { r#age: 10 });
        }
        if state_id == 6753 {
            return Some(SugarCane { r#age: 7 });
        }
        if state_id == 6758 {
            return Some(SugarCane { r#age: 12 });
        }
        if state_id == 6748 {
            return Some(SugarCane { r#age: 2 });
        }
        if state_id == 6757 {
            return Some(SugarCane { r#age: 11 });
        }
        if state_id == 6749 {
            return Some(SugarCane { r#age: 3 });
        }
        if state_id == 6747 {
            return Some(SugarCane { r#age: 1 });
        }
        if state_id == 6760 {
            return Some(SugarCane { r#age: 14 });
        }
        if state_id == 6761 {
            return Some(SugarCane { r#age: 15 });
        }
        if state_id == 6754 {
            return Some(SugarCane { r#age: 8 });
        }
        if state_id == 6759 {
            return Some(SugarCane { r#age: 13 });
        }
        return None;
    }
}
