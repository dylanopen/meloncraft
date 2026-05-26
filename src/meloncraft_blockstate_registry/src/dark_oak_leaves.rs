use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakLeaves {
    pub persistent: bool,
    pub waterlogged: bool,
    pub distance: i32,
}

impl BlockState for DarkOakLeaves {
    fn to_id(&self) -> i32 {
        if self.r#persistent == true && self.r#distance == 7 && self.r#waterlogged == false {
            return 445;
        }
        if self.r#waterlogged == false && self.r#distance == 5 && self.r#persistent == true {
            return 437;
        }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == true {
            return 446;
        }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 5 {
            return 439;
        }
        if self.r#waterlogged == true && self.r#distance == 6 && self.r#persistent == true {
            return 440;
        }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 6 {
            return 443;
        }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 3 {
            return 430;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 1 {
            return 420;
        }
        if self.r#persistent == true && self.r#distance == 4 && self.r#waterlogged == true {
            return 432;
        }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 1 {
            return 421;
        }
        if self.r#waterlogged == false && self.r#distance == 3 && self.r#persistent == true {
            return 429;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 5 {
            return 436;
        }
        if self.r#waterlogged == true && self.r#distance == 2 && self.r#persistent == false {
            return 426;
        }
        if self.r#distance == 4 && self.r#persistent == true && self.r#waterlogged == false {
            return 433;
        }
        if self.r#distance == 2 && self.r#waterlogged == true && self.r#persistent == true {
            return 424;
        }
        if self.r#persistent == false && self.r#distance == 1 && self.r#waterlogged == false {
            return 423;
        }
        if self.r#waterlogged == true && self.r#distance == 5 && self.r#persistent == false {
            return 438;
        }
        if self.r#waterlogged == false && self.r#distance == 6 && self.r#persistent == true {
            return 441;
        }
        if self.r#distance == 4 && self.r#waterlogged == true && self.r#persistent == false {
            return 434;
        }
        if self.r#distance == 7 && self.r#waterlogged == false && self.r#persistent == false {
            return 447;
        }
        if self.r#waterlogged == false && self.r#distance == 2 && self.r#persistent == true {
            return 425;
        }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 1 {
            return 422;
        }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 2 {
            return 427;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 3 {
            return 428;
        }
        if self.r#distance == 4 && self.r#persistent == false && self.r#waterlogged == false {
            return 435;
        }
        if self.r#persistent == false && self.r#distance == 6 && self.r#waterlogged == true {
            return 442;
        }
        if self.r#distance == 3 && self.r#waterlogged == false && self.r#persistent == false {
            return 431;
        }
        if self.r#distance == 7 && self.r#waterlogged == true && self.r#persistent == true {
            return 444;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 445 {
            return Some(DarkOakLeaves {
                r#persistent: true,
                r#distance: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 437 {
            return Some(DarkOakLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 446 {
            return Some(DarkOakLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 439 {
            return Some(DarkOakLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 5,
            });
        }
        if state_id == 440 {
            return Some(DarkOakLeaves {
                r#waterlogged: true,
                r#distance: 6,
                r#persistent: true,
            });
        }
        if state_id == 443 {
            return Some(DarkOakLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 430 {
            return Some(DarkOakLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 3,
            });
        }
        if state_id == 420 {
            return Some(DarkOakLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 1,
            });
        }
        if state_id == 432 {
            return Some(DarkOakLeaves {
                r#persistent: true,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 421 {
            return Some(DarkOakLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 429 {
            return Some(DarkOakLeaves {
                r#waterlogged: false,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 436 {
            return Some(DarkOakLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 426 {
            return Some(DarkOakLeaves {
                r#waterlogged: true,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 433 {
            return Some(DarkOakLeaves {
                r#distance: 4,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 424 {
            return Some(DarkOakLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 423 {
            return Some(DarkOakLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 438 {
            return Some(DarkOakLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 441 {
            return Some(DarkOakLeaves {
                r#waterlogged: false,
                r#distance: 6,
                r#persistent: true,
            });
        }
        if state_id == 434 {
            return Some(DarkOakLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 447 {
            return Some(DarkOakLeaves {
                r#distance: 7,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 425 {
            return Some(DarkOakLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: true,
            });
        }
        if state_id == 422 {
            return Some(DarkOakLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 1,
            });
        }
        if state_id == 427 {
            return Some(DarkOakLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 2,
            });
        }
        if state_id == 428 {
            return Some(DarkOakLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 3,
            });
        }
        if state_id == 435 {
            return Some(DarkOakLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 442 {
            return Some(DarkOakLeaves {
                r#persistent: false,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 431 {
            return Some(DarkOakLeaves {
                r#distance: 3,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 444 {
            return Some(DarkOakLeaves {
                r#distance: 7,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        return None;
    }
}
