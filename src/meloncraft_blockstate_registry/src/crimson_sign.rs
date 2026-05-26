use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonSign {
    pub rotation: i32,
    pub waterlogged: bool,
}

impl BlockState for CrimsonSign {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 14 && self.r#waterlogged == false {
            return 21469;
        }
        if self.r#waterlogged == false && self.r#rotation == 1 {
            return 21443;
        }
        if self.r#waterlogged == true && self.r#rotation == 1 {
            return 21442;
        }
        if self.r#rotation == 8 && self.r#waterlogged == false {
            return 21457;
        }
        if self.r#rotation == 3 && self.r#waterlogged == false {
            return 21447;
        }
        if self.r#waterlogged == true && self.r#rotation == 14 {
            return 21468;
        }
        if self.r#rotation == 13 && self.r#waterlogged == false {
            return 21467;
        }
        if self.r#rotation == 13 && self.r#waterlogged == true {
            return 21466;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false {
            return 21451;
        }
        if self.r#rotation == 6 && self.r#waterlogged == true {
            return 21452;
        }
        if self.r#waterlogged == true && self.r#rotation == 10 {
            return 21460;
        }
        if self.r#waterlogged == false && self.r#rotation == 9 {
            return 21459;
        }
        if self.r#waterlogged == true && self.r#rotation == 2 {
            return 21444;
        }
        if self.r#rotation == 15 && self.r#waterlogged == true {
            return 21470;
        }
        if self.r#rotation == 0 && self.r#waterlogged == true {
            return 21440;
        }
        if self.r#rotation == 0 && self.r#waterlogged == false {
            return 21441;
        }
        if self.r#waterlogged == false && self.r#rotation == 11 {
            return 21463;
        }
        if self.r#rotation == 8 && self.r#waterlogged == true {
            return 21456;
        }
        if self.r#waterlogged == false && self.r#rotation == 12 {
            return 21465;
        }
        if self.r#rotation == 15 && self.r#waterlogged == false {
            return 21471;
        }
        if self.r#rotation == 9 && self.r#waterlogged == true {
            return 21458;
        }
        if self.r#waterlogged == false && self.r#rotation == 2 {
            return 21445;
        }
        if self.r#rotation == 4 && self.r#waterlogged == false {
            return 21449;
        }
        if self.r#rotation == 3 && self.r#waterlogged == true {
            return 21446;
        }
        if self.r#rotation == 7 && self.r#waterlogged == false {
            return 21455;
        }
        if self.r#rotation == 6 && self.r#waterlogged == false {
            return 21453;
        }
        if self.r#waterlogged == true && self.r#rotation == 12 {
            return 21464;
        }
        if self.r#rotation == 10 && self.r#waterlogged == false {
            return 21461;
        }
        if self.r#waterlogged == true && self.r#rotation == 5 {
            return 21450;
        }
        if self.r#rotation == 4 && self.r#waterlogged == true {
            return 21448;
        }
        if self.r#rotation == 7 && self.r#waterlogged == true {
            return 21454;
        }
        if self.r#rotation == 11 && self.r#waterlogged == true {
            return 21462;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21469 {
            return Some(CrimsonSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 21443 {
            return Some(CrimsonSign {
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 21442 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 21457 {
            return Some(CrimsonSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 21447 {
            return Some(CrimsonSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 21468 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 21467 {
            return Some(CrimsonSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 21466 {
            return Some(CrimsonSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 21451 {
            return Some(CrimsonSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 21452 {
            return Some(CrimsonSign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 21460 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 21459 {
            return Some(CrimsonSign {
                r#waterlogged: false,
                r#rotation: 9,
            });
        }
        if state_id == 21444 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 21470 {
            return Some(CrimsonSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 21440 {
            return Some(CrimsonSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 21441 {
            return Some(CrimsonSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 21463 {
            return Some(CrimsonSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 21456 {
            return Some(CrimsonSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 21465 {
            return Some(CrimsonSign {
                r#waterlogged: false,
                r#rotation: 12,
            });
        }
        if state_id == 21471 {
            return Some(CrimsonSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 21458 {
            return Some(CrimsonSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 21445 {
            return Some(CrimsonSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 21449 {
            return Some(CrimsonSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 21446 {
            return Some(CrimsonSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 21455 {
            return Some(CrimsonSign {
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 21453 {
            return Some(CrimsonSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 21464 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 21461 {
            return Some(CrimsonSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 21450 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 21448 {
            return Some(CrimsonSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 21454 {
            return Some(CrimsonSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 21462 {
            return Some(CrimsonSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
