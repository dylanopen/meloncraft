use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkCandle {
    pub lit: bool,
    pub waterlogged: bool,
    pub candles: i32,
}


impl BlockState for PinkCandle {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 3 { return 23015; }
        if block_state.r#lit == false && block_state.r#candles == 2 && block_state.r#waterlogged == false { return 23013; }
        if block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#candles == 1 { return 23009; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == false { return 23017; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 2 { return 23012; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 4 { return 23020; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 2 { return 23010; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == true { return 23019; }
        if block_state.r#candles == 1 && block_state.r#waterlogged == true && block_state.r#lit == false { return 23008; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 3 { return 23016; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 1 { return 23007; }
        if block_state.r#waterlogged == true && block_state.r#candles == 1 && block_state.r#lit == true { return 23006; }
        if block_state.r#waterlogged == false && block_state.r#candles == 2 && block_state.r#lit == true { return 23011; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == false { return 23021; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#candles == 4 { return 23018; }
        if block_state.r#candles == 3 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23014; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23015 {
            return Some(PinkCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 23013 {
            return Some(PinkCandle {
                r#lit: false,
                r#candles: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 23009 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 23017 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23012 {
            return Some(PinkCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 2,
            });
        }
        if state_id == 23020 {
            return Some(PinkCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 23010 {
            return Some(PinkCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 23019 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23008 {
            return Some(PinkCandle {
                r#candles: 1,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23016 {
            return Some(PinkCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23007 {
            return Some(PinkCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 23006 {
            return Some(PinkCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 23011 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 23021 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 23018 {
            return Some(PinkCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 23014 {
            return Some(PinkCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

