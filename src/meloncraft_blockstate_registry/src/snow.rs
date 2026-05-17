use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snow {
    pub layers: i32,
}


impl BlockState for Snow {
    fn to_id(&self) -> i32 {
        if self.r#layers == 4 { return 6721; }
        if self.r#layers == 5 { return 6722; }
        if self.r#layers == 7 { return 6724; }
        if self.r#layers == 6 { return 6723; }
        if self.r#layers == 3 { return 6720; }
        if self.r#layers == 8 { return 6725; }
        if self.r#layers == 1 { return 6718; }
        if self.r#layers == 2 { return 6719; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6721 {
            return Some(Snow {
                r#layers: 4,
            });
        }
        if state_id == 6722 {
            return Some(Snow {
                r#layers: 5,
            });
        }
        if state_id == 6724 {
            return Some(Snow {
                r#layers: 7,
            });
        }
        if state_id == 6723 {
            return Some(Snow {
                r#layers: 6,
            });
        }
        if state_id == 6720 {
            return Some(Snow {
                r#layers: 3,
            });
        }
        if state_id == 6725 {
            return Some(Snow {
                r#layers: 8,
            });
        }
        if state_id == 6718 {
            return Some(Snow {
                r#layers: 1,
            });
        }
        if state_id == 6719 {
            return Some(Snow {
                r#layers: 2,
            });
        }
        return None;
    }
}

