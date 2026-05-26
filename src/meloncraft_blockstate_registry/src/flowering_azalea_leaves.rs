use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloweringAzaleaLeaves {
    pub persistent: bool,
    pub waterlogged: bool,
    pub distance: i32,
}

impl BlockState for FloweringAzaleaLeaves {
    fn to_id(&self) -> i32 {
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == true {
            return 558;
        }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == false {
            return 559;
        }
        if self.r#distance == 2 && self.r#persistent == false && self.r#waterlogged == true {
            return 538;
        }
        if self.r#distance == 3 && self.r#persistent == false && self.r#waterlogged == false {
            return 543;
        }
        if self.r#distance == 4 && self.r#waterlogged == false && self.r#persistent == true {
            return 545;
        }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 6 {
            return 553;
        }
        if self.r#persistent == true && self.r#distance == 4 && self.r#waterlogged == true {
            return 544;
        }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 3 {
            return 541;
        }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 2 {
            return 539;
        }
        if self.r#distance == 1 && self.r#persistent == true && self.r#waterlogged == false {
            return 533;
        }
        if self.r#persistent == true && self.r#distance == 2 && self.r#waterlogged == true {
            return 536;
        }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 4 {
            return 547;
        }
        if self.r#distance == 4 && self.r#waterlogged == true && self.r#persistent == false {
            return 546;
        }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 2 {
            return 537;
        }
        if self.r#distance == 5 && self.r#persistent == true && self.r#waterlogged == true {
            return 548;
        }
        if self.r#persistent == true && self.r#distance == 5 && self.r#waterlogged == false {
            return 549;
        }
        if self.r#distance == 1 && self.r#persistent == false && self.r#waterlogged == false {
            return 535;
        }
        if self.r#persistent == false && self.r#distance == 5 && self.r#waterlogged == true {
            return 550;
        }
        if self.r#distance == 5 && self.r#waterlogged == false && self.r#persistent == false {
            return 551;
        }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 6 {
            return 554;
        }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 6 {
            return 555;
        }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 1 {
            return 534;
        }
        if self.r#waterlogged == false && self.r#persistent == true && self.r#distance == 7 {
            return 557;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 7 {
            return 556;
        }
        if self.r#waterlogged == true && self.r#distance == 3 && self.r#persistent == true {
            return 540;
        }
        if self.r#distance == 6 && self.r#persistent == true && self.r#waterlogged == true {
            return 552;
        }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 3 {
            return 542;
        }
        if self.r#persistent == true && self.r#distance == 1 && self.r#waterlogged == true {
            return 532;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 558 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 559 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 538 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 543 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 545 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 4,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 553 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 544 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 541 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 539 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 2,
            });
        }
        if state_id == 533 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 536 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#distance: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 547 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 4,
            });
        }
        if state_id == 546 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 537 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 2,
            });
        }
        if state_id == 548 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 549 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#distance: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 535 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 550 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: false,
                r#distance: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 551 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 5,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 554 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 555 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 534 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 1,
            });
        }
        if state_id == 557 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 556 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 7,
            });
        }
        if state_id == 540 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: true,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 552 {
            return Some(FloweringAzaleaLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 542 {
            return Some(FloweringAzaleaLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 3,
            });
        }
        if state_id == 532 {
            return Some(FloweringAzaleaLeaves {
                r#persistent: true,
                r#distance: 1,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
