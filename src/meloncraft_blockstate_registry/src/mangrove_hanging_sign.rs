use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveHangingSign {
    pub attached: bool,
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for MangroveHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 12 { return 6370; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 11 { return 6368; }
        if self.r#rotation == 14 && self.r#attached == true && self.r#waterlogged == true { return 6374; }
        if self.r#rotation == 7 && self.r#waterlogged == true && self.r#attached == false { return 6392; }
        if self.r#rotation == 7 && self.r#attached == false && self.r#waterlogged == false { return 6393; }
        if self.r#attached == false && self.r#rotation == 0 && self.r#waterlogged == true { return 6378; }
        if self.r#rotation == 9 && self.r#attached == false && self.r#waterlogged == true { return 6396; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 0 { return 6379; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 10 { return 6367; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 9 { return 6397; }
        if self.r#waterlogged == false && self.r#rotation == 7 && self.r#attached == true { return 6361; }
        if self.r#rotation == 11 && self.r#waterlogged == false && self.r#attached == true { return 6369; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 5 { return 6357; }
        if self.r#rotation == 2 && self.r#waterlogged == false && self.r#attached == true { return 6351; }
        if self.r#attached == true && self.r#rotation == 9 && self.r#waterlogged == true { return 6364; }
        if self.r#rotation == 10 && self.r#waterlogged == false && self.r#attached == false { return 6399; }
        if self.r#rotation == 15 && self.r#waterlogged == false && self.r#attached == true { return 6377; }
        if self.r#attached == true && self.r#rotation == 7 && self.r#waterlogged == true { return 6360; }
        if self.r#attached == true && self.r#rotation == 4 && self.r#waterlogged == true { return 6354; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 5 { return 6356; }
        if self.r#rotation == 2 && self.r#attached == false && self.r#waterlogged == true { return 6382; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 2 { return 6350; }
        if self.r#rotation == 5 && self.r#waterlogged == true && self.r#attached == false { return 6388; }
        if self.r#rotation == 11 && self.r#waterlogged == true && self.r#attached == false { return 6400; }
        if self.r#rotation == 12 && self.r#waterlogged == true && self.r#attached == false { return 6402; }
        if self.r#attached == false && self.r#rotation == 4 && self.r#waterlogged == true { return 6386; }
        if self.r#attached == false && self.r#rotation == 15 && self.r#waterlogged == false { return 6409; }
        if self.r#attached == false && self.r#rotation == 8 && self.r#waterlogged == true { return 6394; }
        if self.r#attached == true && self.r#waterlogged == false && self.r#rotation == 6 { return 6359; }
        if self.r#waterlogged == false && self.r#rotation == 6 && self.r#attached == false { return 6391; }
        if self.r#waterlogged == true && self.r#rotation == 14 && self.r#attached == false { return 6406; }
        if self.r#attached == false && self.r#rotation == 6 && self.r#waterlogged == true { return 6390; }
        if self.r#rotation == 14 && self.r#waterlogged == false && self.r#attached == true { return 6375; }
        if self.r#rotation == 0 && self.r#attached == true && self.r#waterlogged == true { return 6346; }
        if self.r#attached == true && self.r#rotation == 13 && self.r#waterlogged == false { return 6373; }
        if self.r#rotation == 0 && self.r#waterlogged == false && self.r#attached == true { return 6347; }
        if self.r#rotation == 12 && self.r#attached == true && self.r#waterlogged == false { return 6371; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 8 { return 6395; }
        if self.r#rotation == 13 && self.r#attached == false && self.r#waterlogged == false { return 6405; }
        if self.r#attached == false && self.r#rotation == 14 && self.r#waterlogged == false { return 6407; }
        if self.r#rotation == 15 && self.r#attached == true && self.r#waterlogged == true { return 6376; }
        if self.r#rotation == 3 && self.r#attached == false && self.r#waterlogged == true { return 6384; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 11 { return 6401; }
        if self.r#rotation == 3 && self.r#attached == false && self.r#waterlogged == false { return 6385; }
        if self.r#attached == false && self.r#rotation == 10 && self.r#waterlogged == true { return 6398; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 8 { return 6362; }
        if self.r#attached == true && self.r#rotation == 13 && self.r#waterlogged == true { return 6372; }
        if self.r#rotation == 13 && self.r#waterlogged == true && self.r#attached == false { return 6404; }
        if self.r#waterlogged == false && self.r#rotation == 4 && self.r#attached == true { return 6355; }
        if self.r#waterlogged == false && self.r#rotation == 8 && self.r#attached == true { return 6363; }
        if self.r#rotation == 3 && self.r#waterlogged == false && self.r#attached == true { return 6353; }
        if self.r#attached == false && self.r#rotation == 1 && self.r#waterlogged == false { return 6381; }
        if self.r#attached == false && self.r#rotation == 12 && self.r#waterlogged == false { return 6403; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 5 { return 6389; }
        if self.r#attached == true && self.r#rotation == 3 && self.r#waterlogged == true { return 6352; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 9 { return 6365; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 6 { return 6358; }
        if self.r#rotation == 10 && self.r#waterlogged == true && self.r#attached == true { return 6366; }
        if self.r#attached == false && self.r#rotation == 1 && self.r#waterlogged == true { return 6380; }
        if self.r#rotation == 4 && self.r#waterlogged == false && self.r#attached == false { return 6387; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 1 { return 6349; }
        if self.r#rotation == 2 && self.r#waterlogged == false && self.r#attached == false { return 6383; }
        if self.r#attached == false && self.r#waterlogged == true && self.r#rotation == 15 { return 6408; }
        if self.r#rotation == 1 && self.r#attached == true && self.r#waterlogged == true { return 6348; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6370 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 6368 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 6374 {
            return Some(MangroveHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6392 {
            return Some(MangroveHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6393 {
            return Some(MangroveHangingSign {
                r#rotation: 7,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6378 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 6396 {
            return Some(MangroveHangingSign {
                r#rotation: 9,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6379 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 0,
            });
        }
        if state_id == 6367 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 10,
            });
        }
        if state_id == 6397 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 6361 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#rotation: 7,
                r#attached: true,
            });
        }
        if state_id == 6369 {
            return Some(MangroveHangingSign {
                r#rotation: 11,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6357 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 5,
            });
        }
        if state_id == 6351 {
            return Some(MangroveHangingSign {
                r#rotation: 2,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6364 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6399 {
            return Some(MangroveHangingSign {
                r#rotation: 10,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6377 {
            return Some(MangroveHangingSign {
                r#rotation: 15,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6360 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 6354 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 6356 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 6382 {
            return Some(MangroveHangingSign {
                r#rotation: 2,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6350 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 6388 {
            return Some(MangroveHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6400 {
            return Some(MangroveHangingSign {
                r#rotation: 11,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6402 {
            return Some(MangroveHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6386 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 6409 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 6394 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 6359 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 6391 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#rotation: 6,
                r#attached: false,
            });
        }
        if state_id == 6406 {
            return Some(MangroveHangingSign {
                r#waterlogged: true,
                r#rotation: 14,
                r#attached: false,
            });
        }
        if state_id == 6390 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 6375 {
            return Some(MangroveHangingSign {
                r#rotation: 14,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6346 {
            return Some(MangroveHangingSign {
                r#rotation: 0,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6373 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 6347 {
            return Some(MangroveHangingSign {
                r#rotation: 0,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6371 {
            return Some(MangroveHangingSign {
                r#rotation: 12,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6395 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 6405 {
            return Some(MangroveHangingSign {
                r#rotation: 13,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6407 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 6376 {
            return Some(MangroveHangingSign {
                r#rotation: 15,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6384 {
            return Some(MangroveHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6401 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 6385 {
            return Some(MangroveHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6398 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 6362 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 6372 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 6404 {
            return Some(MangroveHangingSign {
                r#rotation: 13,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6355 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#rotation: 4,
                r#attached: true,
            });
        }
        if state_id == 6363 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#rotation: 8,
                r#attached: true,
            });
        }
        if state_id == 6353 {
            return Some(MangroveHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6381 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 6403 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 6389 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 5,
            });
        }
        if state_id == 6352 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 6365 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 9,
            });
        }
        if state_id == 6358 {
            return Some(MangroveHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 6366 {
            return Some(MangroveHangingSign {
                r#rotation: 10,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6380 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 6387 {
            return Some(MangroveHangingSign {
                r#rotation: 4,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6349 {
            return Some(MangroveHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 1,
            });
        }
        if state_id == 6383 {
            return Some(MangroveHangingSign {
                r#rotation: 2,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6408 {
            return Some(MangroveHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 6348 {
            return Some(MangroveHangingSign {
                r#rotation: 1,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

