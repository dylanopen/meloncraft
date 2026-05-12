use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeCandle {
    pub lit: bool,
    pub candles: i32,
    pub waterlogged: bool,
}


impl BlockState for LimeCandle {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#lit == true && self.r#candles == 1 { return 22990; }
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == false { return 23003; }
        if self.r#waterlogged == true && self.r#lit == true && self.r#candles == 2 { return 22994; }
        if self.r#lit == false && self.r#candles == 2 && self.r#waterlogged == true { return 22996; }
        if self.r#lit == false && self.r#candles == 3 && self.r#waterlogged == false { return 23001; }
        if self.r#waterlogged == true && self.r#lit == false && self.r#candles == 3 { return 23000; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 4 { return 23004; }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 3 { return 22998; }
        if self.r#waterlogged == false && self.r#lit == false && self.r#candles == 4 { return 23005; }
        if self.r#candles == 1 && self.r#waterlogged == false && self.r#lit == true { return 22991; }
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == false { return 22999; }
        if self.r#lit == true && self.r#candles == 2 && self.r#waterlogged == false { return 22995; }
        if self.r#lit == false && self.r#candles == 1 && self.r#waterlogged == false { return 22993; }
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == true { return 23002; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 1 { return 22992; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 2 { return 22997; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22990 {
            return Some(LimeCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 23003 {
            return Some(LimeCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22994 {
            return Some(LimeCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 2,
            });
        }
        if state_id == 22996 {
            return Some(LimeCandle {
                r#lit: false,
                r#candles: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 23001 {
            return Some(LimeCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 23000 {
            return Some(LimeCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 3,
            });
        }
        if state_id == 23004 {
            return Some(LimeCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 22998 {
            return Some(LimeCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23005 {
            return Some(LimeCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 4,
            });
        }
        if state_id == 22991 {
            return Some(LimeCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22999 {
            return Some(LimeCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22995 {
            return Some(LimeCandle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 22993 {
            return Some(LimeCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 23002 {
            return Some(LimeCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22992 {
            return Some(LimeCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 22997 {
            return Some(LimeCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        return None;
    }
}

