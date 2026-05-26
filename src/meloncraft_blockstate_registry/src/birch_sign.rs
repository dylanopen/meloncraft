use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchSign {
    pub rotation: i32,
    pub waterlogged: bool,
}

impl BlockState for BirchSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#rotation == 14 {
            return 5227;
        }
        if self.r#waterlogged == true && self.r#rotation == 1 {
            return 5200;
        }
        if self.r#rotation == 3 && self.r#waterlogged == true {
            return 5204;
        }
        if self.r#rotation == 7 && self.r#waterlogged == true {
            return 5212;
        }
        if self.r#waterlogged == true && self.r#rotation == 9 {
            return 5216;
        }
        if self.r#rotation == 12 && self.r#waterlogged == false {
            return 5223;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false {
            return 5209;
        }
        if self.r#waterlogged == true && self.r#rotation == 12 {
            return 5222;
        }
        if self.r#rotation == 8 && self.r#waterlogged == true {
            return 5214;
        }
        if self.r#waterlogged == false && self.r#rotation == 8 {
            return 5215;
        }
        if self.r#waterlogged == false && self.r#rotation == 3 {
            return 5205;
        }
        if self.r#waterlogged == false && self.r#rotation == 0 {
            return 5199;
        }
        if self.r#rotation == 4 && self.r#waterlogged == true {
            return 5206;
        }
        if self.r#rotation == 14 && self.r#waterlogged == true {
            return 5226;
        }
        if self.r#waterlogged == false && self.r#rotation == 6 {
            return 5211;
        }
        if self.r#waterlogged == false && self.r#rotation == 10 {
            return 5219;
        }
        if self.r#rotation == 7 && self.r#waterlogged == false {
            return 5213;
        }
        if self.r#waterlogged == false && self.r#rotation == 11 {
            return 5221;
        }
        if self.r#waterlogged == true && self.r#rotation == 15 {
            return 5228;
        }
        if self.r#waterlogged == false && self.r#rotation == 15 {
            return 5229;
        }
        if self.r#rotation == 4 && self.r#waterlogged == false {
            return 5207;
        }
        if self.r#waterlogged == true && self.r#rotation == 5 {
            return 5208;
        }
        if self.r#rotation == 6 && self.r#waterlogged == true {
            return 5210;
        }
        if self.r#rotation == 9 && self.r#waterlogged == false {
            return 5217;
        }
        if self.r#rotation == 13 && self.r#waterlogged == true {
            return 5224;
        }
        if self.r#rotation == 2 && self.r#waterlogged == false {
            return 5203;
        }
        if self.r#rotation == 11 && self.r#waterlogged == true {
            return 5220;
        }
        if self.r#waterlogged == false && self.r#rotation == 1 {
            return 5201;
        }
        if self.r#rotation == 13 && self.r#waterlogged == false {
            return 5225;
        }
        if self.r#rotation == 0 && self.r#waterlogged == true {
            return 5198;
        }
        if self.r#waterlogged == true && self.r#rotation == 2 {
            return 5202;
        }
        if self.r#rotation == 10 && self.r#waterlogged == true {
            return 5218;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5227 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5200 {
            return Some(BirchSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 5204 {
            return Some(BirchSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5212 {
            return Some(BirchSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5216 {
            return Some(BirchSign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 5223 {
            return Some(BirchSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5209 {
            return Some(BirchSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5222 {
            return Some(BirchSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5214 {
            return Some(BirchSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5215 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5205 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5199 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5206 {
            return Some(BirchSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5226 {
            return Some(BirchSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5211 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5219 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 5213 {
            return Some(BirchSign {
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5221 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5228 {
            return Some(BirchSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5229 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5207 {
            return Some(BirchSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5208 {
            return Some(BirchSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5210 {
            return Some(BirchSign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 5217 {
            return Some(BirchSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5224 {
            return Some(BirchSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5203 {
            return Some(BirchSign {
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5220 {
            return Some(BirchSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5201 {
            return Some(BirchSign {
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 5225 {
            return Some(BirchSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5198 {
            return Some(BirchSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5202 {
            return Some(BirchSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5218 {
            return Some(BirchSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
