use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooSign {
    pub waterlogged: bool,
    pub rotation: i32,
}

impl BlockState for BambooSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#rotation == 8 {
            return 5439;
        }
        if self.r#rotation == 9 && self.r#waterlogged == true {
            return 5440;
        }
        if self.r#rotation == 8 && self.r#waterlogged == true {
            return 5438;
        }
        if self.r#waterlogged == true && self.r#rotation == 13 {
            return 5448;
        }
        if self.r#rotation == 14 && self.r#waterlogged == true {
            return 5450;
        }
        if self.r#rotation == 3 && self.r#waterlogged == false {
            return 5429;
        }
        if self.r#waterlogged == true && self.r#rotation == 2 {
            return 5426;
        }
        if self.r#rotation == 10 && self.r#waterlogged == false {
            return 5443;
        }
        if self.r#rotation == 4 && self.r#waterlogged == false {
            return 5431;
        }
        if self.r#waterlogged == true && self.r#rotation == 15 {
            return 5452;
        }
        if self.r#rotation == 5 && self.r#waterlogged == true {
            return 5432;
        }
        if self.r#waterlogged == false && self.r#rotation == 15 {
            return 5453;
        }
        if self.r#waterlogged == true && self.r#rotation == 0 {
            return 5422;
        }
        if self.r#rotation == 3 && self.r#waterlogged == true {
            return 5428;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false {
            return 5433;
        }
        if self.r#waterlogged == false && self.r#rotation == 7 {
            return 5437;
        }
        if self.r#rotation == 1 && self.r#waterlogged == true {
            return 5424;
        }
        if self.r#waterlogged == false && self.r#rotation == 1 {
            return 5425;
        }
        if self.r#waterlogged == false && self.r#rotation == 2 {
            return 5427;
        }
        if self.r#waterlogged == true && self.r#rotation == 6 {
            return 5434;
        }
        if self.r#rotation == 10 && self.r#waterlogged == true {
            return 5442;
        }
        if self.r#rotation == 11 && self.r#waterlogged == false {
            return 5445;
        }
        if self.r#rotation == 12 && self.r#waterlogged == true {
            return 5446;
        }
        if self.r#waterlogged == true && self.r#rotation == 4 {
            return 5430;
        }
        if self.r#rotation == 11 && self.r#waterlogged == true {
            return 5444;
        }
        if self.r#rotation == 7 && self.r#waterlogged == true {
            return 5436;
        }
        if self.r#rotation == 12 && self.r#waterlogged == false {
            return 5447;
        }
        if self.r#waterlogged == false && self.r#rotation == 0 {
            return 5423;
        }
        if self.r#waterlogged == false && self.r#rotation == 13 {
            return 5449;
        }
        if self.r#rotation == 14 && self.r#waterlogged == false {
            return 5451;
        }
        if self.r#rotation == 9 && self.r#waterlogged == false {
            return 5441;
        }
        if self.r#waterlogged == false && self.r#rotation == 6 {
            return 5435;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5439 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5440 {
            return Some(BambooSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5438 {
            return Some(BambooSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5448 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5450 {
            return Some(BambooSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5429 {
            return Some(BambooSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5426 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5443 {
            return Some(BambooSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5431 {
            return Some(BambooSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5452 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5432 {
            return Some(BambooSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5453 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5422 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 5428 {
            return Some(BambooSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5433 {
            return Some(BambooSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5437 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5424 {
            return Some(BambooSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5425 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 5427 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5434 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5442 {
            return Some(BambooSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5445 {
            return Some(BambooSign {
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5446 {
            return Some(BambooSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5430 {
            return Some(BambooSign {
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 5444 {
            return Some(BambooSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5436 {
            return Some(BambooSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5447 {
            return Some(BambooSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5423 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5449 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5451 {
            return Some(BambooSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5441 {
            return Some(BambooSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5435 {
            return Some(BambooSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        return None;
    }
}
