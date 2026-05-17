use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooHangingSign {
    pub rotation: i32,
    pub attached: bool,
    pub waterlogged: bool,
}


impl BlockState for BambooHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#attached == true && self.r#rotation == 6 && self.r#waterlogged == false { return 6423; }
        if self.r#attached == true && self.r#rotation == 8 && self.r#waterlogged == true { return 6426; }
        if self.r#attached == true && self.r#rotation == 7 && self.r#waterlogged == false { return 6425; }
        if self.r#waterlogged == true && self.r#rotation == 10 && self.r#attached == true { return 6430; }
        if self.r#attached == false && self.r#rotation == 0 && self.r#waterlogged == false { return 6443; }
        if self.r#rotation == 6 && self.r#waterlogged == true && self.r#attached == false { return 6454; }
        if self.r#rotation == 5 && self.r#waterlogged == true && self.r#attached == false { return 6452; }
        if self.r#rotation == 4 && self.r#waterlogged == true && self.r#attached == true { return 6418; }
        if self.r#attached == false && self.r#waterlogged == true && self.r#rotation == 10 { return 6462; }
        if self.r#rotation == 5 && self.r#waterlogged == false && self.r#attached == false { return 6453; }
        if self.r#rotation == 3 && self.r#attached == true && self.r#waterlogged == false { return 6417; }
        if self.r#attached == true && self.r#rotation == 4 && self.r#waterlogged == false { return 6419; }
        if self.r#rotation == 15 && self.r#waterlogged == false && self.r#attached == true { return 6441; }
        if self.r#waterlogged == true && self.r#rotation == 15 && self.r#attached == false { return 6472; }
        if self.r#waterlogged == true && self.r#rotation == 9 && self.r#attached == true { return 6428; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 9 { return 6429; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 15 { return 6473; }
        if self.r#attached == false && self.r#rotation == 7 && self.r#waterlogged == false { return 6457; }
        if self.r#attached == false && self.r#rotation == 11 && self.r#waterlogged == true { return 6464; }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 7 { return 6424; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 11 { return 6465; }
        if self.r#attached == true && self.r#rotation == 12 && self.r#waterlogged == false { return 6435; }
        if self.r#rotation == 3 && self.r#attached == true && self.r#waterlogged == true { return 6416; }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 4 { return 6450; }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 11 { return 6432; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 10 { return 6463; }
        if self.r#attached == true && self.r#rotation == 13 && self.r#waterlogged == true { return 6436; }
        if self.r#rotation == 14 && self.r#waterlogged == true && self.r#attached == true { return 6438; }
        if self.r#waterlogged == true && self.r#rotation == 14 && self.r#attached == false { return 6470; }
        if self.r#rotation == 0 && self.r#waterlogged == true && self.r#attached == true { return 6410; }
        if self.r#attached == false && self.r#rotation == 2 && self.r#waterlogged == true { return 6446; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 12 { return 6467; }
        if self.r#attached == false && self.r#waterlogged == true && self.r#rotation == 0 { return 6442; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 8 { return 6459; }
        if self.r#rotation == 4 && self.r#waterlogged == false && self.r#attached == false { return 6451; }
        if self.r#rotation == 12 && self.r#attached == false && self.r#waterlogged == true { return 6466; }
        if self.r#rotation == 6 && self.r#waterlogged == true && self.r#attached == true { return 6422; }
        if self.r#attached == true && self.r#rotation == 14 && self.r#waterlogged == false { return 6439; }
        if self.r#attached == true && self.r#rotation == 8 && self.r#waterlogged == false { return 6427; }
        if self.r#rotation == 13 && self.r#attached == true && self.r#waterlogged == false { return 6437; }
        if self.r#waterlogged == false && self.r#rotation == 0 && self.r#attached == true { return 6411; }
        if self.r#attached == true && self.r#waterlogged == false && self.r#rotation == 10 { return 6431; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 1 { return 6413; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 14 { return 6471; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 6 { return 6455; }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 13 { return 6468; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 2 { return 6414; }
        if self.r#attached == false && self.r#rotation == 3 && self.r#waterlogged == true { return 6448; }
        if self.r#waterlogged == true && self.r#rotation == 1 && self.r#attached == true { return 6412; }
        if self.r#rotation == 2 && self.r#attached == true && self.r#waterlogged == false { return 6415; }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 8 { return 6458; }
        if self.r#attached == true && self.r#rotation == 12 && self.r#waterlogged == true { return 6434; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 15 { return 6440; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 2 { return 6447; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 3 { return 6449; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 13 { return 6469; }
        if self.r#rotation == 5 && self.r#attached == true && self.r#waterlogged == false { return 6421; }
        if self.r#rotation == 11 && self.r#attached == true && self.r#waterlogged == false { return 6433; }
        if self.r#rotation == 9 && self.r#waterlogged == false && self.r#attached == false { return 6461; }
        if self.r#rotation == 5 && self.r#attached == true && self.r#waterlogged == true { return 6420; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 1 { return 6445; }
        if self.r#waterlogged == true && self.r#rotation == 9 && self.r#attached == false { return 6460; }
        if self.r#rotation == 7 && self.r#attached == false && self.r#waterlogged == true { return 6456; }
        if self.r#waterlogged == true && self.r#rotation == 1 && self.r#attached == false { return 6444; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6423 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 6426 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 6425 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 6430 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 10,
                r#attached: true,
            });
        }
        if state_id == 6443 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 6454 {
            return Some(BambooHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6452 {
            return Some(BambooHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6418 {
            return Some(BambooHangingSign {
                r#rotation: 4,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6462 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 6453 {
            return Some(BambooHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6417 {
            return Some(BambooHangingSign {
                r#rotation: 3,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6419 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 6441 {
            return Some(BambooHangingSign {
                r#rotation: 15,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6472 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 15,
                r#attached: false,
            });
        }
        if state_id == 6428 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 9,
                r#attached: true,
            });
        }
        if state_id == 6429 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 9,
            });
        }
        if state_id == 6473 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 6457 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 6464 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 6424 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 7,
            });
        }
        if state_id == 6465 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 6435 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 6416 {
            return Some(BambooHangingSign {
                r#rotation: 3,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6450 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 6432 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 11,
            });
        }
        if state_id == 6463 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 10,
            });
        }
        if state_id == 6436 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 6438 {
            return Some(BambooHangingSign {
                r#rotation: 14,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6470 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 14,
                r#attached: false,
            });
        }
        if state_id == 6410 {
            return Some(BambooHangingSign {
                r#rotation: 0,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6446 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 6467 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 12,
            });
        }
        if state_id == 6442 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 6459 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 6451 {
            return Some(BambooHangingSign {
                r#rotation: 4,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6466 {
            return Some(BambooHangingSign {
                r#rotation: 12,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6422 {
            return Some(BambooHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6439 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 6427 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 6437 {
            return Some(BambooHangingSign {
                r#rotation: 13,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6411 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#rotation: 0,
                r#attached: true,
            });
        }
        if state_id == 6431 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 6413 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 1,
            });
        }
        if state_id == 6471 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 14,
            });
        }
        if state_id == 6455 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 6,
            });
        }
        if state_id == 6468 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 6414 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 6448 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 6412 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 1,
                r#attached: true,
            });
        }
        if state_id == 6415 {
            return Some(BambooHangingSign {
                r#rotation: 2,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6458 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 8,
            });
        }
        if state_id == 6434 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 6440 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 6447 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 6449 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 3,
            });
        }
        if state_id == 6469 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 6421 {
            return Some(BambooHangingSign {
                r#rotation: 5,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6433 {
            return Some(BambooHangingSign {
                r#rotation: 11,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6461 {
            return Some(BambooHangingSign {
                r#rotation: 9,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6420 {
            return Some(BambooHangingSign {
                r#rotation: 5,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6445 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 1,
            });
        }
        if state_id == 6460 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 9,
                r#attached: false,
            });
        }
        if state_id == 6456 {
            return Some(BambooHangingSign {
                r#rotation: 7,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6444 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 1,
                r#attached: false,
            });
        }
        return None;
    }
}

