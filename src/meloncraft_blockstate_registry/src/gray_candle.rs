use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayCandle {
    pub lit: bool,
    pub candles: i32,
    pub waterlogged: bool,
}

impl BlockState for GrayCandle {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == false {
            return 23036;
        }
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 3 {
            return 23031;
        }
        if self.r#waterlogged == false && self.r#lit == false && self.r#candles == 3 {
            return 23033;
        }
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == true {
            return 23022;
        }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == true {
            return 23034;
        }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == false {
            return 23028;
        }
        if self.r#waterlogged == false && self.r#candles == 4 && self.r#lit == true {
            return 23035;
        }
        if self.r#lit == true && self.r#candles == 2 && self.r#waterlogged == true {
            return 23026;
        }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 3 {
            return 23030;
        }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 1 {
            return 23024;
        }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == false {
            return 23029;
        }
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 1 {
            return 23023;
        }
        if self.r#waterlogged == false && self.r#lit == false && self.r#candles == 1 {
            return 23025;
        }
        if self.r#waterlogged == false && self.r#candles == 2 && self.r#lit == true {
            return 23027;
        }
        if self.r#waterlogged == false && self.r#lit == false && self.r#candles == 4 {
            return 23037;
        }
        if self.r#lit == false && self.r#candles == 3 && self.r#waterlogged == true {
            return 23032;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23036 {
            return Some(GrayCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 23031 {
            return Some(GrayCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 23033 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 3,
            });
        }
        if state_id == 23022 {
            return Some(GrayCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 23034 {
            return Some(GrayCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23028 {
            return Some(GrayCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23035 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23026 {
            return Some(GrayCandle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 23030 {
            return Some(GrayCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 23024 {
            return Some(GrayCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 23029 {
            return Some(GrayCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23023 {
            return Some(GrayCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 23025 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 23027 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: true,
            });
        }
        if state_id == 23037 {
            return Some(GrayCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 4,
            });
        }
        if state_id == 23032 {
            return Some(GrayCandle {
                r#lit: false,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
