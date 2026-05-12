use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueCandle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}


impl BlockState for LightBlueCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 1 { return 22958; }
        if block_state.r#lit == false && block_state.r#candles == 1 && block_state.r#waterlogged == false { return 22961; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == false { return 22973; }
        if block_state.r#candles == 2 && block_state.r#lit == false && block_state.r#waterlogged == false { return 22965; }
        if block_state.r#candles == 3 && block_state.r#lit == true && block_state.r#waterlogged == true { return 22966; }
        if block_state.r#candles == 3 && block_state.r#waterlogged == true && block_state.r#lit == false { return 22968; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 4 { return 22970; }
        if block_state.r#lit == false && block_state.r#candles == 1 && block_state.r#waterlogged == true { return 22960; }
        if block_state.r#waterlogged == false && block_state.r#candles == 2 && block_state.r#lit == true { return 22963; }
        if block_state.r#lit == true && block_state.r#candles == 3 && block_state.r#waterlogged == false { return 22967; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 4 { return 22971; }
        if block_state.r#candles == 2 && block_state.r#lit == false && block_state.r#waterlogged == true { return 22964; }
        if block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#candles == 1 { return 22959; }
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == false { return 22972; }
        if block_state.r#candles == 3 && block_state.r#lit == false && block_state.r#waterlogged == false { return 22969; }
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == true { return 22962; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22958 {
            return Some(LightBlueCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 22961 {
            return Some(LightBlueCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 22973 {
            return Some(LightBlueCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22965 {
            return Some(LightBlueCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22966 {
            return Some(LightBlueCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22968 {
            return Some(LightBlueCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22970 {
            return Some(LightBlueCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 22960 {
            return Some(LightBlueCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 22963 {
            return Some(LightBlueCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 22967 {
            return Some(LightBlueCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 22971 {
            return Some(LightBlueCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 22964 {
            return Some(LightBlueCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22959 {
            return Some(LightBlueCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 22972 {
            return Some(LightBlueCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22969 {
            return Some(LightBlueCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22962 {
            return Some(LightBlueCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

