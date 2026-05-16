use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceLeaves {
    pub distance: i32,
    pub waterlogged: bool,
    pub persistent: bool,
}


impl BlockState for SpruceLeaves {
    fn to_id(&self) -> i32 {
        if self.r#distance == 3 && self.r#waterlogged == true && self.r#persistent == true { return 288; }
        if self.r#distance == 2 && self.r#persistent == true && self.r#waterlogged == false { return 285; }
        if self.r#distance == 4 && self.r#waterlogged == true && self.r#persistent == true { return 292; }
        if self.r#persistent == false && self.r#distance == 3 && self.r#waterlogged == true { return 290; }
        if self.r#distance == 5 && self.r#persistent == false && self.r#waterlogged == true { return 298; }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 6 { return 300; }
        if self.r#distance == 2 && self.r#persistent == false && self.r#waterlogged == false { return 287; }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 2 { return 284; }
        if self.r#distance == 4 && self.r#persistent == false && self.r#waterlogged == false { return 295; }
        if self.r#distance == 1 && self.r#persistent == true && self.r#waterlogged == true { return 280; }
        if self.r#distance == 3 && self.r#persistent == true && self.r#waterlogged == false { return 289; }
        if self.r#waterlogged == false && self.r#distance == 5 && self.r#persistent == true { return 297; }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 6 { return 302; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 1 { return 281; }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 5 { return 296; }
        if self.r#waterlogged == false && self.r#distance == 7 && self.r#persistent == true { return 305; }
        if self.r#waterlogged == true && self.r#distance == 2 && self.r#persistent == false { return 286; }
        if self.r#distance == 4 && self.r#waterlogged == false && self.r#persistent == true { return 293; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 6 { return 301; }
        if self.r#waterlogged == false && self.r#distance == 3 && self.r#persistent == false { return 291; }
        if self.r#persistent == false && self.r#distance == 6 && self.r#waterlogged == false { return 303; }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 1 { return 283; }
        if self.r#distance == 4 && self.r#waterlogged == true && self.r#persistent == false { return 294; }
        if self.r#distance == 5 && self.r#waterlogged == false && self.r#persistent == false { return 299; }
        if self.r#distance == 7 && self.r#persistent == true && self.r#waterlogged == true { return 304; }
        if self.r#distance == 7 && self.r#persistent == false && self.r#waterlogged == true { return 306; }
        if self.r#distance == 7 && self.r#waterlogged == false && self.r#persistent == false { return 307; }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 1 { return 282; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 288 {
            return Some(SpruceLeaves {
                r#distance: 3,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 285 {
            return Some(SpruceLeaves {
                r#distance: 2,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 292 {
            return Some(SpruceLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 290 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#distance: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 298 {
            return Some(SpruceLeaves {
                r#distance: 5,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 300 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 287 {
            return Some(SpruceLeaves {
                r#distance: 2,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 284 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 2,
            });
        }
        if state_id == 295 {
            return Some(SpruceLeaves {
                r#distance: 4,
                r#persistent: false,
                r#waterlogged: false,
            });
        }
        if state_id == 280 {
            return Some(SpruceLeaves {
                r#distance: 1,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 289 {
            return Some(SpruceLeaves {
                r#distance: 3,
                r#persistent: true,
                r#waterlogged: false,
            });
        }
        if state_id == 297 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#distance: 5,
                r#persistent: true,
            });
        }
        if state_id == 302 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 281 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 296 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 305 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#distance: 7,
                r#persistent: true,
            });
        }
        if state_id == 286 {
            return Some(SpruceLeaves {
                r#waterlogged: true,
                r#distance: 2,
                r#persistent: false,
            });
        }
        if state_id == 293 {
            return Some(SpruceLeaves {
                r#distance: 4,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 301 {
            return Some(SpruceLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 291 {
            return Some(SpruceLeaves {
                r#waterlogged: false,
                r#distance: 3,
                r#persistent: false,
            });
        }
        if state_id == 303 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#distance: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 283 {
            return Some(SpruceLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 294 {
            return Some(SpruceLeaves {
                r#distance: 4,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 299 {
            return Some(SpruceLeaves {
                r#distance: 5,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 304 {
            return Some(SpruceLeaves {
                r#distance: 7,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 306 {
            return Some(SpruceLeaves {
                r#distance: 7,
                r#persistent: false,
                r#waterlogged: true,
            });
        }
        if state_id == 307 {
            return Some(SpruceLeaves {
                r#distance: 7,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 282 {
            return Some(SpruceLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 1,
            });
        }
        return None;
    }
}

