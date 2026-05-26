use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackCandle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}

impl BlockState for BlackCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 4 && self.r#waterlogged == true && self.r#lit == false {
            return 23164;
        }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 2 {
            return 23156;
        }
        if self.r#lit == true && self.r#candles == 1 && self.r#waterlogged == false {
            return 23151;
        }
        if self.r#waterlogged == false && self.r#candles == 1 && self.r#lit == false {
            return 23153;
        }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == false {
            return 23155;
        }
        if self.r#waterlogged == false && self.r#candles == 2 && self.r#lit == false {
            return 23157;
        }
        if self.r#waterlogged == true && self.r#candles == 3 && self.r#lit == false {
            return 23160;
        }
        if self.r#lit == true && self.r#candles == 4 && self.r#waterlogged == true {
            return 23162;
        }
        if self.r#lit == false && self.r#candles == 3 && self.r#waterlogged == false {
            return 23161;
        }
        if self.r#candles == 3 && self.r#waterlogged == true && self.r#lit == true {
            return 23158;
        }
        if self.r#candles == 2 && self.r#waterlogged == true && self.r#lit == true {
            return 23154;
        }
        if self.r#lit == true && self.r#candles == 4 && self.r#waterlogged == false {
            return 23163;
        }
        if self.r#waterlogged == true && self.r#lit == true && self.r#candles == 1 {
            return 23150;
        }
        if self.r#candles == 1 && self.r#lit == false && self.r#waterlogged == true {
            return 23152;
        }
        if self.r#waterlogged == false && self.r#candles == 3 && self.r#lit == true {
            return 23159;
        }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 4 {
            return 23165;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23164 {
            return Some(BlackCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23156 {
            return Some(BlackCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 23151 {
            return Some(BlackCandle {
                r#lit: true,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 23153 {
            return Some(BlackCandle {
                r#waterlogged: false,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 23155 {
            return Some(BlackCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23157 {
            return Some(BlackCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23160 {
            return Some(BlackCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 23162 {
            return Some(BlackCandle {
                r#lit: true,
                r#candles: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 23161 {
            return Some(BlackCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 23158 {
            return Some(BlackCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23154 {
            return Some(BlackCandle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 23163 {
            return Some(BlackCandle {
                r#lit: true,
                r#candles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 23150 {
            return Some(BlackCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 23152 {
            return Some(BlackCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23159 {
            return Some(BlackCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 23165 {
            return Some(BlackCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        return None;
    }
}
