use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueBanner {
    pub rotation: i32,
}


impl BlockState for BlueBanner {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 6 { return 12907; }
        if block_state.r#rotation == 8 { return 12909; }
        if block_state.r#rotation == 3 { return 12904; }
        if block_state.r#rotation == 0 { return 12901; }
        if block_state.r#rotation == 9 { return 12910; }
        if block_state.r#rotation == 11 { return 12912; }
        if block_state.r#rotation == 14 { return 12915; }
        if block_state.r#rotation == 7 { return 12908; }
        if block_state.r#rotation == 15 { return 12916; }
        if block_state.r#rotation == 5 { return 12906; }
        if block_state.r#rotation == 1 { return 12902; }
        if block_state.r#rotation == 4 { return 12905; }
        if block_state.r#rotation == 13 { return 12914; }
        if block_state.r#rotation == 12 { return 12913; }
        if block_state.r#rotation == 2 { return 12903; }
        if block_state.r#rotation == 10 { return 12911; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12907 {
            return Some(BlueBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12909 {
            return Some(BlueBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12904 {
            return Some(BlueBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12901 {
            return Some(BlueBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12910 {
            return Some(BlueBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12912 {
            return Some(BlueBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12915 {
            return Some(BlueBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12908 {
            return Some(BlueBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12916 {
            return Some(BlueBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12906 {
            return Some(BlueBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12902 {
            return Some(BlueBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12905 {
            return Some(BlueBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12914 {
            return Some(BlueBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12913 {
            return Some(BlueBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12903 {
            return Some(BlueBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12911 {
            return Some(BlueBanner {
                r#rotation: 10,
            });
        }
        return None;
    }
}

