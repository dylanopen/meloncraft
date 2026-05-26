use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaLeaves {
    pub persistent: bool,
    pub waterlogged: bool,
    pub distance: i32,
}

impl BlockState for AcaciaLeaves {
    fn to_id(&self) -> i32 {
        if self.r#distance == 6 && self.r#persistent == true && self.r#waterlogged == true {
            return 384;
        }
        if self.r#waterlogged == true && self.r#distance == 5 && self.r#persistent == true {
            return 380;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 2 {
            return 368;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 1 {
            return 364;
        }
        if self.r#waterlogged == true && self.r#distance == 3 && self.r#persistent == true {
            return 372;
        }
        if self.r#distance == 3 && self.r#waterlogged == true && self.r#persistent == false {
            return 374;
        }
        if self.r#waterlogged == false && self.r#persistent == true && self.r#distance == 1 {
            return 365;
        }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 6 {
            return 386;
        }
        if self.r#waterlogged == true && self.r#persistent == true && self.r#distance == 4 {
            return 376;
        }
        if self.r#distance == 7 && self.r#persistent == true && self.r#waterlogged == true {
            return 388;
        }
        if self.r#persistent == true && self.r#distance == 3 && self.r#waterlogged == false {
            return 373;
        }
        if self.r#waterlogged == false && self.r#distance == 7 && self.r#persistent == true {
            return 389;
        }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == true {
            return 390;
        }
        if self.r#waterlogged == false && self.r#distance == 7 && self.r#persistent == false {
            return 391;
        }
        if self.r#waterlogged == false && self.r#persistent == true && self.r#distance == 6 {
            return 385;
        }
        if self.r#waterlogged == false && self.r#distance == 5 && self.r#persistent == true {
            return 381;
        }
        if self.r#persistent == false && self.r#distance == 4 && self.r#waterlogged == true {
            return 378;
        }
        if self.r#distance == 5 && self.r#persistent == false && self.r#waterlogged == false {
            return 383;
        }
        if self.r#waterlogged == false && self.r#persistent == true && self.r#distance == 4 {
            return 377;
        }
        if self.r#persistent == false && self.r#distance == 5 && self.r#waterlogged == true {
            return 382;
        }
        if self.r#distance == 6 && self.r#persistent == false && self.r#waterlogged == false {
            return 387;
        }
        if self.r#persistent == false && self.r#distance == 1 && self.r#waterlogged == true {
            return 366;
        }
        if self.r#persistent == false && self.r#distance == 1 && self.r#waterlogged == false {
            return 367;
        }
        if self.r#persistent == false && self.r#distance == 2 && self.r#waterlogged == false {
            return 371;
        }
        if self.r#distance == 2 && self.r#waterlogged == false && self.r#persistent == true {
            return 369;
        }
        if self.r#distance == 3 && self.r#waterlogged == false && self.r#persistent == false {
            return 375;
        }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 2 {
            return 370;
        }
        if self.r#distance == 4 && self.r#waterlogged == false && self.r#persistent == false {
            return 379;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 384 {
            return Some(AcaciaLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 380 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 368 {
            return Some(AcaciaLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 2,
            });
        }
        if state_id == 364 {
            return Some(AcaciaLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 1,
            });
        }
        if state_id == 372 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 374 {
            return Some(AcaciaLeaves {
                r#distance: 3,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 365 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 1,
            });
        }
        if state_id == 386 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 376 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 4,
            });
        }
        if state_id == 388 {
            return Some(AcaciaLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 373 {
            return Some(AcaciaLeaves {
                r#persistent: true,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 389 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 390 {
            return Some(AcaciaLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 391 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 385 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 6,
            });
        }
        if state_id == 381 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 378 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 383 {
            return Some(AcaciaLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 377 {
            return Some(AcaciaLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 4,
            });
        }
        if state_id == 382 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 387 {
            return Some(AcaciaLeaves {
                r#distance: 6,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 366 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 367 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 371 {
            return Some(AcaciaLeaves {
                r#persistent: false,
                r#distance: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 369 {
            return Some(AcaciaLeaves {
                r#distance: 2,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 375 {
            return Some(AcaciaLeaves {
                r#distance: 3,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 370 {
            return Some(AcaciaLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 2,
            });
        }
        if state_id == 379 {
            return Some(AcaciaLeaves {
                r#distance: 4,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        return None;
    }
}
