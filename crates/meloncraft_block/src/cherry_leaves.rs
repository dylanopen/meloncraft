use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryLeaves {
    pub waterlogged: bool,
    pub distance: i32,
    pub persistent: bool,
}


impl BlockState for CherryLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 6 { return 413; }
        if block_state.r#distance == 1 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 392; }
        if block_state.r#waterlogged == true && block_state.r#distance == 6 && block_state.r#persistent == false { return 414; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 7 { return 418; }
        if block_state.r#distance == 2 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 398; }
        if block_state.r#distance == 2 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 397; }
        if block_state.r#waterlogged == true && block_state.r#distance == 3 && block_state.r#persistent == true { return 400; }
        if block_state.r#distance == 3 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 403; }
        if block_state.r#waterlogged == false && block_state.r#distance == 4 && block_state.r#persistent == true { return 405; }
        if block_state.r#distance == 4 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 406; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 1 { return 395; }
        if block_state.r#distance == 1 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 394; }
        if block_state.r#persistent == true && block_state.r#distance == 5 && block_state.r#waterlogged == true { return 408; }
        if block_state.r#distance == 3 && block_state.r#waterlogged == false && block_state.r#persistent == true { return 401; }
        if block_state.r#distance == 1 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 393; }
        if block_state.r#distance == 5 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 410; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 4 { return 404; }
        if block_state.r#distance == 6 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 412; }
        if block_state.r#waterlogged == false && block_state.r#distance == 5 && block_state.r#persistent == true { return 409; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 415; }
        if block_state.r#distance == 7 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 416; }
        if block_state.r#waterlogged == false && block_state.r#distance == 2 && block_state.r#persistent == false { return 399; }
        if block_state.r#distance == 4 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 407; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 7 { return 417; }
        if block_state.r#distance == 5 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 411; }
        if block_state.r#waterlogged == false && block_state.r#distance == 7 && block_state.r#persistent == false { return 419; }
        if block_state.r#distance == 2 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 396; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == true { return 402; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 413 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 392 {
            return Some(CherryLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 414 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#distance: 6,
                r#persistent: false,
            });
        }
        if state_id == 418 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 7,
            });
        }
        if state_id == 398 {
            return Some(CherryLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 397 {
            return Some(CherryLeaves {
                r#distance: 2,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 400 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 403 {
            return Some(CherryLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 405 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: true,
            });
        }
        if state_id == 406 {
            return Some(CherryLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 395 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 394 {
            return Some(CherryLeaves {
                r#distance: 1,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 408 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#distance: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 401 {
            return Some(CherryLeaves {
                r#distance: 3,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 393 {
            return Some(CherryLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 410 {
            return Some(CherryLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 404 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 412 {
            return Some(CherryLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 409 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 415 {
            return Some(CherryLeaves {
                r#distance: 6,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 416 {
            return Some(CherryLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 399 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 407 {
            return Some(CherryLeaves {
                r#distance: 4,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 417 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 411 {
            return Some(CherryLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 419 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 396 {
            return Some(CherryLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 402 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

