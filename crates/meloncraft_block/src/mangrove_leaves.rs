use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveLeaves {
    pub waterlogged: bool,
    pub distance: i32,
    pub persistent: bool,
}


impl BlockState for MangroveLeaves {
    fn to_id(&self) -> i32 {
        if self.r#distance == 6 && self.r#persistent == true && self.r#waterlogged == true { return 496; }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 7 { return 500; }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 7 { return 503; }
        if self.r#distance == 1 && self.r#waterlogged == false && self.r#persistent == false { return 479; }
        if self.r#waterlogged == true && self.r#persistent == true && self.r#distance == 3 { return 484; }
        if self.r#distance == 1 && self.r#waterlogged == true && self.r#persistent == false { return 478; }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 3 { return 487; }
        if self.r#distance == 4 && self.r#persistent == true && self.r#waterlogged == false { return 489; }
        if self.r#distance == 1 && self.r#persistent == true && self.r#waterlogged == false { return 477; }
        if self.r#persistent == false && self.r#distance == 6 && self.r#waterlogged == true { return 498; }
        if self.r#persistent == true && self.r#distance == 2 && self.r#waterlogged == true { return 480; }
        if self.r#persistent == true && self.r#distance == 4 && self.r#waterlogged == true { return 488; }
        if self.r#distance == 4 && self.r#persistent == false && self.r#waterlogged == false { return 491; }
        if self.r#persistent == false && self.r#distance == 5 && self.r#waterlogged == true { return 494; }
        if self.r#distance == 1 && self.r#persistent == true && self.r#waterlogged == true { return 476; }
        if self.r#distance == 4 && self.r#waterlogged == true && self.r#persistent == false { return 490; }
        if self.r#waterlogged == false && self.r#persistent == true && self.r#distance == 6 { return 497; }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 2 { return 483; }
        if self.r#distance == 2 && self.r#waterlogged == true && self.r#persistent == false { return 482; }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == true { return 502; }
        if self.r#distance == 5 && self.r#waterlogged == true && self.r#persistent == true { return 492; }
        if self.r#waterlogged == false && self.r#distance == 2 && self.r#persistent == true { return 481; }
        if self.r#distance == 5 && self.r#persistent == false && self.r#waterlogged == false { return 495; }
        if self.r#waterlogged == false && self.r#distance == 7 && self.r#persistent == true { return 501; }
        if self.r#waterlogged == false && self.r#distance == 3 && self.r#persistent == true { return 485; }
        if self.r#distance == 3 && self.r#persistent == false && self.r#waterlogged == true { return 486; }
        if self.r#distance == 6 && self.r#persistent == false && self.r#waterlogged == false { return 499; }
        if self.r#distance == 5 && self.r#waterlogged == false && self.r#persistent == true { return 493; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 496 {
            return Some(MangroveLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 500 {
            return Some(MangroveLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 7,
            });
        }
        if state_id == 503 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 7,
            });
        }
        if state_id == 479 {
            return Some(MangroveLeaves {
                r#distance: 1,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 484 {
            return Some(MangroveLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 3,
            });
        }
        if state_id == 478 {
            return Some(MangroveLeaves {
                r#distance: 1,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 487 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 3,
            });
        }
        if state_id == 489 {
            return Some(MangroveLeaves {
                r#distance: 4,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 477 {
            return Some(MangroveLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 498 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 480 {
            return Some(MangroveLeaves {
                r#persistent: true,
                r#distance: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 488 {
            return Some(MangroveLeaves {
                r#persistent: true,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 491 {
            return Some(MangroveLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 494 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#distance: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 476 {
            return Some(MangroveLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 490 {
            return Some(MangroveLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 497 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 6,
            });
        }
        if state_id == 483 {
            return Some(MangroveLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 2,
            });
        }
        if state_id == 482 {
            return Some(MangroveLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 502 {
            return Some(MangroveLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 492 {
            return Some(MangroveLeaves {
                r#distance: 5,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 481 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: true,
            });
        }
        if state_id == 495 {
            return Some(MangroveLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 501 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 485 {
            return Some(MangroveLeaves {
                r#waterlogged: false,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 486 {
            return Some(MangroveLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 499 {
            return Some(MangroveLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 493 {
            return Some(MangroveLeaves {
                r#distance: 5,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        return None;
    }
}

