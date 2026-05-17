use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueCandle {
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}


impl BlockState for BlueCandle {
    fn to_id(&self) -> i32 {
        if self.r#candles == 2 && self.r#waterlogged == false && self.r#lit == true { return 23091; }
        if self.r#candles == 1 && self.r#lit == false && self.r#waterlogged == false { return 23089; }
        if self.r#waterlogged == true && self.r#candles == 3 && self.r#lit == true { return 23094; }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == true { return 23086; }
        if self.r#waterlogged == true && self.r#candles == 1 && self.r#lit == false { return 23088; }
        if self.r#candles == 3 && self.r#waterlogged == false && self.r#lit == false { return 23097; }
        if self.r#waterlogged == true && self.r#candles == 2 && self.r#lit == false { return 23092; }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == true { return 23098; }
        if self.r#candles == 3 && self.r#lit == true && self.r#waterlogged == false { return 23095; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 2 { return 23093; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == true { return 23090; }
        if self.r#candles == 3 && self.r#waterlogged == true && self.r#lit == false { return 23096; }
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == false { return 23099; }
        if self.r#candles == 4 && self.r#waterlogged == true && self.r#lit == false { return 23100; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#candles == 4 { return 23101; }
        if self.r#lit == true && self.r#candles == 1 && self.r#waterlogged == false { return 23087; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23091 {
            return Some(BlueCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 23089 {
            return Some(BlueCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23094 {
            return Some(BlueCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 23086 {
            return Some(BlueCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23088 {
            return Some(BlueCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 23097 {
            return Some(BlueCandle {
                r#candles: 3,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 23092 {
            return Some(BlueCandle {
                r#waterlogged: true,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 23098 {
            return Some(BlueCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 23095 {
            return Some(BlueCandle {
                r#candles: 3,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23093 {
            return Some(BlueCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 23090 {
            return Some(BlueCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23096 {
            return Some(BlueCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23099 {
            return Some(BlueCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23100 {
            return Some(BlueCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 23101 {
            return Some(BlueCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 23087 {
            return Some(BlueCandle {
                r#lit: true,
                r#candles: 1,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

