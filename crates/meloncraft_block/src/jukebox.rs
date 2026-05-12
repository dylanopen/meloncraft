use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Jukebox {
    pub has_record: bool,
}


impl BlockState for Jukebox {
    fn to_id(&self) -> i32 {
        if self.r#has_record == false { return 6763; }
        if self.r#has_record == true { return 6762; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6763 {
            return Some(Jukebox {
                r#has_record: false,
            });
        }
        if state_id == 6762 {
            return Some(Jukebox {
                r#has_record: true,
            });
        }
        return None;
    }
}

