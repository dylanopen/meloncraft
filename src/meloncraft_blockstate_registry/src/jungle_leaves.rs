use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleLeaves {
    pub waterlogged: bool,
    pub persistent: bool,
    pub distance: i32,
}

impl BlockState for JungleLeaves {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#distance == 1 && self.r#persistent == false {
            return 339;
        }
        if self.r#persistent == false && self.r#distance == 2 && self.r#waterlogged == false {
            return 343;
        }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 6 {
            return 359;
        }
        if self.r#persistent == true && self.r#distance == 5 && self.r#waterlogged == true {
            return 352;
        }
        if self.r#distance == 2 && self.r#persistent == true && self.r#waterlogged == false {
            return 341;
        }
        if self.r#waterlogged == false && self.r#distance == 5 && self.r#persistent == false {
            return 355;
        }
        if self.r#persistent == false && self.r#distance == 4 && self.r#waterlogged == false {
            return 351;
        }
        if self.r#waterlogged == true && self.r#persistent == true && self.r#distance == 4 {
            return 348;
        }
        if self.r#distance == 5 && self.r#persistent == true && self.r#waterlogged == false {
            return 353;
        }
        if self.r#distance == 3 && self.r#persistent == true && self.r#waterlogged == true {
            return 344;
        }
        if self.r#waterlogged == true && self.r#distance == 7 && self.r#persistent == true {
            return 360;
        }
        if self.r#distance == 3 && self.r#persistent == false && self.r#waterlogged == false {
            return 347;
        }
        if self.r#distance == 1 && self.r#persistent == false && self.r#waterlogged == true {
            return 338;
        }
        if self.r#persistent == false && self.r#distance == 2 && self.r#waterlogged == true {
            return 342;
        }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 3 {
            return 345;
        }
        if self.r#distance == 4 && self.r#waterlogged == false && self.r#persistent == true {
            return 349;
        }
        if self.r#waterlogged == true && self.r#distance == 5 && self.r#persistent == false {
            return 354;
        }
        if self.r#persistent == true && self.r#distance == 1 && self.r#waterlogged == true {
            return 336;
        }
        if self.r#waterlogged == true && self.r#distance == 3 && self.r#persistent == false {
            return 346;
        }
        if self.r#distance == 6 && self.r#waterlogged == true && self.r#persistent == false {
            return 358;
        }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 4 {
            return 350;
        }
        if self.r#distance == 6 && self.r#waterlogged == true && self.r#persistent == true {
            return 356;
        }
        if self.r#distance == 6 && self.r#persistent == true && self.r#waterlogged == false {
            return 357;
        }
        if self.r#distance == 7 && self.r#persistent == true && self.r#waterlogged == false {
            return 361;
        }
        if self.r#waterlogged == true && self.r#distance == 7 && self.r#persistent == false {
            return 362;
        }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 2 {
            return 340;
        }
        if self.r#distance == 7 && self.r#waterlogged == false && self.r#persistent == false {
            return 363;
        }
        if self.r#distance == 1 && self.r#persistent == true && self.r#waterlogged == false {
            return 337;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 339 {
            return Some(JungleLeaves {
                r#waterlogged: false,
                r#distance: 1,
                r#persistent: false,
            });
        }
        if state_id == 343 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#distance: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 359 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 352 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#distance: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 341 {
            return Some(JungleLeaves {
                r#distance: 2,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 355 {
            return Some(JungleLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 351 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#distance: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 348 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 4,
            });
        }
        if state_id == 353 {
            return Some(JungleLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 344 {
            return Some(JungleLeaves {
                r#distance: 3,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 360 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 347 {
            return Some(JungleLeaves {
                r#distance: 3,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 338 {
            return Some(JungleLeaves {
                r#distance: 1,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 342 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#distance: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 345 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 349 {
            return Some(JungleLeaves {
                r#distance: 4,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 354 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 336 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#distance: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 346 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#distance: 3,
                r#persistent: false,
            });
        }
        if state_id == 358 {
            return Some(JungleLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 350 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 4,
            });
        }
        if state_id == 356 {
            return Some(JungleLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 357 {
            return Some(JungleLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 361 {
            return Some(JungleLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 362 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 340 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 2,
            });
        }
        if state_id == 363 {
            return Some(JungleLeaves {
                r#distance: 7,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 337 {
            return Some(JungleLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
