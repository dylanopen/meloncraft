use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedCandle {
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}


impl BlockState for RedCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#candles == 2 && block_state.r#lit == false { return 23140; }
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == false { return 23148; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 1 { return 23137; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 4 { return 23149; }
        if block_state.r#lit == true && block_state.r#candles == 3 && block_state.r#waterlogged == true { return 23142; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23134; }
        if block_state.r#lit == true && block_state.r#candles == 2 && block_state.r#waterlogged == false { return 23139; }
        if block_state.r#waterlogged == true && block_state.r#candles == 1 && block_state.r#lit == false { return 23136; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 3 { return 23145; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 3 { return 23144; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 2 { return 23141; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == false && block_state.r#lit == true { return 23135; }
        if block_state.r#lit == true && block_state.r#candles == 4 && block_state.r#waterlogged == false { return 23147; }
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 4 { return 23146; }
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 2 { return 23138; }
        if block_state.r#candles == 3 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23143; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23140 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23148 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 23137 {
            return Some(RedCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 23149 {
            return Some(RedCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 23142 {
            return Some(RedCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 23134 {
            return Some(RedCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23139 {
            return Some(RedCandle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 23136 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 23145 {
            return Some(RedCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 23144 {
            return Some(RedCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23141 {
            return Some(RedCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 23135 {
            return Some(RedCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23147 {
            return Some(RedCandle {
                r#lit: true,
                r#candles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 23146 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 4,
            });
        }
        if state_id == 23138 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 2,
            });
        }
        if state_id == 23143 {
            return Some(RedCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

