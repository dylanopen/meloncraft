use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedSign {
    pub rotation: i32,
    pub waterlogged: bool,
}

impl BlockState for WarpedSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#rotation == 6 {
            return 21484;
        }
        if self.r#rotation == 9 && self.r#waterlogged == false {
            return 21491;
        }
        if self.r#waterlogged == true && self.r#rotation == 15 {
            return 21502;
        }
        if self.r#rotation == 0 && self.r#waterlogged == false {
            return 21473;
        }
        if self.r#waterlogged == true && self.r#rotation == 9 {
            return 21490;
        }
        if self.r#waterlogged == false && self.r#rotation == 6 {
            return 21485;
        }
        if self.r#rotation == 3 && self.r#waterlogged == false {
            return 21479;
        }
        if self.r#rotation == 7 && self.r#waterlogged == false {
            return 21487;
        }
        if self.r#waterlogged == true && self.r#rotation == 8 {
            return 21488;
        }
        if self.r#rotation == 10 && self.r#waterlogged == true {
            return 21492;
        }
        if self.r#waterlogged == false && self.r#rotation == 14 {
            return 21501;
        }
        if self.r#waterlogged == false && self.r#rotation == 10 {
            return 21493;
        }
        if self.r#waterlogged == false && self.r#rotation == 2 {
            return 21477;
        }
        if self.r#rotation == 1 && self.r#waterlogged == true {
            return 21474;
        }
        if self.r#rotation == 4 && self.r#waterlogged == true {
            return 21480;
        }
        if self.r#waterlogged == false && self.r#rotation == 11 {
            return 21495;
        }
        if self.r#rotation == 15 && self.r#waterlogged == false {
            return 21503;
        }
        if self.r#waterlogged == false && self.r#rotation == 4 {
            return 21481;
        }
        if self.r#rotation == 2 && self.r#waterlogged == true {
            return 21476;
        }
        if self.r#rotation == 1 && self.r#waterlogged == false {
            return 21475;
        }
        if self.r#waterlogged == false && self.r#rotation == 13 {
            return 21499;
        }
        if self.r#rotation == 12 && self.r#waterlogged == false {
            return 21497;
        }
        if self.r#rotation == 14 && self.r#waterlogged == true {
            return 21500;
        }
        if self.r#waterlogged == true && self.r#rotation == 0 {
            return 21472;
        }
        if self.r#waterlogged == true && self.r#rotation == 3 {
            return 21478;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false {
            return 21483;
        }
        if self.r#waterlogged == true && self.r#rotation == 5 {
            return 21482;
        }
        if self.r#waterlogged == true && self.r#rotation == 11 {
            return 21494;
        }
        if self.r#rotation == 7 && self.r#waterlogged == true {
            return 21486;
        }
        if self.r#rotation == 13 && self.r#waterlogged == true {
            return 21498;
        }
        if self.r#rotation == 12 && self.r#waterlogged == true {
            return 21496;
        }
        if self.r#rotation == 8 && self.r#waterlogged == false {
            return 21489;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21484 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 21491 {
            return Some(WarpedSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 21502 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 21473 {
            return Some(WarpedSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 21490 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 21485 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 21479 {
            return Some(WarpedSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 21487 {
            return Some(WarpedSign {
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 21488 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 21492 {
            return Some(WarpedSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 21501 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 21493 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 21477 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 21474 {
            return Some(WarpedSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 21480 {
            return Some(WarpedSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 21495 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 21503 {
            return Some(WarpedSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 21481 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 21476 {
            return Some(WarpedSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 21475 {
            return Some(WarpedSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 21499 {
            return Some(WarpedSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 21497 {
            return Some(WarpedSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 21500 {
            return Some(WarpedSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 21472 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 21478 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 21483 {
            return Some(WarpedSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 21482 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 21494 {
            return Some(WarpedSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 21486 {
            return Some(WarpedSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 21498 {
            return Some(WarpedSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 21496 {
            return Some(WarpedSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 21489 {
            return Some(WarpedSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
