use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for Lantern {
    fn to_id(&self) -> i32 {
        if self.r#hanging == false && self.r#waterlogged == false { return 20638; }
        if self.r#hanging == true && self.r#waterlogged == false { return 20636; }
        if self.r#waterlogged == true && self.r#hanging == false { return 20637; }
        if self.r#waterlogged == true && self.r#hanging == true { return 20635; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20638 {
            return Some(Lantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20636 {
            return Some(Lantern {
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20637 {
            return Some(Lantern {
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 20635 {
            return Some(Lantern {
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        return None;
    }
}

