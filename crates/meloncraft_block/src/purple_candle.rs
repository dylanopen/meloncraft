use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleCandle {
    pub lit: bool,
    pub waterlogged: bool,
    pub candles: i32,
}


impl BlockState for PurpleCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 1 { return 23070; }
        if block_state.r#candles == 3 && block_state.r#lit == false && block_state.r#waterlogged == false { return 23081; }
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23074; }
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == true { return 23082; }
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == false { return 23080; }
        if block_state.r#candles == 4 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23083; }
        if block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#candles == 2 { return 23075; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 2 { return 23076; }
        if block_state.r#candles == 1 && block_state.r#lit == false && block_state.r#waterlogged == true { return 23072; }
        if block_state.r#candles == 3 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23079; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == false { return 23071; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == false && block_state.r#lit == false { return 23077; }
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == false { return 23084; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == false { return 23085; }
        if block_state.r#candles == 1 && block_state.r#lit == false && block_state.r#waterlogged == false { return 23073; }
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 3 { return 23078; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23070 {
            return Some(PurpleCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 23081 {
            return Some(PurpleCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23074 {
            return Some(PurpleCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23082 {
            return Some(PurpleCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23080 {
            return Some(PurpleCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23083 {
            return Some(PurpleCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23075 {
            return Some(PurpleCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 2,
            });
        }
        if state_id == 23076 {
            return Some(PurpleCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 2,
            });
        }
        if state_id == 23072 {
            return Some(PurpleCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23079 {
            return Some(PurpleCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23071 {
            return Some(PurpleCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23077 {
            return Some(PurpleCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 23084 {
            return Some(PurpleCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 23085 {
            return Some(PurpleCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 23073 {
            return Some(PurpleCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23078 {
            return Some(PurpleCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 3,
            });
        }
        return None;
    }
}

