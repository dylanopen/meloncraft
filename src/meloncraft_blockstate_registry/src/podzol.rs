use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Podzol {
    pub snowy: bool,
}

impl BlockState for Podzol {
    fn to_id(&self) -> i32 {
        if self.r#snowy == false {
            return 13;
        }
        if self.r#snowy == true {
            return 12;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13 {
            return Some(Podzol { r#snowy: false });
        }
        if state_id == 12 {
            return Some(Podzol { r#snowy: true });
        }
        return None;
    }
}
