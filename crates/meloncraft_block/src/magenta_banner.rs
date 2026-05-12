use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaBanner {
    pub rotation: i32,
}


impl BlockState for MagentaBanner {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 2 { return 12759; }
        if block_state.r#rotation == 10 { return 12767; }
        if block_state.r#rotation == 4 { return 12761; }
        if block_state.r#rotation == 8 { return 12765; }
        if block_state.r#rotation == 13 { return 12770; }
        if block_state.r#rotation == 5 { return 12762; }
        if block_state.r#rotation == 0 { return 12757; }
        if block_state.r#rotation == 15 { return 12772; }
        if block_state.r#rotation == 3 { return 12760; }
        if block_state.r#rotation == 12 { return 12769; }
        if block_state.r#rotation == 6 { return 12763; }
        if block_state.r#rotation == 7 { return 12764; }
        if block_state.r#rotation == 9 { return 12766; }
        if block_state.r#rotation == 11 { return 12768; }
        if block_state.r#rotation == 1 { return 12758; }
        if block_state.r#rotation == 14 { return 12771; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12759 {
            return Some(MagentaBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12767 {
            return Some(MagentaBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12761 {
            return Some(MagentaBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12765 {
            return Some(MagentaBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12770 {
            return Some(MagentaBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12762 {
            return Some(MagentaBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12757 {
            return Some(MagentaBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12772 {
            return Some(MagentaBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12760 {
            return Some(MagentaBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12769 {
            return Some(MagentaBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12763 {
            return Some(MagentaBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12764 {
            return Some(MagentaBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12766 {
            return Some(MagentaBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12768 {
            return Some(MagentaBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12758 {
            return Some(MagentaBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12771 {
            return Some(MagentaBanner {
                r#rotation: 14,
            });
        }
        return None;
    }
}

