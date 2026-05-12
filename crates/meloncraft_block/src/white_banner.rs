use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteBanner {
    pub rotation: i32,
}


impl BlockState for WhiteBanner {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 3 { return 12728; }
        if block_state.r#rotation == 14 { return 12739; }
        if block_state.r#rotation == 0 { return 12725; }
        if block_state.r#rotation == 13 { return 12738; }
        if block_state.r#rotation == 4 { return 12729; }
        if block_state.r#rotation == 11 { return 12736; }
        if block_state.r#rotation == 7 { return 12732; }
        if block_state.r#rotation == 10 { return 12735; }
        if block_state.r#rotation == 2 { return 12727; }
        if block_state.r#rotation == 5 { return 12730; }
        if block_state.r#rotation == 1 { return 12726; }
        if block_state.r#rotation == 9 { return 12734; }
        if block_state.r#rotation == 8 { return 12733; }
        if block_state.r#rotation == 15 { return 12740; }
        if block_state.r#rotation == 12 { return 12737; }
        if block_state.r#rotation == 6 { return 12731; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12728 {
            return Some(WhiteBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12739 {
            return Some(WhiteBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12725 {
            return Some(WhiteBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12738 {
            return Some(WhiteBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12729 {
            return Some(WhiteBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12736 {
            return Some(WhiteBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12732 {
            return Some(WhiteBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12735 {
            return Some(WhiteBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12727 {
            return Some(WhiteBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12730 {
            return Some(WhiteBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12726 {
            return Some(WhiteBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12734 {
            return Some(WhiteBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12733 {
            return Some(WhiteBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12740 {
            return Some(WhiteBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12737 {
            return Some(WhiteBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12731 {
            return Some(WhiteBanner {
                r#rotation: 6,
            });
        }
        return None;
    }
}

