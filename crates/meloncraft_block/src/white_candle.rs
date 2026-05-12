use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteCandle {
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}


impl BlockState for WhiteCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 4 && self.r#waterlogged == false && self.r#lit == true { return 22923; }
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == false { return 22912; }
        if self.r#candles == 2 && self.r#waterlogged == true && self.r#lit == true { return 22914; }
        if self.r#waterlogged == false && self.r#candles == 4 && self.r#lit == false { return 22925; }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == false { return 22917; }
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == false { return 22919; }
        if self.r#candles == 1 && self.r#waterlogged == false && self.r#lit == true { return 22911; }
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == true { return 22918; }
        if self.r#candles == 3 && self.r#lit == false && self.r#waterlogged == true { return 22920; }
        if self.r#waterlogged == false && self.r#candles == 1 && self.r#lit == false { return 22913; }
        if self.r#candles == 3 && self.r#lit == false && self.r#waterlogged == false { return 22921; }
        if self.r#lit == false && self.r#candles == 2 && self.r#waterlogged == true { return 22916; }
        if self.r#candles == 4 && self.r#lit == false && self.r#waterlogged == true { return 22924; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == false { return 22915; }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == true { return 22910; }
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == true { return 22922; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22923 {
            return Some(WhiteCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22912 {
            return Some(WhiteCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 22914 {
            return Some(WhiteCandle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 22925 {
            return Some(WhiteCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22917 {
            return Some(WhiteCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22919 {
            return Some(WhiteCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22911 {
            return Some(WhiteCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22918 {
            return Some(WhiteCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22920 {
            return Some(WhiteCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22913 {
            return Some(WhiteCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 22921 {
            return Some(WhiteCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22916 {
            return Some(WhiteCandle {
                r#lit: false,
                r#candles: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 22924 {
            return Some(WhiteCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22915 {
            return Some(WhiteCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22910 {
            return Some(WhiteCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22922 {
            return Some(WhiteCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

