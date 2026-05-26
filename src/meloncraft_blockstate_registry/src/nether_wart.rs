use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherWart {
    pub age: i32,
}

impl BlockState for NetherWart {
    fn to_id(&self) -> i32 {
        if self.r#age == 0 {
            return 9246;
        }
        if self.r#age == 1 {
            return 9247;
        }
        if self.r#age == 2 {
            return 9248;
        }
        if self.r#age == 3 {
            return 9249;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9246 {
            return Some(NetherWart { r#age: 0 });
        }
        if state_id == 9247 {
            return Some(NetherWart { r#age: 1 });
        }
        if state_id == 9248 {
            return Some(NetherWart { r#age: 2 });
        }
        if state_id == 9249 {
            return Some(NetherWart { r#age: 3 });
        }
        return None;
    }
}
