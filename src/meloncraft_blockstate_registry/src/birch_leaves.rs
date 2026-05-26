use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchLeaves {
    pub distance: i32,
    pub persistent: bool,
    pub waterlogged: bool,
}

impl BlockState for BirchLeaves {
    fn to_id(&self) -> i32 {
        if self.r#distance == 2 && self.r#waterlogged == true && self.r#persistent == true {
            return 312;
        }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 2 {
            return 315;
        }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 1 {
            return 311;
        }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 2 {
            return 313;
        }
        if self.r#waterlogged == true && self.r#distance == 5 && self.r#persistent == true {
            return 324;
        }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 1 {
            return 309;
        }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 5 {
            return 326;
        }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 4 {
            return 323;
        }
        if self.r#waterlogged == false && self.r#distance == 5 && self.r#persistent == false {
            return 327;
        }
        if self.r#distance == 6 && self.r#persistent == true && self.r#waterlogged == false {
            return 329;
        }
        if self.r#waterlogged == false && self.r#distance == 6 && self.r#persistent == false {
            return 331;
        }
        if self.r#waterlogged == true && self.r#distance == 1 && self.r#persistent == false {
            return 310;
        }
        if self.r#persistent == true && self.r#distance == 7 && self.r#waterlogged == false {
            return 333;
        }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == true {
            return 334;
        }
        if self.r#distance == 7 && self.r#waterlogged == false && self.r#persistent == false {
            return 335;
        }
        if self.r#distance == 6 && self.r#waterlogged == true && self.r#persistent == false {
            return 330;
        }
        if self.r#waterlogged == false && self.r#persistent == true && self.r#distance == 3 {
            return 317;
        }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 2 {
            return 314;
        }
        if self.r#waterlogged == true && self.r#persistent == true && self.r#distance == 3 {
            return 316;
        }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 4 {
            return 322;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 4 {
            return 320;
        }
        if self.r#distance == 4 && self.r#persistent == true && self.r#waterlogged == false {
            return 321;
        }
        if self.r#distance == 3 && self.r#waterlogged == true && self.r#persistent == false {
            return 318;
        }
        if self.r#distance == 6 && self.r#persistent == true && self.r#waterlogged == true {
            return 328;
        }
        if self.r#distance == 5 && self.r#persistent == true && self.r#waterlogged == false {
            return 325;
        }
        if self.r#persistent == false && self.r#distance == 3 && self.r#waterlogged == false {
            return 319;
        }
        if self.r#waterlogged == true && self.r#distance == 1 && self.r#persistent == true {
            return 308;
        }
        if self.r#persistent == true && self.r#distance == 7 && self.r#waterlogged == true {
            return 332;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 312 {
            return Some(BirchLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 315 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 2,
            });
        }
        if state_id == 311 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 313 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 2,
            });
        }
        if state_id == 324 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 309 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 326 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 5,
            });
        }
        if state_id == 323 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 4,
            });
        }
        if state_id == 327 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 329 {
            return Some(BirchLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 331 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#distance: 6,
                r#persistent: false,
            });
        }
        if state_id == 310 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: false,
            });
        }
        if state_id == 333 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#distance: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 334 {
            return Some(BirchLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 335 {
            return Some(BirchLeaves {
                r#distance: 7,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 330 {
            return Some(BirchLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 317 {
            return Some(BirchLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 3,
            });
        }
        if state_id == 314 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 2,
            });
        }
        if state_id == 316 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 3,
            });
        }
        if state_id == 322 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 320 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 4,
            });
        }
        if state_id == 321 {
            return Some(BirchLeaves {
                r#distance: 4,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 318 {
            return Some(BirchLeaves {
                r#distance: 3,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 328 {
            return Some(BirchLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 325 {
            return Some(BirchLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 319 {
            return Some(BirchLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 308 {
            return Some(BirchLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: true,
            });
        }
        if state_id == 332 {
            return Some(BirchLeaves {
                r#persistent: true,
                r#distance: 7,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
