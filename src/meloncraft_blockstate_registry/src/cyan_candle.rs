use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanCandle {
    pub lit: bool,
    pub waterlogged: bool,
    pub candles: i32,
}

impl BlockState for CyanCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == false {
            return 23063;
        }
        if self.r#waterlogged == false && self.r#candles == 3 && self.r#lit == false {
            return 23065;
        }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == true {
            return 23066;
        }
        if self.r#lit == false && self.r#candles == 4 && self.r#waterlogged == true {
            return 23068;
        }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 3 {
            return 23062;
        }
        if self.r#candles == 2 && self.r#waterlogged == true && self.r#lit == false {
            return 23060;
        }
        if self.r#candles == 3 && self.r#waterlogged == true && self.r#lit == false {
            return 23064;
        }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == false {
            return 23061;
        }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == true {
            return 23058;
        }
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == false {
            return 23056;
        }
        if self.r#candles == 1 && self.r#lit == false && self.r#waterlogged == false {
            return 23057;
        }
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == true {
            return 23054;
        }
        if self.r#waterlogged == false && self.r#candles == 4 && self.r#lit == false {
            return 23069;
        }
        if self.r#lit == true && self.r#candles == 4 && self.r#waterlogged == false {
            return 23067;
        }
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 1 {
            return 23055;
        }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == false {
            return 23059;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23063 {
            return Some(CyanCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23065 {
            return Some(CyanCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23066 {
            return Some(CyanCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23068 {
            return Some(CyanCandle {
                r#lit: false,
                r#candles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 23062 {
            return Some(CyanCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23060 {
            return Some(CyanCandle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23064 {
            return Some(CyanCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23061 {
            return Some(CyanCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23058 {
            return Some(CyanCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 23056 {
            return Some(CyanCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 23057 {
            return Some(CyanCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23054 {
            return Some(CyanCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 23069 {
            return Some(CyanCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 23067 {
            return Some(CyanCandle {
                r#lit: true,
                r#candles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 23055 {
            return Some(CyanCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 23059 {
            return Some(CyanCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
