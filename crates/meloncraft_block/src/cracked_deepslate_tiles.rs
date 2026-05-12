use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrackedDeepslateTiles {
}


impl BlockState for CrackedDeepslateTiles {
    fn to_id(self) -> i32 {
        return 29370;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29370 {
            return Some(CrackedDeepslateTiles {
            });
        }
        return None;
    }
}

