use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkCandle {
    pub candles: i32,
    pub waterlogged: bool,
    pub lit: bool,
}


impl BlockState for PinkCandle {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == true { return 23018; }
        if self.r#candles == 3 && self.r#waterlogged == true && self.r#lit == true { return 23014; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == false { return 23011; }
        if self.r#candles == 4 && self.r#waterlogged == true && self.r#lit == false { return 23020; }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == false { return 23007; }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == true { return 23006; }
        if self.r#candles == 4 && self.r#waterlogged == false && self.r#lit == false { return 23021; }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == false { return 23012; }
        if self.r#waterlogged == false && self.r#lit == false && self.r#candles == 1 { return 23009; }
        if self.r#waterlogged == false && self.r#candles == 2 && self.r#lit == false { return 23013; }
        if self.r#candles == 3 && self.r#lit == false && self.r#waterlogged == false { return 23017; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == true { return 23010; }
        if self.r#candles == 4 && self.r#waterlogged == false && self.r#lit == true { return 23019; }
        if self.r#waterlogged == false && self.r#candles == 3 && self.r#lit == true { return 23015; }
        if self.r#lit == false && self.r#candles == 1 && self.r#waterlogged == true { return 23008; }
        if self.r#candles == 3 && self.r#lit == false && self.r#waterlogged == true { return 23016; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23018 {
            return Some(PinkCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23014 {
            return Some(PinkCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23011 {
            return Some(PinkCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23020 {
            return Some(PinkCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23007 {
            return Some(PinkCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23006 {
            return Some(PinkCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23021 {
            return Some(PinkCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 23012 {
            return Some(PinkCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23009 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 23013 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23017 {
            return Some(PinkCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23010 {
            return Some(PinkCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23019 {
            return Some(PinkCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23015 {
            return Some(PinkCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 23008 {
            return Some(PinkCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 23016 {
            return Some(PinkCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

