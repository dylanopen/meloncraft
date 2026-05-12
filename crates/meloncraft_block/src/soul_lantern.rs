use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulLantern {
    pub hanging: bool,
    pub waterlogged: bool,
}


impl BlockState for SoulLantern {
    fn to_id(&self) -> i32 {
        if self.r#hanging == true && self.r#waterlogged == true { return 20639; }
        if self.r#waterlogged == false && self.r#hanging == true { return 20640; }
        if self.r#waterlogged == true && self.r#hanging == false { return 20641; }
        if self.r#hanging == false && self.r#waterlogged == false { return 20642; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20639 {
            return Some(SoulLantern {
                r#hanging: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20640 {
            return Some(SoulLantern {
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        if state_id == 20641 {
            return Some(SoulLantern {
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 20642 {
            return Some(SoulLantern {
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

