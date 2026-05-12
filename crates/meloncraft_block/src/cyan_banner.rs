use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanBanner {
    pub rotation: i32,
}


impl BlockState for CyanBanner {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 14 { return 12883; }
        if block_state.r#rotation == 7 { return 12876; }
        if block_state.r#rotation == 4 { return 12873; }
        if block_state.r#rotation == 3 { return 12872; }
        if block_state.r#rotation == 13 { return 12882; }
        if block_state.r#rotation == 12 { return 12881; }
        if block_state.r#rotation == 8 { return 12877; }
        if block_state.r#rotation == 9 { return 12878; }
        if block_state.r#rotation == 2 { return 12871; }
        if block_state.r#rotation == 1 { return 12870; }
        if block_state.r#rotation == 10 { return 12879; }
        if block_state.r#rotation == 5 { return 12874; }
        if block_state.r#rotation == 0 { return 12869; }
        if block_state.r#rotation == 6 { return 12875; }
        if block_state.r#rotation == 15 { return 12884; }
        if block_state.r#rotation == 11 { return 12880; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12883 {
            return Some(CyanBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12876 {
            return Some(CyanBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12873 {
            return Some(CyanBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12872 {
            return Some(CyanBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12882 {
            return Some(CyanBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12881 {
            return Some(CyanBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12877 {
            return Some(CyanBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12878 {
            return Some(CyanBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12871 {
            return Some(CyanBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12870 {
            return Some(CyanBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12879 {
            return Some(CyanBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12874 {
            return Some(CyanBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12869 {
            return Some(CyanBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12875 {
            return Some(CyanBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12884 {
            return Some(CyanBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12880 {
            return Some(CyanBanner {
                r#rotation: 11,
            });
        }
        return None;
    }
}

