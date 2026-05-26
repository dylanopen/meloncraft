use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeaPickle {
    pub pickles: i32,
    pub waterlogged: bool,
}

impl BlockState for SeaPickle {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#pickles == 2 {
            return 15068;
        }
        if self.r#pickles == 3 && self.r#waterlogged == false {
            return 15070;
        }
        if self.r#waterlogged == true && self.r#pickles == 2 {
            return 15067;
        }
        if self.r#waterlogged == true && self.r#pickles == 1 {
            return 15065;
        }
        if self.r#pickles == 4 && self.r#waterlogged == false {
            return 15072;
        }
        if self.r#pickles == 4 && self.r#waterlogged == true {
            return 15071;
        }
        if self.r#waterlogged == true && self.r#pickles == 3 {
            return 15069;
        }
        if self.r#waterlogged == false && self.r#pickles == 1 {
            return 15066;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15068 {
            return Some(SeaPickle {
                r#waterlogged: false,
                r#pickles: 2,
            });
        }
        if state_id == 15070 {
            return Some(SeaPickle {
                r#pickles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 15067 {
            return Some(SeaPickle {
                r#waterlogged: true,
                r#pickles: 2,
            });
        }
        if state_id == 15065 {
            return Some(SeaPickle {
                r#waterlogged: true,
                r#pickles: 1,
            });
        }
        if state_id == 15072 {
            return Some(SeaPickle {
                r#pickles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 15071 {
            return Some(SeaPickle {
                r#pickles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 15069 {
            return Some(SeaPickle {
                r#waterlogged: true,
                r#pickles: 3,
            });
        }
        if state_id == 15066 {
            return Some(SeaPickle {
                r#waterlogged: false,
                r#pickles: 1,
            });
        }
        return None;
    }
}
