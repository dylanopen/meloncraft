use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mycelium {
    pub snowy: bool,
}


impl BlockState for Mycelium {
    fn to_id(&self) -> i32 {
        if self.r#snowy == true { return 8717; }
        if self.r#snowy == false { return 8718; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8717 {
            return Some(Mycelium {
                r#snowy: true,
            });
        }
        if state_id == 8718 {
            return Some(Mycelium {
                r#snowy: false,
            });
        }
        return None;
    }
}

