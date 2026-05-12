use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeCandle {
    pub lit: bool,
    pub waterlogged: bool,
    pub candles: i32,
}


impl BlockState for LimeCandle {
    fn to_id(self) -> i32 {
        if block_state.r#candles == 1 && block_state.r#lit == false && block_state.r#waterlogged == false { return 22993; }
        if block_state.r#waterlogged == true && block_state.r#candles == 2 && block_state.r#lit == false { return 22996; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == true && block_state.r#lit == true { return 22990; }
        if block_state.r#lit == true && block_state.r#candles == 1 && block_state.r#waterlogged == false { return 22991; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 3 { return 22999; }
        if block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#candles == 4 { return 23005; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 3 { return 23000; }
        if block_state.r#candles == 3 && block_state.r#lit == false && block_state.r#waterlogged == false { return 23001; }
        if block_state.r#lit == true && block_state.r#candles == 4 && block_state.r#waterlogged == true { return 23002; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == false && block_state.r#lit == true { return 22995; }
        if block_state.r#waterlogged == false && block_state.r#candles == 2 && block_state.r#lit == false { return 22997; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == true { return 23003; }
        if block_state.r#waterlogged == true && block_state.r#candles == 1 && block_state.r#lit == false { return 22992; }
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == true { return 22998; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 2 { return 22994; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == true && block_state.r#lit == false { return 23004; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22993 {
            return Some(LimeCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22996 {
            return Some(LimeCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 22990 {
            return Some(LimeCandle {
                r#candles: 1,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 22991 {
            return Some(LimeCandle {
                r#lit: true,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 22999 {
            return Some(LimeCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 23005 {
            return Some(LimeCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 4,
            });
        }
        if state_id == 23000 {
            return Some(LimeCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 3,
            });
        }
        if state_id == 23001 {
            return Some(LimeCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23002 {
            return Some(LimeCandle {
                r#lit: true,
                r#candles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 22995 {
            return Some(LimeCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22997 {
            return Some(LimeCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23003 {
            return Some(LimeCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 22992 {
            return Some(LimeCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 22998 {
            return Some(LimeCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 22994 {
            return Some(LimeCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 23004 {
            return Some(LimeCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        return None;
    }
}

