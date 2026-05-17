use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeCandle {
    pub candles: i32,
    pub waterlogged: bool,
    pub lit: bool,
}


impl BlockState for OrangeCandle {
    fn to_id(&self) -> i32 {
        if self.r#lit == true && self.r#waterlogged == false && self.r#candles == 2 { return 22931; }
        if self.r#waterlogged == true && self.r#candles == 4 && self.r#lit == false { return 22940; }
        if self.r#waterlogged == true && self.r#lit == false && self.r#candles == 3 { return 22936; }
        if self.r#lit == true && self.r#candles == 3 && self.r#waterlogged == true { return 22934; }
        if self.r#waterlogged == false && self.r#lit == false && self.r#candles == 3 { return 22937; }
        if self.r#candles == 2 && self.r#lit == false && self.r#waterlogged == true { return 22932; }
        if self.r#waterlogged == false && self.r#candles == 2 && self.r#lit == false { return 22933; }
        if self.r#waterlogged == false && self.r#lit == true && self.r#candles == 3 { return 22935; }
        if self.r#candles == 2 && self.r#lit == true && self.r#waterlogged == true { return 22930; }
        if self.r#candles == 1 && self.r#lit == true && self.r#waterlogged == true { return 22926; }
        if self.r#candles == 1 && self.r#waterlogged == false && self.r#lit == false { return 22929; }
        if self.r#candles == 4 && self.r#waterlogged == false && self.r#lit == false { return 22941; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#candles == 1 { return 22928; }
        if self.r#candles == 4 && self.r#waterlogged == false && self.r#lit == true { return 22939; }
        if self.r#candles == 4 && self.r#waterlogged == true && self.r#lit == true { return 22938; }
        if self.r#candles == 1 && self.r#waterlogged == false && self.r#lit == true { return 22927; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22931 {
            return Some(OrangeCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 22940 {
            return Some(OrangeCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22936 {
            return Some(OrangeCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 3,
            });
        }
        if state_id == 22934 {
            return Some(OrangeCandle {
                r#lit: true,
                r#candles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 22937 {
            return Some(OrangeCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 3,
            });
        }
        if state_id == 22932 {
            return Some(OrangeCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22933 {
            return Some(OrangeCandle {
                r#waterlogged: false,
                r#candles: 2,
                r#lit: false,
            });
        }
        if state_id == 22935 {
            return Some(OrangeCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 3,
            });
        }
        if state_id == 22930 {
            return Some(OrangeCandle {
                r#candles: 2,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22926 {
            return Some(OrangeCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22929 {
            return Some(OrangeCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 22941 {
            return Some(OrangeCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 22928 {
            return Some(OrangeCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 1,
            });
        }
        if state_id == 22939 {
            return Some(OrangeCandle {
                r#candles: 4,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22938 {
            return Some(OrangeCandle {
                r#candles: 4,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 22927 {
            return Some(OrangeCandle {
                r#candles: 1,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        return None;
    }
}

