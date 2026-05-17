use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyCore {
    pub waterlogged: bool,
}


impl BlockState for HeavyCore {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true { return 29499; }
        if self.r#waterlogged == false { return 29500; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29499 {
            return Some(HeavyCore {
                r#waterlogged: true,
            });
        }
        if state_id == 29500 {
            return Some(HeavyCore {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

