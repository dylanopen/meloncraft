use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonSign {
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for CrimsonSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 21471; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 21460; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false { return 21447; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 21452; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 21444; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 21459; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 11 { return 21463; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 21442; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 7 { return 21454; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 21456; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 21458; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 21449; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false { return 21457; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 21470; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 21455; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 21469; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 21453; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 21443; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false { return 21467; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 21461; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 21464; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 13 { return 21466; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 21441; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 21445; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 21448; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 21440; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 21450; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 14 { return 21468; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 21462; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 21446; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 21451; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 21465; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21471 {
            return Some(CrimsonSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 21460 {
            return Some(CrimsonSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 21447 {
            return Some(CrimsonSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 21452 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 21444 {
            return Some(CrimsonSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 21459 {
            return Some(CrimsonSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 21463 {
            return Some(CrimsonSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 21442 {
            return Some(CrimsonSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 21454 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 21456 {
            return Some(CrimsonSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 21458 {
            return Some(CrimsonSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 21449 {
            return Some(CrimsonSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 21457 {
            return Some(CrimsonSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 21470 {
            return Some(CrimsonSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 21455 {
            return Some(CrimsonSign {
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 21469 {
            return Some(CrimsonSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 21453 {
            return Some(CrimsonSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 21443 {
            return Some(CrimsonSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 21467 {
            return Some(CrimsonSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 21461 {
            return Some(CrimsonSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 21464 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 21466 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 21441 {
            return Some(CrimsonSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 21445 {
            return Some(CrimsonSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 21448 {
            return Some(CrimsonSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 21440 {
            return Some(CrimsonSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 21450 {
            return Some(CrimsonSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 21468 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 21462 {
            return Some(CrimsonSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 21446 {
            return Some(CrimsonSign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 21451 {
            return Some(CrimsonSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 21465 {
            return Some(CrimsonSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

