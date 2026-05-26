use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowCandle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}

impl BlockState for YellowCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 2 && self.r#waterlogged == true && self.r#lit == false {
            return 22980;
        }
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 1 {
            return 22975;
        }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 1 {
            return 22974;
        }
        if self.r#lit == true && self.r#candles == 3 && self.r#waterlogged == true {
            return 22982;
        }
        if self.r#candles == 4 && self.r#waterlogged == true && self.r#lit == false {
            return 22988;
        }
        if self.r#waterlogged == false && self.r#candles == 2 && self.r#lit == false {
            return 22981;
        }
        if self.r#candles == 4 && self.r#lit == false && self.r#waterlogged == false {
            return 22989;
        }
        if self.r#lit == false && self.r#candles == 1 && self.r#waterlogged == false {
            return 22977;
        }
        if self.r#candles == 3 && self.r#waterlogged == true && self.r#lit == false {
            return 22984;
        }
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == true {
            return 22986;
        }
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == false {
            return 22987;
        }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 1 {
            return 22976;
        }
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 3 {
            return 22983;
        }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == false {
            return 22979;
        }
        if self.r#candles == 3 && self.r#waterlogged == false && self.r#lit == false {
            return 22985;
        }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == true {
            return 22978;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22980 {
            return Some(YellowCandle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22975 {
            return Some(YellowCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 22974 {
            return Some(YellowCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 22982 {
            return Some(YellowCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 22988 {
            return Some(YellowCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22981 {
            return Some(YellowCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 22989 {
            return Some(YellowCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22977 {
            return Some(YellowCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 22984 {
            return Some(YellowCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22986 {
            return Some(YellowCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22987 {
            return Some(YellowCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22976 {
            return Some(YellowCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 22983 {
            return Some(YellowCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 22979 {
            return Some(YellowCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22985 {
            return Some(YellowCandle {
                r#candles: 3,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 22978 {
            return Some(YellowCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: true,
            });
        }
        return None;
    }
}
