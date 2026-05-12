use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveLeaves {
    pub waterlogged: bool,
    pub persistent: bool,
    pub distance: i32,
}


impl BlockState for MangroveLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == true { return 486; }
        if block_state.r#waterlogged == true && block_state.r#persistent == true && block_state.r#distance == 2 { return 480; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 4 { return 491; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == false { return 487; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 4 { return 488; }
        if block_state.r#waterlogged == true && block_state.r#distance == 5 && block_state.r#persistent == true { return 492; }
        if block_state.r#waterlogged == true && block_state.r#distance == 1 && block_state.r#persistent == true { return 476; }
        if block_state.r#distance == 5 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 493; }
        if block_state.r#distance == 2 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 483; }
        if block_state.r#distance == 5 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 495; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 5 { return 494; }
        if block_state.r#persistent == true && block_state.r#distance == 7 && block_state.r#waterlogged == false { return 501; }
        if block_state.r#distance == 1 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 478; }
        if block_state.r#distance == 2 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 482; }
        if block_state.r#distance == 2 && block_state.r#waterlogged == false && block_state.r#persistent == true { return 481; }
        if block_state.r#distance == 3 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 484; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 4 { return 489; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 6 { return 497; }
        if block_state.r#waterlogged == false && block_state.r#distance == 6 && block_state.r#persistent == false { return 499; }
        if block_state.r#waterlogged == false && block_state.r#distance == 3 && block_state.r#persistent == true { return 485; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 4 { return 490; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 7 { return 500; }
        if block_state.r#distance == 1 && block_state.r#waterlogged == false && block_state.r#persistent == true { return 477; }
        if block_state.r#waterlogged == true && block_state.r#distance == 6 && block_state.r#persistent == false { return 498; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 7 { return 502; }
        if block_state.r#persistent == false && block_state.r#distance == 1 && block_state.r#waterlogged == false { return 479; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 6 { return 496; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 7 { return 503; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 486 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 480 {
            return Some(MangroveLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 2,
            });
        }
        if state_id == 491 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 4,
            });
        }
        if state_id == 487 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 488 {
            return Some(MangroveLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 492 {
            return Some(MangroveLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 476 {
            return Some(MangroveLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: true,
            });
        }
        if state_id == 493 {
            return Some(MangroveLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 483 {
            return Some(MangroveLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 495 {
            return Some(MangroveLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 494 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 501 {
            return Some(MangroveLeaves {
                r#persistent: true,
                r#distance: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 478 {
            return Some(MangroveLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 482 {
            return Some(MangroveLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 481 {
            return Some(MangroveLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 484 {
            return Some(MangroveLeaves {
                r#distance: 3,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 489 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 4,
            });
        }
        if state_id == 497 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 6,
            });
        }
        if state_id == 499 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#distance: 6,
                r#persistent: false,
            });
        }
        if state_id == 485 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 490 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 500 {
            return Some(MangroveLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 7,
            });
        }
        if state_id == 477 {
            return Some(MangroveLeaves {
                r#distance: 1,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 498 {
            return Some(MangroveLeaves {
                r#waterlogged: true,
                r#distance: 6,
                r#persistent: false,
            });
        }
        if state_id == 502 {
            return Some(MangroveLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 7,
            });
        }
        if state_id == 479 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 496 {
            return Some(MangroveLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 503 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 7,
            });
        }
        return None;
    }
}

