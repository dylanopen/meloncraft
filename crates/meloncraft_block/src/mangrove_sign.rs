use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveSign {
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for MangroveSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 5421; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 10 { return 5410; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5409; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == false { return 5395; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false { return 5397; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 5398; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5402; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 5413; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 5 { return 5400; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 5399; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 5394; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 5403; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 9 { return 5408; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5416; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 5420; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 0 { return 5391; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 { return 5405; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 5407; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5393; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 5404; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 8 { return 5406; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 5414; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 5401; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5396; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 0 { return 5390; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 14 { return 5418; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5411; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 { return 5419; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5392; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 5415; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5417; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5412; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5421 {
            return Some(MangroveSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5410 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 5409 {
            return Some(MangroveSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5395 {
            return Some(MangroveSign {
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5397 {
            return Some(MangroveSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5398 {
            return Some(MangroveSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5402 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5413 {
            return Some(MangroveSign {
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5400 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 5399 {
            return Some(MangroveSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5394 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5403 {
            return Some(MangroveSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 5408 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 5416 {
            return Some(MangroveSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5420 {
            return Some(MangroveSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5391 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 5405 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5407 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5393 {
            return Some(MangroveSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5404 {
            return Some(MangroveSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5406 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 5414 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5401 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5396 {
            return Some(MangroveSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5390 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 5418 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 5411 {
            return Some(MangroveSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5419 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5392 {
            return Some(MangroveSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5415 {
            return Some(MangroveSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5417 {
            return Some(MangroveSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5412 {
            return Some(MangroveSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        return None;
    }
}

