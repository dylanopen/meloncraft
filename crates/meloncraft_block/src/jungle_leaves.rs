use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleLeaves {
    pub persistent: bool,
    pub distance: i32,
    pub waterlogged: bool,
}


impl BlockState for JungleLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#distance == 2 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 341; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 2 { return 342; }
        if block_state.r#distance == 3 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 345; }
        if block_state.r#waterlogged == true && block_state.r#persistent == true && block_state.r#distance == 2 { return 340; }
        if block_state.r#distance == 3 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 346; }
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 1 { return 337; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == false { return 347; }
        if block_state.r#distance == 4 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 350; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 5 { return 355; }
        if block_state.r#waterlogged == true && block_state.r#distance == 3 && block_state.r#persistent == true { return 344; }
        if block_state.r#waterlogged == true && block_state.r#distance == 7 && block_state.r#persistent == true { return 360; }
        if block_state.r#distance == 7 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 363; }
        if block_state.r#distance == 5 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 354; }
        if block_state.r#distance == 1 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 336; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 1 { return 339; }
        if block_state.r#waterlogged == false && block_state.r#distance == 2 && block_state.r#persistent == false { return 343; }
        if block_state.r#waterlogged == false && block_state.r#persistent == false && block_state.r#distance == 4 { return 351; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 358; }
        if block_state.r#waterlogged == true && block_state.r#distance == 5 && block_state.r#persistent == true { return 352; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == false && block_state.r#persistent == false { return 359; }
        if block_state.r#distance == 1 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 338; }
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 4 { return 349; }
        if block_state.r#waterlogged == false && block_state.r#distance == 7 && block_state.r#persistent == true { return 361; }
        if block_state.r#persistent == true && block_state.r#waterlogged == false && block_state.r#distance == 5 { return 353; }
        if block_state.r#persistent == true && block_state.r#distance == 6 && block_state.r#waterlogged == true { return 356; }
        if block_state.r#distance == 6 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 357; }
        if block_state.r#distance == 4 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 348; }
        if block_state.r#persistent == false && block_state.r#waterlogged == true && block_state.r#distance == 7 { return 362; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 341 {
            return Some(JungleLeaves {
                r#distance: 2,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 342 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 2,
            });
        }
        if state_id == 345 {
            return Some(JungleLeaves {
                r#distance: 3,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 340 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 2,
            });
        }
        if state_id == 346 {
            return Some(JungleLeaves {
                r#distance: 3,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 337 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 347 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 350 {
            return Some(JungleLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 355 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 5,
            });
        }
        if state_id == 344 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#distance: 3,
                r#persistent: true,
            });
        }
        if state_id == 360 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 363 {
            return Some(JungleLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 354 {
            return Some(JungleLeaves {
                r#distance: 5,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 336 {
            return Some(JungleLeaves {
                r#distance: 1,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 339 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 343 {
            return Some(JungleLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 351 {
            return Some(JungleLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 4,
            });
        }
        if state_id == 358 {
            return Some(JungleLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 352 {
            return Some(JungleLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 359 {
            return Some(JungleLeaves {
                r#distance: 6,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 338 {
            return Some(JungleLeaves {
                r#distance: 1,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 349 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 4,
            });
        }
        if state_id == 361 {
            return Some(JungleLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 353 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 5,
            });
        }
        if state_id == 356 {
            return Some(JungleLeaves {
                r#persistent: true,
                r#distance: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 357 {
            return Some(JungleLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 348 {
            return Some(JungleLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 362 {
            return Some(JungleLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 7,
            });
        }
        return None;
    }
}

