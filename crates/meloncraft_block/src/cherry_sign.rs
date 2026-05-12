use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherrySign {
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for CherrySign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5280; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false { return 5279; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5274; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 5269; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5288; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 0 { return 5263; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5267; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 5278; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 5287; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 { return 5292; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5289; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 11 { return 5285; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5281; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5283; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 5284; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5265; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 4 { return 5270; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 5276; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5272; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 { return 5277; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 0 { return 5262; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5264; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 5282; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 5290; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 { return 5291; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 5266; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 5273; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5286; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 5268; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 5275; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 5293; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 5271; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5280 {
            return Some(CherrySign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5279 {
            return Some(CherrySign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5274 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5269 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5288 {
            return Some(CherrySign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5263 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5267 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5278 {
            return Some(CherrySign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5287 {
            return Some(CherrySign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5292 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5289 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5285 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5281 {
            return Some(CherrySign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5283 {
            return Some(CherrySign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5284 {
            return Some(CherrySign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5265 {
            return Some(CherrySign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5270 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 5276 {
            return Some(CherrySign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5272 {
            return Some(CherrySign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5277 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5262 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 5264 {
            return Some(CherrySign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5282 {
            return Some(CherrySign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5290 {
            return Some(CherrySign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5291 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5266 {
            return Some(CherrySign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5273 {
            return Some(CherrySign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5286 {
            return Some(CherrySign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5268 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5275 {
            return Some(CherrySign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 5293 {
            return Some(CherrySign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5271 {
            return Some(CherrySign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

