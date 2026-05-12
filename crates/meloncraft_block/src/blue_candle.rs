use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueCandle {
    pub lit: bool,
    pub candles: i32,
    pub waterlogged: bool,
}


impl BlockState for BlueCandle {
    fn to_id(self) -> i32 {
        if block_state.r#candles == 1 && block_state.r#lit == false && block_state.r#waterlogged == true { return 23088; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == false && block_state.r#lit == true { return 23091; }
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 3 { return 23094; }
        if block_state.r#candles == 4 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23098; }
        if block_state.r#lit == false && block_state.r#candles == 4 && block_state.r#waterlogged == true { return 23100; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23086; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == false { return 23097; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == true { return 23099; }
        if block_state.r#waterlogged == true && block_state.r#candles == 2 && block_state.r#lit == false { return 23092; }
        if block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#candles == 3 { return 23095; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == false && block_state.r#lit == true { return 23087; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 3 { return 23096; }
        if block_state.r#candles == 2 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23090; }
        if block_state.r#waterlogged == false && block_state.r#candles == 2 && block_state.r#lit == false { return 23093; }
        if block_state.r#lit == false && block_state.r#candles == 4 && block_state.r#waterlogged == false { return 23101; }
        if block_state.r#waterlogged == false && block_state.r#candles == 1 && block_state.r#lit == false { return 23089; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23088 {
            return Some(BlueCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23091 {
            return Some(BlueCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23094 {
            return Some(BlueCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 3,
            });
        }
        if state_id == 23098 {
            return Some(BlueCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23100 {
            return Some(BlueCandle {
                r#lit: false,
                r#candles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 23086 {
            return Some(BlueCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23097 {
            return Some(BlueCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23099 {
            return Some(BlueCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23092 {
            return Some(BlueCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23095 {
            return Some(BlueCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 3,
            });
        }
        if state_id == 23087 {
            return Some(BlueCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23096 {
            return Some(BlueCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 3,
            });
        }
        if state_id == 23090 {
            return Some(BlueCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23093 {
            return Some(BlueCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23101 {
            return Some(BlueCandle {
                r#lit: false,
                r#candles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 23089 {
            return Some(BlueCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: false,
            });
        }
        return None;
    }
}

