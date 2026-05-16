use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenCandle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}


impl BlockState for GreenCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 4 && self.r#lit == false && self.r#waterlogged == true { return 23132; }
        if self.r#candles == 4 && self.r#waterlogged == true && self.r#lit == true { return 23130; }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 1 { return 23118; }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == false { return 23125; }
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == true { return 23126; }
        if self.r#lit == false && self.r#candles == 1 && self.r#waterlogged == true { return 23120; }
        if self.r#waterlogged == false && self.r#candles == 3 && self.r#lit == false { return 23129; }
        if self.r#waterlogged == false && self.r#candles == 2 && self.r#lit == true { return 23123; }
        if self.r#lit == false && self.r#candles == 3 && self.r#waterlogged == true { return 23128; }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == false { return 23124; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == true { return 23122; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 1 { return 23121; }
        if self.r#candles == 4 && self.r#waterlogged == false && self.r#lit == true { return 23131; }
        if self.r#waterlogged == false && self.r#lit == true && self.r#candles == 3 { return 23127; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 4 { return 23133; }
        if self.r#waterlogged == false && self.r#candles == 1 && self.r#lit == true { return 23119; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23132 {
            return Some(GreenCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23130 {
            return Some(GreenCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23118 {
            return Some(GreenCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 23125 {
            return Some(GreenCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23126 {
            return Some(GreenCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23120 {
            return Some(GreenCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 23129 {
            return Some(GreenCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23123 {
            return Some(GreenCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 23128 {
            return Some(GreenCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 23124 {
            return Some(GreenCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23122 {
            return Some(GreenCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23121 {
            return Some(GreenCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 23131 {
            return Some(GreenCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23127 {
            return Some(GreenCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 3,
            });
        }
        if state_id == 23133 {
            return Some(GreenCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 23119 {
            return Some(GreenCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: true,
            });
        }
        return None;
    }
}

