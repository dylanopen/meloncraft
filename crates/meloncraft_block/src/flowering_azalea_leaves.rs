use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloweringAzaleaLeaves {
    pub waterlogged: bool,
    pub distance: i32,
    pub persistent: bool,
}


impl BlockState for FloweringAzaleaLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#distance == 5 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 549; }
        if block_state.r#waterlogged == false && block_state.r#distance == 5 && block_state.r#persistent == false { return 551; }
        if block_state.r#waterlogged == true && block_state.r#distance == 6 && block_state.r#persistent == true { return 552; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 4 { return 546; }
        if block_state.r#persistent == false && block_state.r#distance == 4 && block_state.r#waterlogged == false { return 547; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 555; }
        if block_state.r#distance == 3 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 542; }
        if block_state.r#distance == 4 && block_state.r#waterlogged == false && block_state.r#persistent == true { return 545; }
        if block_state.r#distance == 2 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 536; }
        if block_state.r#distance == 1 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 535; }
        if block_state.r#distance == 5 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 548; }
        if block_state.r#distance == 2 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 538; }
        if block_state.r#waterlogged == true && block_state.r#persistent == false && block_state.r#distance == 7 { return 558; }
        if block_state.r#distance == 7 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 559; }
        if block_state.r#persistent == false && block_state.r#distance == 1 && block_state.r#waterlogged == true { return 534; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 7 { return 557; }
        if block_state.r#waterlogged == false && block_state.r#distance == 6 && block_state.r#persistent == true { return 553; }
        if block_state.r#persistent == true && block_state.r#distance == 7 && block_state.r#waterlogged == true { return 556; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 4 { return 544; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 554; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == false { return 543; }
        if block_state.r#waterlogged == true && block_state.r#distance == 1 && block_state.r#persistent == true { return 532; }
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 1 { return 533; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 3 { return 540; }
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 2 { return 537; }
        if block_state.r#distance == 5 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 550; }
        if block_state.r#waterlogged == false && block_state.r#distance == 2 && block_state.r#persistent == false { return 539; }
        if block_state.r#persistent == true && block_state.r#distance == 3 && block_state.r#waterlogged == false { return 541; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 549 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 551 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 552 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: true,
                r#distance: 6,
                r#persistent: true,
            });
        }
        if state_id == 546 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 4,
            });
        }
        if state_id == 547 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: false,
                r#distance: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 555 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 6,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 542 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 3,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 545 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 4,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 536 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 2,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 535 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 1,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 548 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 538 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 558 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 7,
            });
        }
        if state_id == 559 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 534 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 557 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 553 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: false,
                r#distance: 6,
                r#persistent: true,
            });
        }
        if state_id == 556 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#distance: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 544 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 554 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 543 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 532 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: true,
            });
        }
        if state_id == 533 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 540 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 3,
            });
        }
        if state_id == 537 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 2,
            });
        }
        if state_id == 550 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 5,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 539 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 541 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

