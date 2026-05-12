use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeBanner {
    pub rotation: i32,
}


impl BlockState for OrangeBanner {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 6 { return 12747; }
        if block_state.r#rotation == 4 { return 12745; }
        if block_state.r#rotation == 10 { return 12751; }
        if block_state.r#rotation == 14 { return 12755; }
        if block_state.r#rotation == 2 { return 12743; }
        if block_state.r#rotation == 0 { return 12741; }
        if block_state.r#rotation == 11 { return 12752; }
        if block_state.r#rotation == 9 { return 12750; }
        if block_state.r#rotation == 13 { return 12754; }
        if block_state.r#rotation == 5 { return 12746; }
        if block_state.r#rotation == 1 { return 12742; }
        if block_state.r#rotation == 15 { return 12756; }
        if block_state.r#rotation == 8 { return 12749; }
        if block_state.r#rotation == 7 { return 12748; }
        if block_state.r#rotation == 12 { return 12753; }
        if block_state.r#rotation == 3 { return 12744; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12747 {
            return Some(OrangeBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12745 {
            return Some(OrangeBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12751 {
            return Some(OrangeBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12755 {
            return Some(OrangeBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12743 {
            return Some(OrangeBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12741 {
            return Some(OrangeBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12752 {
            return Some(OrangeBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12750 {
            return Some(OrangeBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12754 {
            return Some(OrangeBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12746 {
            return Some(OrangeBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12742 {
            return Some(OrangeBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12756 {
            return Some(OrangeBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12749 {
            return Some(OrangeBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12748 {
            return Some(OrangeBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12753 {
            return Some(OrangeBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12744 {
            return Some(OrangeBanner {
                r#rotation: 3,
            });
        }
        return None;
    }
}

