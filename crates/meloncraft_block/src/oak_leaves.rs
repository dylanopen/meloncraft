use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakLeaves {
    pub waterlogged: bool,
    pub distance: i32,
    pub persistent: bool,
}


impl BlockState for OakLeaves {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#distance == 7 && self.r#persistent == false { return 279; }
        if self.r#waterlogged == true && self.r#distance == 7 && self.r#persistent == false { return 278; }
        if self.r#distance == 6 && self.r#persistent == false && self.r#waterlogged == true { return 274; }
        if self.r#persistent == true && self.r#distance == 2 && self.r#waterlogged == true { return 256; }
        if self.r#waterlogged == true && self.r#distance == 5 && self.r#persistent == true { return 268; }
        if self.r#persistent == false && self.r#distance == 3 && self.r#waterlogged == false { return 263; }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 3 { return 262; }
        if self.r#distance == 1 && self.r#persistent == true && self.r#waterlogged == true { return 252; }
        if self.r#distance == 1 && self.r#waterlogged == false && self.r#persistent == false { return 255; }
        if self.r#distance == 2 && self.r#waterlogged == false && self.r#persistent == true { return 257; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 3 { return 261; }
        if self.r#persistent == false && self.r#distance == 2 && self.r#waterlogged == false { return 259; }
        if self.r#persistent == false && self.r#distance == 1 && self.r#waterlogged == true { return 254; }
        if self.r#persistent == false && self.r#distance == 5 && self.r#waterlogged == true { return 270; }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 5 { return 271; }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 3 { return 260; }
        if self.r#waterlogged == false && self.r#distance == 5 && self.r#persistent == true { return 269; }
        if self.r#waterlogged == false && self.r#distance == 4 && self.r#persistent == true { return 265; }
        if self.r#distance == 6 && self.r#waterlogged == true && self.r#persistent == true { return 272; }
        if self.r#distance == 4 && self.r#persistent == false && self.r#waterlogged == true { return 266; }
        if self.r#persistent == true && self.r#distance == 6 && self.r#waterlogged == false { return 273; }
        if self.r#distance == 4 && self.r#waterlogged == true && self.r#persistent == true { return 264; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 1 { return 253; }
        if self.r#waterlogged == true && self.r#distance == 2 && self.r#persistent == false { return 258; }
        if self.r#distance == 7 && self.r#waterlogged == true && self.r#persistent == true { return 276; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 7 { return 277; }
        if self.r#waterlogged == false && self.r#distance == 4 && self.r#persistent == false { return 267; }
        if self.r#distance == 6 && self.r#persistent == false && self.r#waterlogged == false { return 275; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 279 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 278 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 274 {
            return Some(OakLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 256 {
            return Some(OakLeaves {
                r#persistent: true,
                r#distance: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 268 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 263 {
            return Some(OakLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 262 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 3,
            });
        }
        if state_id == 252 {
            return Some(OakLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 255 {
            return Some(OakLeaves {
                r#distance: 1,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 257 {
            return Some(OakLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 261 {
            return Some(OakLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 259 {
            return Some(OakLeaves {
                r#persistent: false,
                r#distance: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 254 {
            return Some(OakLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 270 {
            return Some(OakLeaves {
                r#persistent: false,
                r#distance: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 271 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 5,
            });
        }
        if state_id == 260 {
            return Some(OakLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 3,
            });
        }
        if state_id == 269 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 265 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: true,
            });
        }
        if state_id == 272 {
            return Some(OakLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 266 {
            return Some(OakLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 273 {
            return Some(OakLeaves {
                r#persistent: true,
                r#distance: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 264 {
            return Some(OakLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 253 {
            return Some(OakLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 258 {
            return Some(OakLeaves {
                r#waterlogged: true,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 276 {
            return Some(OakLeaves {
                r#distance: 7,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 277 {
            return Some(OakLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 7,
            });
        }
        if state_id == 267 {
            return Some(OakLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: false,
            });
        }
        if state_id == 275 {
            return Some(OakLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

