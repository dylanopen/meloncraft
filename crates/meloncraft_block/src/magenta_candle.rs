use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaCandle {
    pub lit: bool,
    pub waterlogged: bool,
    pub candles: i32,
}


impl BlockState for MagentaCandle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#candles == 3 && block_state.r#lit == false { return 22952; }
        if block_state.r#waterlogged == false && block_state.r#candles == 3 && block_state.r#lit == true { return 22951; }
        if block_state.r#waterlogged == true && block_state.r#candles == 1 && block_state.r#lit == true { return 22942; }
        if block_state.r#candles == 2 && block_state.r#lit == false && block_state.r#waterlogged == false { return 22949; }
        if block_state.r#lit == false && block_state.r#candles == 3 && block_state.r#waterlogged == false { return 22953; }
        if block_state.r#candles == 4 && block_state.r#waterlogged == false && block_state.r#lit == true { return 22955; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#candles == 4 { return 22956; }
        if block_state.r#lit == true && block_state.r#candles == 2 && block_state.r#waterlogged == false { return 22947; }
        if block_state.r#lit == false && block_state.r#candles == 1 && block_state.r#waterlogged == true { return 22944; }
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#candles == 4 { return 22954; }
        if block_state.r#candles == 2 && block_state.r#lit == false && block_state.r#waterlogged == true { return 22948; }
        if block_state.r#candles == 2 && block_state.r#waterlogged == true && block_state.r#lit == true { return 22946; }
        if block_state.r#candles == 3 && block_state.r#waterlogged == true && block_state.r#lit == true { return 22950; }
        if block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#candles == 1 { return 22945; }
        if block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#candles == 1 { return 22943; }
        if block_state.r#lit == false && block_state.r#candles == 4 && block_state.r#waterlogged == false { return 22957; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22952 {
            return Some(MagentaCandle {
                r#waterlogged: true,
                r#candles: 3,
                r#lit: false,
            });
        }
        if state_id == 22951 {
            return Some(MagentaCandle {
                r#waterlogged: false,
                r#candles: 3,
                r#lit: true,
            });
        }
        if state_id == 22942 {
            return Some(MagentaCandle {
                r#waterlogged: true,
                r#candles: 1,
                r#lit: true,
            });
        }
        if state_id == 22949 {
            return Some(MagentaCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22953 {
            return Some(MagentaCandle {
                r#lit: false,
                r#candles: 3,
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
        if state_id == 22956 {
            return Some(MagentaCandle {
                r#waterlogged: true,
                r#lit: false,
                r#candles: 4,
            });
        }
        if state_id == 22947 {
            return Some(MagentaCandle {
                r#lit: true,
                r#candles: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 22944 {
            return Some(MagentaCandle {
                r#lit: false,
                r#candles: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 22954 {
            return Some(MagentaCandle {
                r#waterlogged: true,
                r#lit: true,
                r#candles: 4,
            });
        }
        if state_id == 22948 {
            return Some(MagentaCandle {
                r#candles: 2,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22946 {
            return Some(MagentaCandle {
                r#candles: 2,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 22950 {
            return Some(MagentaCandle {
                r#candles: 3,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        if state_id == 22945 {
            return Some(MagentaCandle {
                r#waterlogged: false,
                r#lit: false,
                r#candles: 1,
            });
        }
        if state_id == 22943 {
            return Some(MagentaCandle {
                r#waterlogged: false,
                r#lit: true,
                r#candles: 1,
            });
        }
        if state_id == 22957 {
            return Some(MagentaCandle {
                r#lit: false,
                r#candles: 4,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

