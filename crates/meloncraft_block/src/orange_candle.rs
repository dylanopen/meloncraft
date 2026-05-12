use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeCandle {
    pub lit: bool,
    pub candles: i32,
    pub waterlogged: bool,
}


impl BlockState for OrangeCandle {
    fn to_id(self) -> i32 {
        if block_state.r#candles == 3 && block_state.r#waterlogged == true && block_state.r#lit == true { return 22934; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == false { return 22927; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 4 { return 22939; }
        if block_state.r#candles == 1 && block_state.r#lit == true && block_state.r#waterlogged == true { return 22926; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == false && block_state.r#lit == true { return 22931; }
        if block_state.r#candles == 4 && block_state.r#lit == false && block_state.r#waterlogged == true { return 22940; }
        if block_state.r#waterlogged == false && block_state.r#candles == 4 && block_state.r#lit == false { return 22941; }
        if block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#candles == 3 { return 22935; }
        if block_state.r#waterlogged == true && block_state.r#candles == 1 && block_state.r#lit == false { return 22928; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 2 { return 22933; }
        if block_state.r#waterlogged == true && block_state.r#candles == 4 && block_state.r#lit == true { return 22938; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#candles == 3 { return 22936; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == false { return 22937; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#candles == 1 { return 22929; }
        if block_state.r#lit == true && block_state.r#candles == 2 && block_state.r#waterlogged == true { return 22930; }
        if block_state.r#candles == 2 && block_state.r#lit == false && block_state.r#waterlogged == true { return 22932; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22934 {
            return Some(OrangeCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 22927 {
            return Some(OrangeCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22939 {
            return Some(OrangeCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 4,
            });
        }
        if state_id == 22926 {
            return Some(OrangeCandle {
                r#candles: 1,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22931 {
            return Some(OrangeCandle {
                r#candles: 2,
                r#waterlogged: false,
                r#lit: true,
            });
        }
        if state_id == 22940 {
            return Some(OrangeCandle {
                r#candles: 4,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22941 {
            return Some(OrangeCandle {
                r#waterlogged: false,
                r#candles: 4,
                r#lit: false,
            });
        }
        if state_id == 22935 {
            return Some(OrangeCandle {
                r#lit: true,
                r#waterlogged: false,
                r#candles: 3,
            });
        }
        if state_id == 22928 {
            return Some(OrangeCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: false,
            });
        }
        if state_id == 22933 {
            return Some(OrangeCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 2,
            });
        }
        if state_id == 22938 {
            return Some(OrangeCandle {
                r#waterlogged: true,
                r#candles: 4,
                r#lit: true,
            });
        }
        if state_id == 22936 {
            return Some(OrangeCandle {
                r#lit: false,
                r#waterlogged: true,
                r#candles: 3,
            });
        }
        if state_id == 22937 {
            return Some(OrangeCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 22929 {
            return Some(OrangeCandle {
                r#lit: false,
                r#waterlogged: false,
                r#candles: 1,
            });
        }
        if state_id == 22930 {
            return Some(OrangeCandle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 22932 {
            return Some(OrangeCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

