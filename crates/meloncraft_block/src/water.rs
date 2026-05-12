use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Water {
    pub level: i32,
}


impl BlockState for Water {
    fn to_id(self) -> i32 {
        if block_state.r#level == 8 { return 94; }
        if block_state.r#level == 0 { return 86; }
        if block_state.r#level == 1 { return 87; }
        if block_state.r#level == 14 { return 100; }
        if block_state.r#level == 5 { return 91; }
        if block_state.r#level == 3 { return 89; }
        if block_state.r#level == 2 { return 88; }
        if block_state.r#level == 9 { return 95; }
        if block_state.r#level == 10 { return 96; }
        if block_state.r#level == 7 { return 93; }
        if block_state.r#level == 15 { return 101; }
        if block_state.r#level == 6 { return 92; }
        if block_state.r#level == 4 { return 90; }
        if block_state.r#level == 11 { return 97; }
        if block_state.r#level == 12 { return 98; }
        if block_state.r#level == 13 { return 99; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 94 {
            return Some(Water {
                r#level: 8,
            });
        }
        if state_id == 86 {
            return Some(Water {
                r#level: 0,
            });
        }
        if state_id == 87 {
            return Some(Water {
                r#level: 1,
            });
        }
        if state_id == 100 {
            return Some(Water {
                r#level: 14,
            });
        }
        if state_id == 91 {
            return Some(Water {
                r#level: 5,
            });
        }
        if state_id == 89 {
            return Some(Water {
                r#level: 3,
            });
        }
        if state_id == 88 {
            return Some(Water {
                r#level: 2,
            });
        }
        if state_id == 95 {
            return Some(Water {
                r#level: 9,
            });
        }
        if state_id == 96 {
            return Some(Water {
                r#level: 10,
            });
        }
        if state_id == 93 {
            return Some(Water {
                r#level: 7,
            });
        }
        if state_id == 101 {
            return Some(Water {
                r#level: 15,
            });
        }
        if state_id == 92 {
            return Some(Water {
                r#level: 6,
            });
        }
        if state_id == 90 {
            return Some(Water {
                r#level: 4,
            });
        }
        if state_id == 97 {
            return Some(Water {
                r#level: 11,
            });
        }
        if state_id == 98 {
            return Some(Water {
                r#level: 12,
            });
        }
        if state_id == 99 {
            return Some(Water {
                r#level: 13,
            });
        }
        return None;
    }
}

