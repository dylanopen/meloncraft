use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackCandle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}


impl BlockState for BlackCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#candles == 1 { return 23153; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 4 { return 23164; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23150; }
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == true { return 23158; }
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == true { return 23162; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == false { return 23161; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 2 { return 23156; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == true { return 23163; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 1 { return 23152; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23151; }
        if block_state.r#lit == true && block_state.r#candles == 3 && block_state.r#waterlogged == false { return 23159; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 2 { return 23157; }
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == false { return 23160; }
        if block_state.r#lit == true && block_state.r#candles == 2 && block_state.r#waterlogged == true { return 23154; }
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23155; }
        if block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#candles == 4 { return 23165; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23153 {
            return Some(BlackCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 23164 {
            return Some(BlackCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 23150 {
            return Some(BlackCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23158 {
            return Some(BlackCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 23162 {
            return Some(BlackCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23161 {
            return Some(BlackCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23156 {
            return Some(BlackCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 23163 {
            return Some(BlackCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23152 {
            return Some(BlackCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 23151 {
            return Some(BlackCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23159 {
            return Some(BlackCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 23157 {
            return Some(BlackCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 23160 {
            return Some(BlackCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23154 {
            return Some(BlackCandle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 23155 {
            return Some(BlackCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23165 {
            return Some(BlackCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 4,
            });
        }
        return None;
    }
}

