use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleSign {
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for JungleSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 5298; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5295; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 5324; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5297; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5299; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5306; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 5308; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 5309; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5321; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 5310; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 5305; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 5325; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5300; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 5301; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 6 { return 5307; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 5322; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 { return 5323; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 5317; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 5 { return 5304; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5315; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 5316; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 5302; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 5314; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 1 { return 5296; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5318; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false { return 5311; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 5319; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 5294; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 9 { return 5312; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 5303; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5313; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5320; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5298 {
            return Some(JungleSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5295 {
            return Some(JungleSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5324 {
            return Some(JungleSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5297 {
            return Some(JungleSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5299 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5306 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5308 {
            return Some(JungleSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5309 {
            return Some(JungleSign {
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5321 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5310 {
            return Some(JungleSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5305 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5325 {
            return Some(JungleSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5300 {
            return Some(JungleSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5301 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5307 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5322 {
            return Some(JungleSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5323 {
            return Some(JungleSign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5317 {
            return Some(JungleSign {
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5304 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5315 {
            return Some(JungleSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5316 {
            return Some(JungleSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5302 {
            return Some(JungleSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5314 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5296 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 5318 {
            return Some(JungleSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5311 {
            return Some(JungleSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5319 {
            return Some(JungleSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5294 {
            return Some(JungleSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5312 {
            return Some(JungleSign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 5303 {
            return Some(JungleSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5313 {
            return Some(JungleSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5320 {
            return Some(JungleSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

