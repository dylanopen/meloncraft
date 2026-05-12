use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueCandle {
    pub waterlogged: bool,
    pub candles: i32,
    pub lit: bool,
}


impl BlockState for LightBlueCandle {
    fn to_id(&self) -> i32 {
        if self.r#lit == true && self.r#candles == 3 && self.r#waterlogged == false { return 22967; }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == true { return 22964; }
        if self.r#candles == 1 && self.r#lit == false && self.r#waterlogged == true { return 22960; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 4 { return 22972; }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == false { return 22959; }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == true { return 22958; }
        if self.r#waterlogged == false && self.r#candles == 3 && self.r#lit == false { return 22969; }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 3 { return 22966; }
        if self.r#candles == 1 && self.r#lit == false && self.r#waterlogged == false { return 22961; }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == false { return 22965; }
        if self.r#lit == true && self.r#waterlogged == true && self.r#candles == 4 { return 22970; }
        if self.r#waterlogged == false && self.r#candles == 4 && self.r#lit == false { return 22973; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == true { return 22962; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 3 { return 22968; }
        if self.r#candles == 2 && self.r#waterlogged == false && self.r#lit == true { return 22963; }
        if self.r#candles == 4 && self.r#lit == true && self.r#waterlogged == false { return 22971; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22967 {
            return Some(LightBlueCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 22964 {
            return Some(LightBlueCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22960 {
            return Some(LightBlueCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22972 {
            return Some(LightBlueCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 22959 {
            return Some(LightBlueCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22958 {
            return Some(LightBlueCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22969 {
            return Some(LightBlueCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 22966 {
            return Some(LightBlueCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 22961 {
            return Some(LightBlueCandle {
                r#candles: 1,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22965 {
            return Some(LightBlueCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22970 {
            return Some(LightBlueCandle {
                r#lit: true,
                r#waterlogged: true,
                r#candles: 4,
            });
        }
        if state_id == 22973 {
            return Some(LightBlueCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22962 {
            return Some(LightBlueCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22968 {
            return Some(LightBlueCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 22963 {
            return Some(LightBlueCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22971 {
            return Some(LightBlueCandle {
                r#candles: 4,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

