use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownCandle {
    pub waterlogged: bool,
    pub lit: bool,
    pub candles: i32,
}


impl BlockState for BrownCandle {
    fn to_id(self) -> i32 {
        if block_state.r#candles == 4 && block_state.r#lit == false && block_state.r#waterlogged == false { return 23117; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 1 { return 23102; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 1 { return 23104; }
        if block_state.r#lit == false && block_state.r#candles == 3 && block_state.r#waterlogged == true { return 23112; }
        if block_state.r#candles == 4 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23115; }
        if block_state.r#waterlogged == true && block_state.r#candles == 2 && block_state.r#lit == false { return 23108; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == false && block_state.r#lit == true { return 23103; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == false && block_state.r#lit == true { return 23107; }
        if block_state.r#waterlogged == false && block_state.r#candles == 2 && block_state.r#lit == false { return 23109; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == true { return 23111; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 4 { return 23116; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 3 { return 23110; }
        if block_state.r#lit == true && block_state.r#candles == 4 && block_state.r#waterlogged == true { return 23114; }
        if block_state.r#candles == 3 && block_state.r#waterlogged == false && block_state.r#lit == false { return 23113; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 2 { return 23106; }
        if block_state.r#lit == false && block_state.r#candles == 1 && block_state.r#waterlogged == false { return 23105; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23117 {
            return Some(BrownCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23102 {
            return Some(BrownCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 23104 {
            return Some(BrownCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 23112 {
            return Some(BrownCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 23115 {
            return Some(BrownCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23108 {
            return Some(BrownCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23103 {
            return Some(BrownCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23107 {
            return Some(BrownCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23109 {
            return Some(BrownCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23111 {
            return Some(BrownCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 23116 {
            return Some(BrownCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 4,
            });
        }
        if state_id == 23110 {
            return Some(BrownCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23114 {
            return Some(BrownCandle {
                r#lit: true,
                r#candles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 23113 {
            return Some(BrownCandle {
                r#candles: 3,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 23106 {
            return Some(BrownCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 23105 {
            return Some(BrownCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

