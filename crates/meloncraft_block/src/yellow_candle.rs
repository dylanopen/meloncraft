use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowCandle {
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}


impl BlockState for YellowCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 3 { return 22982; }
        if block_state.r#candles == 4 && block_state.r#lit == true && block_state.r#waterlogged == false { return 22987; }
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == false { return 22979; }
        if block_state.r#waterlogged == false && block_state.r#candles == 1 && block_state.r#lit == false { return 22977; }
        if block_state.r#waterlogged == false && block_state.r#candles == 1 && block_state.r#lit == true { return 22975; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == true && block_state.r#lit == false { return 22988; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 2 { return 22981; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == false && block_state.r#lit == false { return 22989; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 2 { return 22978; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 3 { return 22984; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == true && block_state.r#lit == false { return 22976; }
        if block_state.r#candles == 4 && block_state.r#lit == true && block_state.r#waterlogged == true { return 22986; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == true { return 22983; }
        if block_state.r#candles == 2 && block_state.r#lit == false && block_state.r#waterlogged == true { return 22980; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == false { return 22985; }
        if block_state.r#lit == true && block_state.r#candles == 1 && block_state.r#waterlogged == true { return 22974; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22982 {
            return Some(YellowCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 3,
            });
        }
        if state_id == 22987 {
            return Some(YellowCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22979 {
            return Some(YellowCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22977 {
            return Some(YellowCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 22975 {
            return Some(YellowCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 22988 {
            return Some(YellowCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22981 {
            return Some(YellowCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 22989 {
            return Some(YellowCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 22978 {
            return Some(YellowCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 22984 {
            return Some(YellowCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 22976 {
            return Some(YellowCandle {
                r#candles: 1,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22986 {
            return Some(YellowCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22983 {
            return Some(YellowCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 22980 {
            return Some(YellowCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22985 {
            return Some(YellowCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 22974 {
            return Some(YellowCandle {
                r#lit: true,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

