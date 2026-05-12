use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaSign {
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for AcaciaSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#rotation == 13 { return 5256; }
        if self.r#waterlogged == true && self.r#rotation == 2 { return 5234; }
        if self.r#waterlogged == false && self.r#rotation == 2 { return 5235; }
        if self.r#waterlogged == false && self.r#rotation == 11 { return 5253; }
        if self.r#waterlogged == true && self.r#rotation == 12 { return 5254; }
        if self.r#rotation == 4 && self.r#waterlogged == true { return 5238; }
        if self.r#rotation == 0 && self.r#waterlogged == false { return 5231; }
        if self.r#waterlogged == true && self.r#rotation == 1 { return 5232; }
        if self.r#rotation == 3 && self.r#waterlogged == false { return 5237; }
        if self.r#rotation == 12 && self.r#waterlogged == false { return 5255; }
        if self.r#waterlogged == true && self.r#rotation == 9 { return 5248; }
        if self.r#waterlogged == true && self.r#rotation == 15 { return 5260; }
        if self.r#rotation == 10 && self.r#waterlogged == true { return 5250; }
        if self.r#rotation == 8 && self.r#waterlogged == true { return 5246; }
        if self.r#rotation == 11 && self.r#waterlogged == true { return 5252; }
        if self.r#waterlogged == false && self.r#rotation == 5 { return 5241; }
        if self.r#waterlogged == false && self.r#rotation == 6 { return 5243; }
        if self.r#waterlogged == false && self.r#rotation == 4 { return 5239; }
        if self.r#waterlogged == false && self.r#rotation == 15 { return 5261; }
        if self.r#rotation == 0 && self.r#waterlogged == true { return 5230; }
        if self.r#rotation == 9 && self.r#waterlogged == false { return 5249; }
        if self.r#waterlogged == false && self.r#rotation == 14 { return 5259; }
        if self.r#waterlogged == false && self.r#rotation == 8 { return 5247; }
        if self.r#rotation == 5 && self.r#waterlogged == true { return 5240; }
        if self.r#waterlogged == false && self.r#rotation == 7 { return 5245; }
        if self.r#waterlogged == false && self.r#rotation == 10 { return 5251; }
        if self.r#waterlogged == false && self.r#rotation == 1 { return 5233; }
        if self.r#rotation == 3 && self.r#waterlogged == true { return 5236; }
        if self.r#rotation == 7 && self.r#waterlogged == true { return 5244; }
        if self.r#waterlogged == true && self.r#rotation == 14 { return 5258; }
        if self.r#waterlogged == false && self.r#rotation == 13 { return 5257; }
        if self.r#rotation == 6 && self.r#waterlogged == true { return 5242; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5256 {
            return Some(AcaciaSign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5234 {
            return Some(AcaciaSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5235 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5253 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5254 {
            return Some(AcaciaSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5238 {
            return Some(AcaciaSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5231 {
            return Some(AcaciaSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5232 {
            return Some(AcaciaSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 5237 {
            return Some(AcaciaSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5255 {
            return Some(AcaciaSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5248 {
            return Some(AcaciaSign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 5260 {
            return Some(AcaciaSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5250 {
            return Some(AcaciaSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5246 {
            return Some(AcaciaSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5252 {
            return Some(AcaciaSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5241 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5243 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5239 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5261 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5230 {
            return Some(AcaciaSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5249 {
            return Some(AcaciaSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5259 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5247 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5240 {
            return Some(AcaciaSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5245 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5251 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 5233 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 5236 {
            return Some(AcaciaSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5244 {
            return Some(AcaciaSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5258 {
            return Some(AcaciaSign {
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 5257 {
            return Some(AcaciaSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5242 {
            return Some(AcaciaSign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

