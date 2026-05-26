use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrassBlock {
    pub snowy: bool,
}

impl BlockState for GrassBlock {
    fn to_id(&self) -> i32 {
        if self.r#snowy == false {
            return 9;
        }
        if self.r#snowy == true {
            return 8;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9 {
            return Some(GrassBlock { r#snowy: false });
        }
        if state_id == 8 {
            return Some(GrassBlock { r#snowy: true });
        }
        return None;
    }
}
