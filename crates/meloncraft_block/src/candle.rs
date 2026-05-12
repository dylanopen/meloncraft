use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candle {
    pub lit: bool,
    pub candles: i32,
    pub waterlogged: bool,
}


impl BlockState for Candle {
    fn to_id(self) -> i32 {
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == false { return 22899; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == false && block_state.r#lit == true { return 22907; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == true { return 22903; }
        if block_state.r#candles == 3 && block_state.r#lit == false && block_state.r#waterlogged == false { return 22905; }
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == true { return 22902; }
        if block_state.r#candles == 2 && block_state.r#lit == false && block_state.r#waterlogged == false { return 22901; }
        if block_state.r#candles == 4 && block_state.r#lit == false && block_state.r#waterlogged == true { return 22908; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == false { return 22895; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == true && block_state.r#lit == false { return 22900; }
        if block_state.r#lit == true && block_state.r#candles == 4 && block_state.r#waterlogged == true { return 22906; }
        if block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#candles == 4 { return 22909; }
        if block_state.r#lit == true && block_state.r#candles == 2 && block_state.r#waterlogged == true { return 22898; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 1 { return 22894; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == false && block_state.r#lit == false { return 22897; }
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == false { return 22904; }
        if block_state.r#waterlogged == true && block_state.r#candles == 1 && block_state.r#lit == false { return 22896; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22899 {
            return Some(Candle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22907 {
            return Some(Candle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22903 {
            return Some(Candle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 22905 {
            return Some(Candle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22902 {
            return Some(Candle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 22901 {
            return Some(Candle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22908 {
            return Some(Candle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22895 {
            return Some(Candle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22900 {
            return Some(Candle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22906 {
            return Some(Candle {
                r#lit: true,
                r#candles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 22909 {
            return Some(Candle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 4,
            });
        }
        if state_id == 22898 {
            return Some(Candle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 22894 {
            return Some(Candle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 22897 {
            return Some(Candle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 22904 {
            return Some(Candle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 22896 {
            return Some(Candle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        return None;
    }
}

