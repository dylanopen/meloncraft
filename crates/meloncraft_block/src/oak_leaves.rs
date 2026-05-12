use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakLeaves {
    pub persistent: bool,
    pub waterlogged: bool,
    pub distance: i32,
}


impl BlockState for OakLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#distance == 5 && block_state.r#persistent == false { return 271; }
        if block_state.r#waterlogged == false && block_state.r#distance == 2 && block_state.r#persistent == false { return 259; }
        if block_state.r#waterlogged == true && block_state.r#distance == 4 && block_state.r#persistent == true { return 264; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 6 { return 274; }
        if block_state.r#waterlogged == false && block_state.r#distance == 7 && block_state.r#persistent == true { return 277; }
        if block_state.r#distance == 7 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 276; }
        if block_state.r#waterlogged == false && block_state.r#distance == 3 && block_state.r#persistent == false { return 263; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 5 { return 268; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 7 { return 278; }
        if block_state.r#waterlogged == false && block_state.r#distance == 3 && block_state.r#persistent == true { return 261; }
        if block_state.r#distance == 6 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 273; }
        if block_state.r#waterlogged == true && block_state.r#persistent == true && block_state.r#distance == 2 { return 256; }
        if block_state.r#distance == 6 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 275; }
        if block_state.r#persistent == false && block_state.r#distance == 4 && block_state.r#waterlogged == false { return 267; }
        if block_state.r#distance == 1 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 254; }
        if block_state.r#waterlogged == false && block_state.r#persistent == false && block_state.r#distance == 1 { return 255; }
        if block_state.r#waterlogged == true && block_state.r#persistent == true && block_state.r#distance == 3 { return 260; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == true { return 262; }
        if block_state.r#waterlogged == true && block_state.r#distance == 2 && block_state.r#persistent == false { return 258; }
        if block_state.r#distance == 4 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 265; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 4 { return 266; }
        if block_state.r#distance == 5 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 270; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 1 { return 252; }
        if block_state.r#persistent == true && block_state.r#distance == 6 && block_state.r#waterlogged == true { return 272; }
        if block_state.r#waterlogged == false && block_state.r#distance == 2 && block_state.r#persistent == true { return 257; }
        if block_state.r#waterlogged == false && block_state.r#distance == 7 && block_state.r#persistent == false { return 279; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 1 { return 253; }
        if block_state.r#distance == 5 && block_state.r#waterlogged == false && block_state.r#persistent == true { return 269; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 271 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 259 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 264 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#distance: 4,
                r#persistent: true,
            });
        }
        if state_id == 274 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 6,
            });
        }
        if state_id == 277 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 276 {
            return Some(OakLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 263 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 3,
                r#persistent: false,
            });
        }
        if state_id == 268 {
            return Some(OakLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 278 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 7,
            });
        }
        if state_id == 261 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 273 {
            return Some(OakLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 256 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 2,
            });
        }
        if state_id == 275 {
            return Some(OakLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 267 {
            return Some(OakLeaves {
                r#persistent: false,
                r#distance: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 254 {
            return Some(OakLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 255 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 1,
            });
        }
        if state_id == 260 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 3,
            });
        }
        if state_id == 262 {
            return Some(OakLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 258 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 265 {
            return Some(OakLeaves {
                r#distance: 4,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 266 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 4,
            });
        }
        if state_id == 270 {
            return Some(OakLeaves {
                r#distance: 5,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 252 {
            return Some(OakLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 1,
            });
        }
        if state_id == 272 {
            return Some(OakLeaves {
                r#persistent: true,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 257 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: true,
            });
        }
        if state_id == 279 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 253 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 1,
            });
        }
        if state_id == 269 {
            return Some(OakLeaves {
                r#distance: 5,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        return None;
    }
}

