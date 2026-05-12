use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayCandle {
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}


impl BlockState for LightGrayCandle {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true && block_state.r#candles == 3 && block_state.r#waterlogged == false { return 23047; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23039; }
        if block_state.r#lit == true && block_state.r#candles == 2 && block_state.r#waterlogged == false { return 23043; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 2 { return 23042; }
        if block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#candles == 4 { return 23051; }
        if block_state.r#candles == 4 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23050; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == true && block_state.r#lit == true { return 23038; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 2 { return 23044; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 4 { return 23052; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 3 { return 23046; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 1 { return 23040; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 3 { return 23049; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == false && block_state.r#lit == false { return 23053; }
        if block_state.r#lit == false && block_state.r#candles == 3 && block_state.r#waterlogged == true { return 23048; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == false && block_state.r#lit == false { return 23041; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == false && block_state.r#lit == false { return 23045; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23047 {
            return Some(LightGrayCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 23039 {
            return Some(LightGrayCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23043 {
            return Some(LightGrayCandle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 23042 {
            return Some(LightGrayCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 23051 {
            return Some(LightGrayCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 4,
            });
        }
        if state_id == 23050 {
            return Some(LightGrayCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23038 {
            return Some(LightGrayCandle {
                r#candles: 1,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23044 {
            return Some(LightGrayCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 2,
            });
        }
        if state_id == 23052 {
            return Some(LightGrayCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 23046 {
            return Some(LightGrayCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23040 {
            return Some(LightGrayCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 23049 {
            return Some(LightGrayCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 23053 {
            return Some(LightGrayCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 23048 {
            return Some(LightGrayCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 23041 {
            return Some(LightGrayCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 23045 {
            return Some(LightGrayCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        return None;
    }
}

