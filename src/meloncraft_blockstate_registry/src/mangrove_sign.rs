use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveSign {
    pub rotation: i32,
    pub waterlogged: bool,
}

impl BlockState for MangroveSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#rotation == 1 {
            return 5393;
        }
        if self.r#waterlogged == true && self.r#rotation == 11 {
            return 5412;
        }
        if self.r#waterlogged == true && self.r#rotation == 5 {
            return 5400;
        }
        if self.r#waterlogged == false && self.r#rotation == 7 {
            return 5405;
        }
        if self.r#waterlogged == true && self.r#rotation == 14 {
            return 5418;
        }
        if self.r#rotation == 6 && self.r#waterlogged == false {
            return 5403;
        }
        if self.r#rotation == 9 && self.r#waterlogged == false {
            return 5409;
        }
        if self.r#rotation == 15 && self.r#waterlogged == true {
            return 5420;
        }
        if self.r#waterlogged == true && self.r#rotation == 2 {
            return 5394;
        }
        if self.r#rotation == 0 && self.r#waterlogged == true {
            return 5390;
        }
        if self.r#rotation == 13 && self.r#waterlogged == true {
            return 5416;
        }
        if self.r#waterlogged == true && self.r#rotation == 3 {
            return 5396;
        }
        if self.r#rotation == 13 && self.r#waterlogged == false {
            return 5417;
        }
        if self.r#waterlogged == false && self.r#rotation == 3 {
            return 5397;
        }
        if self.r#rotation == 15 && self.r#waterlogged == false {
            return 5421;
        }
        if self.r#waterlogged == true && self.r#rotation == 7 {
            return 5404;
        }
        if self.r#rotation == 8 && self.r#waterlogged == false {
            return 5407;
        }
        if self.r#rotation == 14 && self.r#waterlogged == false {
            return 5419;
        }
        if self.r#waterlogged == false && self.r#rotation == 0 {
            return 5391;
        }
        if self.r#rotation == 10 && self.r#waterlogged == false {
            return 5411;
        }
        if self.r#rotation == 9 && self.r#waterlogged == true {
            return 5408;
        }
        if self.r#rotation == 8 && self.r#waterlogged == true {
            return 5406;
        }
        if self.r#waterlogged == false && self.r#rotation == 11 {
            return 5413;
        }
        if self.r#rotation == 6 && self.r#waterlogged == true {
            return 5402;
        }
        if self.r#rotation == 12 && self.r#waterlogged == true {
            return 5414;
        }
        if self.r#waterlogged == false && self.r#rotation == 4 {
            return 5399;
        }
        if self.r#rotation == 12 && self.r#waterlogged == false {
            return 5415;
        }
        if self.r#rotation == 1 && self.r#waterlogged == true {
            return 5392;
        }
        if self.r#waterlogged == true && self.r#rotation == 4 {
            return 5398;
        }
        if self.r#rotation == 10 && self.r#waterlogged == true {
            return 5410;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false {
            return 5401;
        }
        if self.r#rotation == 2 && self.r#waterlogged == false {
            return 5395;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5393 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 5412 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5400 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5405 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5418 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 5403 {
            return Some(MangroveSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 5409 {
            return Some(MangroveSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5420 {
            return Some(MangroveSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5394 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5390 {
            return Some(MangroveSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5416 {
            return Some(MangroveSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5396 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5417 {
            return Some(MangroveSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5397 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5421 {
            return Some(MangroveSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5404 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 5407 {
            return Some(MangroveSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5419 {
            return Some(MangroveSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5391 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5411 {
            return Some(MangroveSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5408 {
            return Some(MangroveSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5406 {
            return Some(MangroveSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5413 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5402 {
            return Some(MangroveSign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 5414 {
            return Some(MangroveSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5399 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5415 {
            return Some(MangroveSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5392 {
            return Some(MangroveSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5398 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 5410 {
            return Some(MangroveSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5401 {
            return Some(MangroveSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5395 {
            return Some(MangroveSign {
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
