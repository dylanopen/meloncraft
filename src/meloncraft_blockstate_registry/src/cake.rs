use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cake {
    pub bites: i32,
}

impl BlockState for Cake {
    fn to_id(&self) -> i32 {
        if self.r#bites == 1 {
            return 6827;
        }
        if self.r#bites == 0 {
            return 6826;
        }
        if self.r#bites == 2 {
            return 6828;
        }
        if self.r#bites == 3 {
            return 6829;
        }
        if self.r#bites == 4 {
            return 6830;
        }
        if self.r#bites == 5 {
            return 6831;
        }
        if self.r#bites == 6 {
            return 6832;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6827 {
            return Some(Cake { r#bites: 1 });
        }
        if state_id == 6826 {
            return Some(Cake { r#bites: 0 });
        }
        if state_id == 6828 {
            return Some(Cake { r#bites: 2 });
        }
        if state_id == 6829 {
            return Some(Cake { r#bites: 3 });
        }
        if state_id == 6830 {
            return Some(Cake { r#bites: 4 });
        }
        if state_id == 6831 {
            return Some(Cake { r#bites: 5 });
        }
        if state_id == 6832 {
            return Some(Cake { r#bites: 6 });
        }
        return None;
    }
}
