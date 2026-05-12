use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedHangingSign {
    pub rotation: i32,
    pub attached: bool,
    pub waterlogged: bool,
}


impl BlockState for WarpedHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 11 { return 6337; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6305; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6325; }
        if block_state.r#rotation == 14 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6310; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6313; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 13 { return 6340; }
        if block_state.r#attached == true && block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 6312; }
        if block_state.r#attached == true && block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 6282; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6326; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 0 && block_state.r#attached == true { return 6283; }
        if block_state.r#rotation == 4 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6322; }
        if block_state.r#attached == true && block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 6291; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6342; }
        if block_state.r#rotation == 8 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6298; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6341; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 1 { return 6316; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6306; }
        if block_state.r#rotation == 6 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6294; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6331; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6303; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 6338; }
        if block_state.r#rotation == 6 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6327; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6289; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 11 { return 6304; }
        if block_state.r#attached == true && block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 6295; }
        if block_state.r#attached == false && block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 6336; }
        if block_state.r#rotation == 1 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6284; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 9 { return 6333; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 3 { return 6288; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 5 && block_state.r#attached == true { return 6292; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 7 { return 6329; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 8 && block_state.r#attached == false { return 6330; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6343; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6286; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6297; }
        if block_state.r#attached == false && block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 6317; }
        if block_state.r#attached == true && block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 6311; }
        if block_state.r#rotation == 9 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6301; }
        if block_state.r#rotation == 2 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6287; }
        if block_state.r#attached == false && block_state.r#rotation == 2 && block_state.r#waterlogged == false { return 6319; }
        if block_state.r#rotation == 13 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6308; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 4 && block_state.r#attached == true { return 6290; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 0 { return 6314; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6300; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 10 { return 6334; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 10 { return 6335; }
        if block_state.r#attached == false && block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 6344; }
        if block_state.r#rotation == 12 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6307; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 3 { return 6320; }
        if block_state.r#attached == true && block_state.r#rotation == 8 && block_state.r#waterlogged == false { return 6299; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 6323; }
        if block_state.r#attached == false && block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 6318; }
        if block_state.r#attached == false && block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 6345; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 6293; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 7 { return 6328; }
        if block_state.r#attached == false && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 6315; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6339; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6309; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6324; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6285; }
        if block_state.r#rotation == 3 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6321; }
        if block_state.r#attached == false && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 6332; }
        if block_state.r#rotation == 7 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6296; }
        if block_state.r#attached == true && block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 6302; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6337 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 6305 {
            return Some(WarpedHangingSign {
                r#rotation: 11,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6325 {
            return Some(WarpedHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6310 {
            return Some(WarpedHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6313 {
            return Some(WarpedHangingSign {
                r#rotation: 15,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6340 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 6312 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 6282 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 6326 {
            return Some(WarpedHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6283 {
            return Some(WarpedHangingSign {
                r#waterlogged: false,
                r#rotation: 0,
                r#attached: true,
            });
        }
        if state_id == 6322 {
            return Some(WarpedHangingSign {
                r#rotation: 4,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6291 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 6342 {
            return Some(WarpedHangingSign {
                r#rotation: 14,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6298 {
            return Some(WarpedHangingSign {
                r#rotation: 8,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6341 {
            return Some(WarpedHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6316 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 6306 {
            return Some(WarpedHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6294 {
            return Some(WarpedHangingSign {
                r#rotation: 6,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6331 {
            return Some(WarpedHangingSign {
                r#rotation: 8,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6303 {
            return Some(WarpedHangingSign {
                r#rotation: 10,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6338 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 6327 {
            return Some(WarpedHangingSign {
                r#rotation: 6,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6289 {
            return Some(WarpedHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6304 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 11,
            });
        }
        if state_id == 6295 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 6336 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 6284 {
            return Some(WarpedHangingSign {
                r#rotation: 1,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6333 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 9,
            });
        }
        if state_id == 6288 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 3,
            });
        }
        if state_id == 6292 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#rotation: 5,
                r#attached: true,
            });
        }
        if state_id == 6329 {
            return Some(WarpedHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 7,
            });
        }
        if state_id == 6330 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#rotation: 8,
                r#attached: false,
            });
        }
        if state_id == 6343 {
            return Some(WarpedHangingSign {
                r#rotation: 14,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6286 {
            return Some(WarpedHangingSign {
                r#rotation: 2,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6297 {
            return Some(WarpedHangingSign {
                r#rotation: 7,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6317 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 6311 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 6301 {
            return Some(WarpedHangingSign {
                r#rotation: 9,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6287 {
            return Some(WarpedHangingSign {
                r#rotation: 2,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6319 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 6308 {
            return Some(WarpedHangingSign {
                r#rotation: 13,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6290 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#rotation: 4,
                r#attached: true,
            });
        }
        if state_id == 6314 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 0,
            });
        }
        if state_id == 6300 {
            return Some(WarpedHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6334 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 10,
            });
        }
        if state_id == 6335 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 10,
            });
        }
        if state_id == 6344 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 6307 {
            return Some(WarpedHangingSign {
                r#rotation: 12,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6320 {
            return Some(WarpedHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 3,
            });
        }
        if state_id == 6299 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 6323 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 6318 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 6345 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 6293 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 6328 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 6315 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 6339 {
            return Some(WarpedHangingSign {
                r#rotation: 12,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6309 {
            return Some(WarpedHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6324 {
            return Some(WarpedHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6285 {
            return Some(WarpedHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6321 {
            return Some(WarpedHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6332 {
            return Some(WarpedHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6296 {
            return Some(WarpedHangingSign {
                r#rotation: 7,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6302 {
            return Some(WarpedHangingSign {
                r#attached: true,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

