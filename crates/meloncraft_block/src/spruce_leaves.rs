use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceLeaves {
    pub waterlogged: bool,
    pub persistent: bool,
    pub distance: i32,
}


impl BlockState for SpruceLeaves {
    fn to_id(self) -> i32 {
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 3 { return 291; }
        if block_state.r#persistent == true && block_state.r#distance == 4 && block_state.r#waterlogged == false { return 293; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 3 { return 289; }
        if block_state.r#persistent == false && block_state.r#distance == 4 && block_state.r#waterlogged == false { return 295; }
        if block_state.r#distance == 6 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 302; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 6 { return 300; }
        if block_state.r#persistent == true && block_state.r#distance == 4 && block_state.r#waterlogged == true { return 292; }
        if block_state.r#persistent == true && block_state.r#waterlogged == true && block_state.r#distance == 5 { return 296; }
        if block_state.r#distance == 5 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 297; }
        if block_state.r#waterlogged == false && block_state.r#persistent == false && block_state.r#distance == 6 { return 303; }
        if block_state.r#persistent == false && block_state.r#distance == 3 && block_state.r#waterlogged == true { return 290; }
        if block_state.r#distance == 1 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 282; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 2 { return 285; }
        if block_state.r#distance == 4 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 294; }
        if block_state.r#persistent == true && block_state.r#distance == 7 && block_state.r#waterlogged == false { return 305; }
        if block_state.r#distance == 7 && block_state.r#waterlogged == true && block_state.r#persistent == false { return 306; }
        if block_state.r#distance == 1 && block_state.r#waterlogged == true && block_state.r#persistent == true { return 280; }
        if block_state.r#persistent == false && block_state.r#distance == 1 && block_state.r#waterlogged == false { return 283; }
        if block_state.r#distance == 2 && block_state.r#persistent == false && block_state.r#waterlogged == false { return 287; }
        if block_state.r#distance == 2 && block_state.r#persistent == false && block_state.r#waterlogged == true { return 286; }
        if block_state.r#distance == 7 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 304; }
        if block_state.r#persistent == false && block_state.r#waterlogged == false && block_state.r#distance == 7 { return 307; }
        if block_state.r#persistent == true && block_state.r#distance == 2 && block_state.r#waterlogged == true { return 284; }
        if block_state.r#waterlogged == false && block_state.r#persistent == true && block_state.r#distance == 1 { return 281; }
        if block_state.r#distance == 6 && block_state.r#persistent == true && block_state.r#waterlogged == false { return 301; }
        if block_state.r#waterlogged == true && block_state.r#distance == 5 && block_state.r#persistent == false { return 298; }
        if block_state.r#waterlogged == false && block_state.r#persistent == false && block_state.r#distance == 5 { return 299; }
        if block_state.r#distance == 3 && block_state.r#persistent == true && block_state.r#waterlogged == true { return 288; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 291 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 293 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#distance: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 289 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 3,
            });
        }
        if state_id == 295 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#distance: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 302 {
            return Some(SpruceLeaves {
                r#distance: 6,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 300 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 292 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#distance: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 296 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 297 {
            return Some(SpruceLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 303 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 6,
            });
        }
        if state_id == 290 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 282 {
            return Some(SpruceLeaves {
                r#distance: 1,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 285 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 2,
            });
        }
        if state_id == 294 {
            return Some(SpruceLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 305 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#distance: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 306 {
            return Some(SpruceLeaves {
                r#distance: 7,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 280 {
            return Some(SpruceLeaves {
                r#distance: 1,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 283 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#distance: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 287 {
            return Some(SpruceLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 286 {
            return Some(SpruceLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 304 {
            return Some(SpruceLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 307 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 7,
            });
        }
        if state_id == 284 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#distance: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 281 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 1,
            });
        }
        if state_id == 301 {
            return Some(SpruceLeaves {
                r#distance: 6,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 298 {
            return Some(SpruceLeaves {
                r#waterlogged: true,
                r#distance: 5,
                r#persistent: false,
            });
        }
        if state_id == 299 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 5,
            });
        }
        if state_id == 288 {
            return Some(SpruceLeaves {
                r#distance: 3,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

