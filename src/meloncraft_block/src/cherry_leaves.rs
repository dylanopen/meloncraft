use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryLeaves {
    pub persistent: bool,
    pub distance: i32,
    pub waterlogged: bool,
}


impl BlockState for CherryLeaves {
    fn to_id(&self) -> i32 {
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 3 { return 400; }
        if self.r#distance == 5 && self.r#persistent == true && self.r#waterlogged == true { return 408; }
        if self.r#waterlogged == true && self.r#persistent == false && self.r#distance == 3 { return 402; }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 6 { return 415; }
        if self.r#waterlogged == false && self.r#persistent == true && self.r#distance == 6 { return 413; }
        if self.r#waterlogged == true && self.r#persistent == true && self.r#distance == 7 { return 416; }
        if self.r#waterlogged == true && self.r#distance == 7 && self.r#persistent == false { return 418; }
        if self.r#distance == 7 && self.r#waterlogged == false && self.r#persistent == false { return 419; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 1 { return 393; }
        if self.r#waterlogged == false && self.r#distance == 4 && self.r#persistent == true { return 405; }
        if self.r#waterlogged == false && self.r#distance == 2 && self.r#persistent == true { return 397; }
        if self.r#waterlogged == false && self.r#distance == 4 && self.r#persistent == false { return 407; }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 5 { return 410; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 3 { return 401; }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 2 { return 398; }
        if self.r#waterlogged == true && self.r#distance == 4 && self.r#persistent == false { return 406; }
        if self.r#waterlogged == true && self.r#distance == 1 && self.r#persistent == true { return 392; }
        if self.r#persistent == true && self.r#waterlogged == false && self.r#distance == 5 { return 409; }
        if self.r#waterlogged == false && self.r#persistent == false && self.r#distance == 5 { return 411; }
        if self.r#persistent == true && self.r#waterlogged == true && self.r#distance == 6 { return 412; }
        if self.r#distance == 1 && self.r#waterlogged == true && self.r#persistent == false { return 394; }
        if self.r#distance == 7 && self.r#waterlogged == false && self.r#persistent == true { return 417; }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 1 { return 395; }
        if self.r#persistent == false && self.r#waterlogged == false && self.r#distance == 3 { return 403; }
        if self.r#distance == 2 && self.r#waterlogged == true && self.r#persistent == true { return 396; }
        if self.r#persistent == false && self.r#distance == 2 && self.r#waterlogged == false { return 399; }
        if self.r#waterlogged == true && self.r#persistent == true && self.r#distance == 4 { return 404; }
        if self.r#persistent == false && self.r#waterlogged == true && self.r#distance == 6 { return 414; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 400 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 3,
            });
        }
        if state_id == 408 {
            return Some(CherryLeaves {
                r#distance: 5,
                r#persistent: true,
                r#waterlogged: true,
            });
        }
        if state_id == 402 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#persistent: false,
                r#distance: 3,
            });
        }
        if state_id == 415 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 6,
            });
        }
        if state_id == 413 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#persistent: true,
                r#distance: 6,
            });
        }
        if state_id == 416 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 7,
            });
        }
        if state_id == 418 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#distance: 7,
                r#persistent: false,
            });
        }
        if state_id == 419 {
            return Some(CherryLeaves {
                r#distance: 7,
                r#waterlogged: false,
                r#persistent: false,
            });
        }
        if state_id == 393 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 405 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: true,
            });
        }
        if state_id == 397 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#distance: 2,
                r#persistent: true,
            });
        }
        if state_id == 407 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#distance: 4,
                r#persistent: false,
            });
        }
        if state_id == 410 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 5,
            });
        }
        if state_id == 401 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 398 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 2,
            });
        }
        if state_id == 406 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#distance: 4,
                r#persistent: false,
            });
        }
        if state_id == 392 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#distance: 1,
                r#persistent: true,
            });
        }
        if state_id == 409 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#waterlogged: false,
                r#distance: 5,
            });
        }
        if state_id == 411 {
            return Some(CherryLeaves {
                r#waterlogged: false,
                r#persistent: false,
                r#distance: 5,
            });
        }
        if state_id == 412 {
            return Some(CherryLeaves {
                r#persistent: true,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        if state_id == 394 {
            return Some(CherryLeaves {
                r#distance: 1,
                r#waterlogged: true,
                r#persistent: false,
            });
        }
        if state_id == 417 {
            return Some(CherryLeaves {
                r#distance: 7,
                r#waterlogged: false,
                r#persistent: true,
            });
        }
        if state_id == 395 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 1,
            });
        }
        if state_id == 403 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: false,
                r#distance: 3,
            });
        }
        if state_id == 396 {
            return Some(CherryLeaves {
                r#distance: 2,
                r#waterlogged: true,
                r#persistent: true,
            });
        }
        if state_id == 399 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#distance: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 404 {
            return Some(CherryLeaves {
                r#waterlogged: true,
                r#persistent: true,
                r#distance: 4,
            });
        }
        if state_id == 414 {
            return Some(CherryLeaves {
                r#persistent: false,
                r#waterlogged: true,
                r#distance: 6,
            });
        }
        return None;
    }
}

