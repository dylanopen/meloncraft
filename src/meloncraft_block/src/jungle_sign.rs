use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleSign {
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for JungleSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#rotation == 7 { return 5309; }
        if self.r#waterlogged == true && self.r#rotation == 6 { return 5306; }
        if self.r#rotation == 12 && self.r#waterlogged == false { return 5319; }
        if self.r#waterlogged == true && self.r#rotation == 12 { return 5318; }
        if self.r#waterlogged == false && self.r#rotation == 13 { return 5321; }
        if self.r#rotation == 10 && self.r#waterlogged == false { return 5315; }
        if self.r#rotation == 4 && self.r#waterlogged == true { return 5302; }
        if self.r#waterlogged == true && self.r#rotation == 5 { return 5304; }
        if self.r#rotation == 6 && self.r#waterlogged == false { return 5307; }
        if self.r#waterlogged == false && self.r#rotation == 14 { return 5323; }
        if self.r#waterlogged == true && self.r#rotation == 11 { return 5316; }
        if self.r#rotation == 14 && self.r#waterlogged == true { return 5322; }
        if self.r#waterlogged == false && self.r#rotation == 15 { return 5325; }
        if self.r#rotation == 2 && self.r#waterlogged == false { return 5299; }
        if self.r#waterlogged == false && self.r#rotation == 0 { return 5295; }
        if self.r#rotation == 3 && self.r#waterlogged == false { return 5301; }
        if self.r#rotation == 1 && self.r#waterlogged == true { return 5296; }
        if self.r#waterlogged == true && self.r#rotation == 3 { return 5300; }
        if self.r#waterlogged == true && self.r#rotation == 2 { return 5298; }
        if self.r#rotation == 7 && self.r#waterlogged == true { return 5308; }
        if self.r#rotation == 9 && self.r#waterlogged == true { return 5312; }
        if self.r#waterlogged == false && self.r#rotation == 5 { return 5305; }
        if self.r#rotation == 1 && self.r#waterlogged == false { return 5297; }
        if self.r#waterlogged == false && self.r#rotation == 11 { return 5317; }
        if self.r#rotation == 13 && self.r#waterlogged == true { return 5320; }
        if self.r#rotation == 9 && self.r#waterlogged == false { return 5313; }
        if self.r#waterlogged == false && self.r#rotation == 4 { return 5303; }
        if self.r#rotation == 15 && self.r#waterlogged == true { return 5324; }
        if self.r#waterlogged == true && self.r#rotation == 10 { return 5314; }
        if self.r#rotation == 8 && self.r#waterlogged == true { return 5310; }
        if self.r#rotation == 8 && self.r#waterlogged == false { return 5311; }
        if self.r#rotation == 0 && self.r#waterlogged == true { return 5294; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5309 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5306 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5319 {
            return Some(JungleSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5318 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5321 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5315 {
            return Some(JungleSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5302 {
            return Some(JungleSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5304 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5307 {
            return Some(JungleSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 5323 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5316 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5322 {
            return Some(JungleSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5325 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5299 {
            return Some(JungleSign {
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5295 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5301 {
            return Some(JungleSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5296 {
            return Some(JungleSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5300 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5298 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5308 {
            return Some(JungleSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5312 {
            return Some(JungleSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5305 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5297 {
            return Some(JungleSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5317 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5320 {
            return Some(JungleSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5313 {
            return Some(JungleSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5303 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5324 {
            return Some(JungleSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5314 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5310 {
            return Some(JungleSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5311 {
            return Some(JungleSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5294 {
            return Some(JungleSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

