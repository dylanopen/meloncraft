use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedBanner {
    pub rotation: i32,
}


impl BlockState for RedBanner {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 4 { return 12953; }
        if block_state.r#rotation == 12 { return 12961; }
        if block_state.r#rotation == 6 { return 12955; }
        if block_state.r#rotation == 14 { return 12963; }
        if block_state.r#rotation == 5 { return 12954; }
        if block_state.r#rotation == 2 { return 12951; }
        if block_state.r#rotation == 7 { return 12956; }
        if block_state.r#rotation == 9 { return 12958; }
        if block_state.r#rotation == 15 { return 12964; }
        if block_state.r#rotation == 11 { return 12960; }
        if block_state.r#rotation == 0 { return 12949; }
        if block_state.r#rotation == 1 { return 12950; }
        if block_state.r#rotation == 13 { return 12962; }
        if block_state.r#rotation == 3 { return 12952; }
        if block_state.r#rotation == 10 { return 12959; }
        if block_state.r#rotation == 8 { return 12957; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12953 {
            return Some(RedBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12961 {
            return Some(RedBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12955 {
            return Some(RedBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12963 {
            return Some(RedBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12954 {
            return Some(RedBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12951 {
            return Some(RedBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12956 {
            return Some(RedBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12958 {
            return Some(RedBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12964 {
            return Some(RedBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12960 {
            return Some(RedBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12949 {
            return Some(RedBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12950 {
            return Some(RedBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12962 {
            return Some(RedBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12952 {
            return Some(RedBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12959 {
            return Some(RedBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12957 {
            return Some(RedBanner {
                r#rotation: 8,
            });
        }
        return None;
    }
}

