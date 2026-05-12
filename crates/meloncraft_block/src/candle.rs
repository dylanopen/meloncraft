use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}


impl BlockState for Candle {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == false { return 22896; }
        if self.r#candles == 2 && self.r#waterlogged == false && self.r#lit == true { return 22899; }
        if self.r#waterlogged == false && self.r#candles == 4 && self.r#lit == true { return 22907; }
        if self.r#candles == 4 && self.r#lit == false && self.r#waterlogged == false { return 22909; }
        if self.r#lit == true && self.r#candles == 3 && self.r#waterlogged == true { return 22902; }
        if self.r#lit == true && self.r#candles == 1 && self.r#waterlogged == true { return 22894; }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == true { return 22900; }
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == false { return 22903; }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == true { return 22898; }
        if self.r#candles == 1 && self.r#waterlogged == false && self.r#lit == true { return 22895; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 3 { return 22905; }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 4 { return 22906; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 2 { return 22901; }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == false { return 22908; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 3 { return 22904; }
        if self.r#lit == false && self.r#candles == 1 && self.r#waterlogged == false { return 22897; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22896 {
            return Some(Candle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 22899 {
            return Some(Candle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22907 {
            return Some(Candle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 22909 {
            return Some(Candle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22902 {
            return Some(Candle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 22894 {
            return Some(Candle {
                r#lit: true,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 22900 {
            return Some(Candle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22903 {
            return Some(Candle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22898 {
            return Some(Candle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 22895 {
            return Some(Candle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22905 {
            return Some(Candle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 22906 {
            return Some(Candle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 22901 {
            return Some(Candle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 22908 {
            return Some(Candle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22904 {
            return Some(Candle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 22897 {
            return Some(Candle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

