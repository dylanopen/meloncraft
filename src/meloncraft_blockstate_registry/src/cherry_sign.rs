use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherrySign {
    pub rotation: i32,
    pub waterlogged: bool,
}

impl BlockState for CherrySign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#rotation == 12 {
            return 5287;
        }
        if self.r#rotation == 10 && self.r#waterlogged == false {
            return 5283;
        }
        if self.r#rotation == 14 && self.r#waterlogged == true {
            return 5290;
        }
        if self.r#rotation == 3 && self.r#waterlogged == false {
            return 5269;
        }
        if self.r#rotation == 11 && self.r#waterlogged == true {
            return 5284;
        }
        if self.r#rotation == 1 && self.r#waterlogged == false {
            return 5265;
        }
        if self.r#rotation == 4 && self.r#waterlogged == true {
            return 5270;
        }
        if self.r#rotation == 8 && self.r#waterlogged == false {
            return 5279;
        }
        if self.r#waterlogged == true && self.r#rotation == 9 {
            return 5280;
        }
        if self.r#rotation == 0 && self.r#waterlogged == true {
            return 5262;
        }
        if self.r#waterlogged == true && self.r#rotation == 10 {
            return 5282;
        }
        if self.r#waterlogged == false && self.r#rotation == 13 {
            return 5289;
        }
        if self.r#waterlogged == true && self.r#rotation == 15 {
            return 5292;
        }
        if self.r#waterlogged == false && self.r#rotation == 0 {
            return 5263;
        }
        if self.r#waterlogged == true && self.r#rotation == 2 {
            return 5266;
        }
        if self.r#rotation == 6 && self.r#waterlogged == true {
            return 5274;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false {
            return 5273;
        }
        if self.r#rotation == 12 && self.r#waterlogged == true {
            return 5286;
        }
        if self.r#waterlogged == false && self.r#rotation == 15 {
            return 5293;
        }
        if self.r#waterlogged == false && self.r#rotation == 6 {
            return 5275;
        }
        if self.r#waterlogged == false && self.r#rotation == 2 {
            return 5267;
        }
        if self.r#rotation == 3 && self.r#waterlogged == true {
            return 5268;
        }
        if self.r#waterlogged == false && self.r#rotation == 7 {
            return 5277;
        }
        if self.r#waterlogged == true && self.r#rotation == 13 {
            return 5288;
        }
        if self.r#waterlogged == true && self.r#rotation == 8 {
            return 5278;
        }
        if self.r#waterlogged == false && self.r#rotation == 11 {
            return 5285;
        }
        if self.r#waterlogged == true && self.r#rotation == 7 {
            return 5276;
        }
        if self.r#waterlogged == true && self.r#rotation == 1 {
            return 5264;
        }
        if self.r#waterlogged == false && self.r#rotation == 4 {
            return 5271;
        }
        if self.r#rotation == 9 && self.r#waterlogged == false {
            return 5281;
        }
        if self.r#waterlogged == true && self.r#rotation == 5 {
            return 5272;
        }
        if self.r#waterlogged == false && self.r#rotation == 14 {
            return 5291;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5287 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 12,
            });
        }
        if state_id == 5283 {
            return Some(CherrySign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5290 {
            return Some(CherrySign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5269 {
            return Some(CherrySign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5284 {
            return Some(CherrySign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5265 {
            return Some(CherrySign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5270 {
            return Some(CherrySign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5279 {
            return Some(CherrySign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5280 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 5262 {
            return Some(CherrySign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5282 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5289 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5292 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5263 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5266 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5274 {
            return Some(CherrySign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 5273 {
            return Some(CherrySign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5286 {
            return Some(CherrySign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5293 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5275 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5267 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5268 {
            return Some(CherrySign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5277 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5288 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5278 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 5285 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5276 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 5264 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 5271 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5281 {
            return Some(CherrySign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5272 {
            return Some(CherrySign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5291 {
            return Some(CherrySign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        return None;
    }
}
