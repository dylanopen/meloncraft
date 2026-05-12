use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchLeaves {
    pub waterlogged: bool,
    pub distance: i32,
    pub persistent: bool,
}


impl BlockState for BirchLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#distance == 1 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 311; }
        if block_state.r#distance == 6 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 331; }
        if block_state.r#distance == 7 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 332; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 7 { return 334; }
        if block_state.r#persistent == true && block_state.r#distance == 6 && block_state.r#waterlogged == true { return 328; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 7 { return 333; }
        if block_state.r#distance == 1 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 309; }
        if block_state.r#distance == 2 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 315; }
        if block_state.r#waterlogged == true && block_state.r#distance == 3 && block_state.r#persistent == true { return 316; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 3 { return 318; }
        if block_state.r#persistent == false && block_state.r#distance == 1 && block_state.r#waterlogged == true { return 310; }
        if block_state.r#waterlogged == true && block_state.r#distance == 2 && block_state.r#persistent == true { return 312; }
        if block_state.r#waterlogged == false && block_state.r#distance == 4 && block_state.r#persistent == false { return 323; }
        if block_state.r#distance == 5 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 324; }
        if block_state.r#distance == 6 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 330; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 6 { return 329; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 7 { return 335; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 1 { return 308; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 4 { return 321; }
        if block_state.r#waterlogged == false && block_state.r#distance == 2 && block_state.r#persistent == true { return 313; }
        if block_state.r#persistent == true && block_state.r#distance == 4 && block_state.r#waterlogged == true { return 320; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 3 { return 319; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 2 { return 314; }
        if block_state.r#persistent == true && block_state.r#distance == 5 && block_state.r#waterlogged == false { return 325; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 5 { return 327; }
        if block_state.r#waterlogged == true && block_state.r#distance == 5 && block_state.r#persistent == false { return 326; }
        if block_state.r#waterlogged == false && block_state.r#distance == 3 && block_state.r#persistent == true { return 317; }
        if block_state.r#distance == 4 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 322; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 311 {
            return Some(BirchLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 331 {
            return Some(BirchLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 332 {
            return Some(BirchLeaves {
                r#distance: 7,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 334 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 7,
            });
        }
        if state_id == 328 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 333 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 309 {
            return Some(BirchLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 315 {
            return Some(BirchLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 316 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 318 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 3,
            });
        }
        if state_id == 310 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 312 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#distance: 2,
                r#persistent: true,
            });
        }
        if state_id == 323 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: false,
            });
        }
        if state_id == 324 {
            return Some(BirchLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 330 {
            return Some(BirchLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 329 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 6,
            });
        }
        if state_id == 335 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 7,
            });
        }
        if state_id == 308 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 1,
            });
        }
        if state_id == 321 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 4,
            });
        }
        if state_id == 313 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: true,
            });
        }
        if state_id == 320 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 319 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 314 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 2,
            });
        }
        if state_id == 325 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#distance: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 327 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 5,
            });
        }
        if state_id == 326 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 317 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 322 {
            return Some(BirchLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        return None;
    }
}

