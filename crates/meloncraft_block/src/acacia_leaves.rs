use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaLeaves {
    pub persistent: bool,
    pub distance: i32,
    pub waterlogged: bool,
}


impl BlockState for AcaciaLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#persistent == false && block_state.r#distance == 7 && block_state.r#waterlogged == true { return 390; }
        if block_state.r#persistent == false && block_state.r#distance == 4 && block_state.r#waterlogged == true { return 378; }
        if block_state.r#waterlogged == true && block_state.r#distance == 7 && block_state.r#persistent == true { return 388; }
        if block_state.r#distance == 1 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 366; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == false && block_state.r#persistent == true { return 385; }
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 2 { return 369; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 6 { return 386; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == true { return 374; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 1 { return 367; }
        if block_state.r#waterlogged == true && block_state.r#persistent == true && block_state.r#distance == 1 { return 364; }
        if block_state.r#distance == 1 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 365; }
        if block_state.r#distance == 3 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 372; }
        if block_state.r#distance == 3 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 373; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == false { return 375; }
        if block_state.r#waterlogged == true && block_state.r#distance == 5 && block_state.r#persistent == false { return 382; }
        if block_state.r#waterlogged == false && block_state.r#distance == 5 && block_state.r#persistent == false { return 383; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 384; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 6 { return 387; }
        if block_state.r#distance == 2 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 371; }
        if block_state.r#waterlogged == true && block_state.r#distance == 5 && block_state.r#persistent == true { return 380; }
        if block_state.r#persistent == true && block_state.r#distance == 7 && block_state.r#waterlogged == false { return 389; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 7 { return 391; }
        if block_state.r#distance == 4 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 376; }
        if block_state.r#distance == 4 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 379; }
        if block_state.r#distance == 2 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 368; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 5 { return 381; }
        if block_state.r#waterlogged == false && block_state.r#distance == 4 && block_state.r#persistent == true { return 377; }
        if block_state.r#waterlogged == true && block_state.r#distance == 2 && block_state.r#persistent == false { return 370; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 390 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 378 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 388 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 366 {
            return Some(AcaciaLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 385 {
            return Some(AcaciaLeaves {
                r#distance: 6,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 369 {
            return Some(AcaciaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 2,
            });
        }
        if state_id == 386 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 6,
            });
        }
        if state_id == 374 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 367 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 364 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 1,
            });
        }
        if state_id == 365 {
            return Some(AcaciaLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 372 {
            return Some(AcaciaLeaves {
                r#distance: 3,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 373 {
            return Some(AcaciaLeaves {
                r#distance: 3,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 375 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 382 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 383 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 384 {
            return Some(AcaciaLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 387 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 371 {
            return Some(AcaciaLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 380 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 389 {
            return Some(AcaciaLeaves {
                r#persistent: true,
                r#distance: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 391 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 7,
            });
        }
        if state_id == 376 {
            return Some(AcaciaLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 379 {
            return Some(AcaciaLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 368 {
            return Some(AcaciaLeaves {
                r#distance: 2,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 381 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 5,
            });
        }
        if state_id == 377 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: true,
            });
        }
        if state_id == 370 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#distance: 2,
                r#persistent: false,
            });
        }
        return None;
    }
}

