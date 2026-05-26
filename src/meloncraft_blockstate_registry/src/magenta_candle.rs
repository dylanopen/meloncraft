use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaCandle {
    pub candles: i32,
    pub waterlogged: bool,
    pub lit: bool,
}

impl BlockState for MagentaCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 3 && self.r#lit == false && self.r#waterlogged == false {
            return 22953;
        }
        if self.r#candles == 1 && self.r#lit == false && self.r#waterlogged == false {
            return 22945;
        }
        if self.r#candles == 2 && self.r#waterlogged == true && self.r#lit == false {
            return 22948;
        }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == true {
            return 22954;
        }
        if self.r#waterlogged == true && self.r#lit == true && self.r#candles == 3 {
            return 22950;
        }
        if self.r#candles == 4 && self.r#lit == false && self.r#waterlogged == false {
            return 22957;
        }
        if self.r#waterlogged == false && self.r#lit == true && self.r#candles == 2 {
            return 22947;
        }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == false {
            return 22956;
        }
        if self.r#candles == 1 && self.r#lit == false && self.r#waterlogged == true {
            return 22944;
        }
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == false {
            return 22951;
        }
        if self.r#candles == 4 && self.r#waterlogged == false && self.r#lit == true {
            return 22955;
        }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == false {
            return 22943;
        }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 2 {
            return 22946;
        }
        if self.r#candles == 3 && self.r#waterlogged == true && self.r#lit == false {
            return 22952;
        }
        if self.r#candles == 2 && self.r#waterlogged == false && self.r#lit == false {
            return 22949;
        }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 1 {
            return 22942;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22953 {
            return Some(MagentaCandle {
                r#candles: 3,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22945 {
            return Some(MagentaCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22948 {
            return Some(MagentaCandle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22954 {
            return Some(MagentaCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 22950 {
            return Some(MagentaCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 3,
            });
        }
        if state_id == 22957 {
            return Some(MagentaCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22947 {
            return Some(MagentaCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 2,
            });
        }
        if state_id == 22956 {
            return Some(MagentaCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22944 {
            return Some(MagentaCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22951 {
            return Some(MagentaCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22955 {
            return Some(MagentaCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22943 {
            return Some(MagentaCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22946 {
            return Some(MagentaCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 2,
            });
        }
        if state_id == 22952 {
            return Some(MagentaCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 22949 {
            return Some(MagentaCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 22942 {
            return Some(MagentaCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        return None;
    }
}
