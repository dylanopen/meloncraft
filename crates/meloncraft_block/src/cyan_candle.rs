use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanCandle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}


impl BlockState for CyanCandle {
    fn to_id(self) -> i32 {
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 1 { return 23056; }
        if block_state.r#candles == 3 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23062; }
        if block_state.r#waterlogged == false && block_state.r#candles == 1 && block_state.r#lit == true { return 23055; }
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 1 { return 23054; }
        if block_state.r#lit == false && block_state.r#candles == 4 && block_state.r#waterlogged == false { return 23069; }
        if block_state.r#candles == 4 && block_state.r#lit == true && block_state.r#waterlogged == true { return 23066; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == false && block_state.r#lit == true { return 23067; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == true && block_state.r#lit == true { return 23058; }
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == false { return 23068; }
        if block_state.r#waterlogged == false && block_state.r#candles == 1 && block_state.r#lit == false { return 23057; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 3 { return 23063; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 2 { return 23060; }
        if block_state.r#lit == false && block_state.r#candles == 3 && block_state.r#waterlogged == true { return 23064; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 2 { return 23061; }
        if block_state.r#candles == 3 && block_state.r#lit == false && block_state.r#waterlogged == false { return 23065; }
        if block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#candles == 2 { return 23059; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23056 {
            return Some(CyanCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 23062 {
            return Some(CyanCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23055 {
            return Some(CyanCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 23054 {
            return Some(CyanCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 23069 {
            return Some(CyanCandle {
                r#lit: false,
                r#candles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 23066 {
            return Some(CyanCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23067 {
            return Some(CyanCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23058 {
            return Some(CyanCandle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23068 {
            return Some(CyanCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 23057 {
            return Some(CyanCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 23063 {
            return Some(CyanCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 23060 {
            return Some(CyanCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 2,
            });
        }
        if state_id == 23064 {
            return Some(CyanCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 23061 {
            return Some(CyanCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 23065 {
            return Some(CyanCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23059 {
            return Some(CyanCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 2,
            });
        }
        return None;
    }
}

