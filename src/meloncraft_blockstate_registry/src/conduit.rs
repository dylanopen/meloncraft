use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conduit {
    pub waterlogged: bool,
}

impl BlockState for Conduit {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true {
            return 15074;
        }
        if self.r#waterlogged == false {
            return 15075;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15074 {
            return Some(Conduit {
                r#waterlogged: true,
            });
        }
        if state_id == 15075 {
            return Some(Conduit {
                r#waterlogged: false,
            });
        }
        return None;
    }
}
