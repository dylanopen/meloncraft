use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakSign {
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for DarkOakSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 5354; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 5355; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5350; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 5343; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 11 { return 5349; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5336; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5338; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == true { return 5348; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 8 { return 5342; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == false { return 5331; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 5326; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 5357; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 5332; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 5337; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5347; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 6 { return 5339; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5344; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 7 { return 5340; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 12 { return 5351; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 { return 5341; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5352; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5327; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5329; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 5334; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == false { return 5335; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 5346; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false { return 5333; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 9 { return 5345; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 5330; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 5356; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5353; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 1 { return 5328; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5354 {
            return Some(DarkOakSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5355 {
            return Some(DarkOakSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5350 {
            return Some(DarkOakSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5343 {
            return Some(DarkOakSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5349 {
            return Some(DarkOakSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5336 {
            return Some(DarkOakSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5338 {
            return Some(DarkOakSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5348 {
            return Some(DarkOakSign {
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 5342 {
            return Some(DarkOakSign {
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 5331 {
            return Some(DarkOakSign {
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5326 {
            return Some(DarkOakSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5357 {
            return Some(DarkOakSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5332 {
            return Some(DarkOakSign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5337 {
            return Some(DarkOakSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5347 {
            return Some(DarkOakSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5339 {
            return Some(DarkOakSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5344 {
            return Some(DarkOakSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5340 {
            return Some(DarkOakSign {
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 5351 {
            return Some(DarkOakSign {
                r#waterlogged: false,
                r#rotation: 12,
            });
        }
        if state_id == 5341 {
            return Some(DarkOakSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5352 {
            return Some(DarkOakSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5327 {
            return Some(DarkOakSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5329 {
            return Some(DarkOakSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5334 {
            return Some(DarkOakSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5335 {
            return Some(DarkOakSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5346 {
            return Some(DarkOakSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5333 {
            return Some(DarkOakSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5345 {
            return Some(DarkOakSign {
                r#waterlogged: false,
                r#rotation: 9,
            });
        }
        if state_id == 5330 {
            return Some(DarkOakSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5356 {
            return Some(DarkOakSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5353 {
            return Some(DarkOakSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5328 {
            return Some(DarkOakSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        return None;
    }
}

