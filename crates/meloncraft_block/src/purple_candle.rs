use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleCandle {
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}


impl BlockState for PurpleCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == true { return 23082; }
        if self.r#waterlogged == false && self.r#lit == true && self.r#candles == 4 { return 23083; }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == true { return 23076; }
        if self.r#lit == true && self.r#candles == 3 && self.r#waterlogged == true { return 23078; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 2 { return 23077; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == true { return 23074; }
        if self.r#waterlogged == false && self.r#lit == true && self.r#candles == 1 { return 23071; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 3 { return 23080; }
        if self.r#lit == false && self.r#candles == 4 && self.r#waterlogged == false { return 23085; }
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == true { return 23070; }
        if self.r#lit == false && self.r#candles == 1 && self.r#waterlogged == true { return 23072; }
        if self.r#waterlogged == false && self.r#lit == false && self.r#candles == 1 { return 23073; }
        if self.r#lit == false && self.r#candles == 3 && self.r#waterlogged == false { return 23081; }
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 3 { return 23079; }
        if self.r#candles == 4 && self.r#lit == false && self.r#waterlogged == true { return 23084; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == false { return 23075; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23082 {
            return Some(PurpleCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23083 {
            return Some(PurpleCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 4,
            });
        }
        if state_id == 23076 {
            return Some(PurpleCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23078 {
            return Some(PurpleCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 23077 {
            return Some(PurpleCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 23074 {
            return Some(PurpleCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23071 {
            return Some(PurpleCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 23080 {
            return Some(PurpleCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23085 {
            return Some(PurpleCandle {
                r#lit: false,
                r#candles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 23070 {
            return Some(PurpleCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 23072 {
            return Some(PurpleCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 23073 {
            return Some(PurpleCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 23081 {
            return Some(PurpleCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 23079 {
            return Some(PurpleCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 23084 {
            return Some(PurpleCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23075 {
            return Some(PurpleCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

