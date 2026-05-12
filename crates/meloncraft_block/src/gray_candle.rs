use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayCandle {
    pub waterlogged: bool,
    pub lit: bool,
    pub candles: i32,
}


impl BlockState for GrayCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == true { return 23035; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 1 { return 23025; }
        if block_state.r#lit == false && block_state.r#candles == 2 && block_state.r#waterlogged == true { return 23028; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 2 { return 23029; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == false && block_state.r#lit == true { return 23027; }
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == false { return 23032; }
        if block_state.r#lit == true && block_state.r#candles == 1 && block_state.r#waterlogged == false { return 23023; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == true && block_state.r#lit == true { return 23034; }
        if block_state.r#candles == 4 && block_state.r#lit == false && block_state.r#waterlogged == false { return 23037; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23022; }
        if block_state.r#lit == false && block_state.r#candles == 4 && block_state.r#waterlogged == true { return 23036; }
        if block_state.r#waterlogged == true && block_state.r#candles == 2 && block_state.r#lit == true { return 23026; }
        if block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#candles == 3 { return 23031; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == false { return 23033; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 1 { return 23024; }
        if block_state.r#candles == 3 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23030; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23035 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23025 {
            return Some(GrayCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 23028 {
            return Some(GrayCandle {
                r#lit: false,
                r#candles: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 23029 {
            return Some(GrayCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 23027 {
            return Some(GrayCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23032 {
            return Some(GrayCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23023 {
            return Some(GrayCandle {
                r#lit: true,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 23034 {
            return Some(GrayCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23037 {
            return Some(GrayCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23022 {
            return Some(GrayCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23036 {
            return Some(GrayCandle {
                r#lit: false,
                r#candles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 23026 {
            return Some(GrayCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 23031 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 3,
            });
        }
        if state_id == 23033 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23024 {
            return Some(GrayCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 23030 {
            return Some(GrayCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

