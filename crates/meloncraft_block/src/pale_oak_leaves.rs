use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakLeaves {
    pub waterlogged: bool,
    pub persistent: bool,
    pub distance: i32,
}


impl BlockState for PaleOakLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#distance == 2 && block_state.r#persistent == false { return 454; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 6 { return 470; }
        if block_state.r#persistent == false && block_state.r#distance == 6 && block_state.r#waterlogged == false { return 471; }
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 4 { return 461; }
        if block_state.r#distance == 7 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 474; }
        if block_state.r#waterlogged == true && block_state.r#distance == 7 && block_state.r#persistent == true { return 472; }
        if block_state.r#persistent == true && block_state.r#distance == 6 && block_state.r#waterlogged == true { return 468; }
        if block_state.r#waterlogged == true && block_state.r#distance == 4 && block_state.r#persistent == true { return 460; }
        if block_state.r#persistent == true && block_state.r#distance == 1 && block_state.r#waterlogged == true { return 448; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 2 { return 453; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 5 { return 467; }
        if block_state.r#distance == 3 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 459; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 7 { return 473; }
        if block_state.r#distance == 6 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 469; }
        if block_state.r#waterlogged == false && block_state.r#distance == 7 && block_state.r#persistent == false { return 475; }
        if block_state.r#distance == 1 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 449; }
        if block_state.r#distance == 1 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 451; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 4 { return 462; }
        if block_state.r#distance == 2 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 452; }
        if block_state.r#distance == 3 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 458; }
        if block_state.r#waterlogged == false && block_state.r#distance == 4 && block_state.r#persistent == false { return 463; }
        if block_state.r#waterlogged == true && block_state.r#persistent == true && block_state.r#distance == 5 { return 464; }
        if block_state.r#distance == 5 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 466; }
        if block_state.r#distance == 3 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 456; }
        if block_state.r#waterlogged == true && block_state.r#distance == 1 && block_state.r#persistent == false { return 450; }
        if block_state.r#distance == 2 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 455; }
        if block_state.r#distance == 3 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 457; }
        if block_state.r#distance == 5 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 465; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 454 {
            return Some(PaleOakLeaves {
                r#waterlogged: true,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 470 {
            return Some(PaleOakLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 471 {
            return Some(PaleOakLeaves {
                r#persistent: false,
                r#distance: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 461 {
            return Some(PaleOakLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 4,
            });
        }
        if state_id == 474 {
            return Some(PaleOakLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 472 {
            return Some(PaleOakLeaves {
                r#waterlogged: true,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 468 {
            return Some(PaleOakLeaves {
                r#persistent: true,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 460 {
            return Some(PaleOakLeaves {
                r#waterlogged: true,
                r#distance: 4,
                r#persistent: true,
            });
        }
        if state_id == 448 {
            return Some(PaleOakLeaves {
                r#persistent: true,
                r#distance: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 453 {
            return Some(PaleOakLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 2,
            });
        }
        if state_id == 467 {
            return Some(PaleOakLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 5,
            });
        }
        if state_id == 459 {
            return Some(PaleOakLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 473 {
            return Some(PaleOakLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 469 {
            return Some(PaleOakLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 475 {
            return Some(PaleOakLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 449 {
            return Some(PaleOakLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 451 {
            return Some(PaleOakLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 462 {
            return Some(PaleOakLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 452 {
            return Some(PaleOakLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 458 {
            return Some(PaleOakLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 463 {
            return Some(PaleOakLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: false,
            });
        }
        if state_id == 464 {
            return Some(PaleOakLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 5,
            });
        }
        if state_id == 466 {
            return Some(PaleOakLeaves {
                r#distance: 5,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 456 {
            return Some(PaleOakLeaves {
                r#distance: 3,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 450 {
            return Some(PaleOakLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: false,
            });
        }
        if state_id == 455 {
            return Some(PaleOakLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 457 {
            return Some(PaleOakLeaves {
                r#distance: 3,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 465 {
            return Some(PaleOakLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

