use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedCandle {
    pub waterlogged: bool,
    pub lit: bool,
    pub candles: i32,
}


impl BlockState for RedCandle {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == true { return 23138; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 4 { return 23149; }
        if self.r#lit == true && self.r#candles == 1 && self.r#waterlogged == true { return 23134; }
        if self.r#candles == 4 && self.r#waterlogged == true && self.r#lit == false { return 23148; }
        if self.r#waterlogged == false && self.r#lit == true && self.r#candles == 1 { return 23135; }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == true { return 23146; }
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 4 { return 23147; }
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == false { return 23136; }
        if self.r#waterlogged == true && self.r#candles == 3 && self.r#lit == false { return 23144; }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == false { return 23140; }
        if self.r#candles == 3 && self.r#waterlogged == true && self.r#lit == true { return 23142; }
        if self.r#lit == false && self.r#candles == 2 && self.r#waterlogged == false { return 23141; }
        if self.r#lit == true && self.r#candles == 3 && self.r#waterlogged == false { return 23143; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 1 { return 23137; }
        if self.r#waterlogged == false && self.r#lit == true && self.r#candles == 2 { return 23139; }
        if self.r#candles == 3 && self.r#waterlogged == false && self.r#lit == false { return 23145; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23138 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 23149 {
            return Some(RedCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 23134 {
            return Some(RedCandle {
                r#lit: true,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 23148 {
            return Some(RedCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23135 {
            return Some(RedCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 23146 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23147 {
            return Some(RedCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 23136 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 23144 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23140 {
            return Some(RedCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23142 {
            return Some(RedCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23141 {
            return Some(RedCandle {
                r#lit: false,
                r#candles: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 23143 {
            return Some(RedCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 23137 {
            return Some(RedCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 23139 {
            return Some(RedCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 2,
            });
        }
        if state_id == 23145 {
            return Some(RedCandle {
                r#candles: 3,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        return None;
    }
}

