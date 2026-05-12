use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooHangingSign {
    pub attached: bool,
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for BambooHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6449; }
        if block_state.r#attached == true && block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 6420; }
        if block_state.r#rotation == 7 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6456; }
        if block_state.r#attached == true && block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 6438; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 0 { return 6442; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6417; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6413; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6411; }
        if block_state.r#attached == false && block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 6471; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 5 && block_state.r#attached == true { return 6421; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6432; }
        if block_state.r#attached == false && block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 6452; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6473; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6467; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 9 && block_state.r#attached == false { return 6460; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 4 { return 6418; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 13 { return 6468; }
        if block_state.r#rotation == 13 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6436; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6455; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 6414; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6443; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 7 { return 6425; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 9 { return 6461; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 1 { return 6444; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 8 { return 6426; }
        if block_state.r#rotation == 11 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6465; }
        if block_state.r#attached == false && block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 6451; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6454; }
        if block_state.r#rotation == 14 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6470; }
        if block_state.r#rotation == 7 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6424; }
        if block_state.r#rotation == 11 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6433; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6458; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 0 && block_state.r#attached == true { return 6410; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 12 && block_state.r#attached == true { return 6434; }
        if block_state.r#attached == true && block_state.r#rotation == 2 && block_state.r#waterlogged == false { return 6415; }
        if block_state.r#rotation == 12 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6435; }
        if block_state.r#attached == true && block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 6441; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6412; }
        if block_state.r#rotation == 3 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6448; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 10 { return 6462; }
        if block_state.r#rotation == 9 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6429; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 7 { return 6457; }
        if block_state.r#attached == true && block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 6419; }
        if block_state.r#attached == false && block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 6464; }
        if block_state.r#rotation == 13 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6437; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 && block_state.r#attached == true { return 6440; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 12 { return 6466; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6450; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6439; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 1 { return 6445; }
        if block_state.r#attached == true && block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 6423; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 6453; }
        if block_state.r#attached == false && block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 6463; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 13 { return 6469; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6428; }
        if block_state.r#attached == true && block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 6431; }
        if block_state.r#rotation == 3 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6416; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 6446; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6447; }
        if block_state.r#rotation == 8 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6459; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6427; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 6430; }
        if block_state.r#attached == true && block_state.r#rotation == 6 && block_state.r#waterlogged == true { return 6422; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 && block_state.r#attached == false { return 6472; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6449 {
            return Some(BambooHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6420 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 6456 {
            return Some(BambooHangingSign {
                r#rotation: 7,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6438 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 6442 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 6417 {
            return Some(BambooHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6413 {
            return Some(BambooHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6411 {
            return Some(BambooHangingSign {
                r#rotation: 0,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6471 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 6421 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#rotation: 5,
                r#attached: true,
            });
        }
        if state_id == 6432 {
            return Some(BambooHangingSign {
                r#rotation: 11,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6452 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 6473 {
            return Some(BambooHangingSign {
                r#rotation: 15,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6467 {
            return Some(BambooHangingSign {
                r#rotation: 12,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6460 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 9,
                r#attached: false,
            });
        }
        if state_id == 6418 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 6468 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 6436 {
            return Some(BambooHangingSign {
                r#rotation: 13,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6455 {
            return Some(BambooHangingSign {
                r#rotation: 6,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6414 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 6443 {
            return Some(BambooHangingSign {
                r#rotation: 0,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6425 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 7,
            });
        }
        if state_id == 6461 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 6444 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 1,
            });
        }
        if state_id == 6426 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 8,
            });
        }
        if state_id == 6465 {
            return Some(BambooHangingSign {
                r#rotation: 11,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6451 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 4,
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
        if state_id == 6470 {
            return Some(BambooHangingSign {
                r#rotation: 14,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6424 {
            return Some(BambooHangingSign {
                r#rotation: 7,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6433 {
            return Some(BambooHangingSign {
                r#rotation: 11,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6458 {
            return Some(BambooHangingSign {
                r#rotation: 8,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6410 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 0,
                r#attached: true,
            });
        }
        if state_id == 6434 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 12,
                r#attached: true,
            });
        }
        if state_id == 6415 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 6435 {
            return Some(BambooHangingSign {
                r#rotation: 12,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6441 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 6412 {
            return Some(BambooHangingSign {
                r#rotation: 1,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6448 {
            return Some(BambooHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6462 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 10,
            });
        }
        if state_id == 6429 {
            return Some(BambooHangingSign {
                r#rotation: 9,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6457 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 7,
            });
        }
        if state_id == 6419 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 4,
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
        if state_id == 6437 {
            return Some(BambooHangingSign {
                r#rotation: 13,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6440 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 15,
                r#attached: true,
            });
        }
        if state_id == 6466 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 12,
            });
        }
        if state_id == 6450 {
            return Some(BambooHangingSign {
                r#rotation: 4,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6439 {
            return Some(BambooHangingSign {
                r#rotation: 14,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6445 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 1,
            });
        }
        if state_id == 6423 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 6453 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 6463 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 6469 {
            return Some(BambooHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 6428 {
            return Some(BambooHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6431 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 10,
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
        if state_id == 6446 {
            return Some(BambooHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 6447 {
            return Some(BambooHangingSign {
                r#rotation: 2,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6459 {
            return Some(BambooHangingSign {
                r#rotation: 8,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6427 {
            return Some(BambooHangingSign {
                r#rotation: 8,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6430 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 6422 {
            return Some(BambooHangingSign {
                r#attached: true,
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 6472 {
            return Some(BambooHangingSign {
                r#waterlogged: true,
                r#rotation: 15,
                r#attached: false,
            });
        }
        return None;
    }
}

