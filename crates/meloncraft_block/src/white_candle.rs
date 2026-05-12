use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteCandle {
    pub lit: bool,
    pub waterlogged: bool,
    pub candles: i32,
}


impl BlockState for WhiteCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == true { return 22922; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 4 { return 22924; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 3 { return 22920; }
        if block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#candles == 1 { return 22913; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 4 { return 22925; }
        if block_state.r#waterlogged == false && block_state.r#candles == 2 && block_state.r#lit == false { return 22917; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 4 { return 22923; }
        if block_state.r#waterlogged == true && block_state.r#candles == 2 && block_state.r#lit == false { return 22916; }
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == true { return 22914; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == true { return 22910; }
        if block_state.r#candles == 3 && block_state.r#lit == true && block_state.r#waterlogged == true { return 22918; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 3 { return 22919; }
        if block_state.r#lit == false && block_state.r#candles == 3 && block_state.r#waterlogged == false { return 22921; }
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == false { return 22915; }
        if block_state.r#waterlogged == false && block_state.r#candles == 1 && block_state.r#lit == true { return 22911; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 1 { return 22912; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22922 {
            return Some(WhiteCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 22924 {
            return Some(WhiteCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 4,
            });
        }
        if state_id == 22920 {
            return Some(WhiteCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 22913 {
            return Some(WhiteCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 22925 {
            return Some(WhiteCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 22917 {
            return Some(WhiteCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 22923 {
            return Some(WhiteCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 22916 {
            return Some(WhiteCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 22914 {
            return Some(WhiteCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22910 {
            return Some(WhiteCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22918 {
            return Some(WhiteCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22919 {
            return Some(WhiteCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 22921 {
            return Some(WhiteCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 22915 {
            return Some(WhiteCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22911 {
            return Some(WhiteCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 22912 {
            return Some(WhiteCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        return None;
    }
}

