use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Light {
    pub level: i32,
    pub waterlogged: bool,
}


impl BlockState for Light {
    fn to_id(self) -> i32 {
        if block_state.r#level == 2 && block_state.r#waterlogged == false { return 12338; }
        if block_state.r#waterlogged == true && block_state.r#level == 9 { return 12351; }
        if block_state.r#level == 9 && block_state.r#waterlogged == false { return 12352; }
        if block_state.r#level == 12 && block_state.r#waterlogged == false { return 12358; }
        if block_state.r#waterlogged == true && block_state.r#level == 3 { return 12339; }
        if block_state.r#waterlogged == false && block_state.r#level == 1 { return 12336; }
        if block_state.r#waterlogged == true && block_state.r#level == 4 { return 12341; }
        if block_state.r#waterlogged == true && block_state.r#level == 1 { return 12335; }
        if block_state.r#waterlogged == true && block_state.r#level == 7 { return 12347; }
        if block_state.r#level == 13 && block_state.r#waterlogged == false { return 12360; }
        if block_state.r#level == 3 && block_state.r#waterlogged == false { return 12340; }
        if block_state.r#level == 0 && block_state.r#waterlogged == false { return 12334; }
        if block_state.r#waterlogged == false && block_state.r#level == 8 { return 12350; }
        if block_state.r#waterlogged == true && block_state.r#level == 12 { return 12357; }
        if block_state.r#level == 13 && block_state.r#waterlogged == true { return 12359; }
        if block_state.r#level == 6 && block_state.r#waterlogged == true { return 12345; }
        if block_state.r#waterlogged == false && block_state.r#level == 6 { return 12346; }
        if block_state.r#level == 8 && block_state.r#waterlogged == true { return 12349; }
        if block_state.r#level == 11 && block_state.r#waterlogged == true { return 12355; }
        if block_state.r#level == 7 && block_state.r#waterlogged == false { return 12348; }
        if block_state.r#level == 15 && block_state.r#waterlogged == false { return 12364; }
        if block_state.r#level == 5 && block_state.r#waterlogged == true { return 12343; }
        if block_state.r#level == 2 && block_state.r#waterlogged == true { return 12337; }
        if block_state.r#level == 10 && block_state.r#waterlogged == true { return 12353; }
        if block_state.r#level == 14 && block_state.r#waterlogged == false { return 12362; }
        if block_state.r#level == 0 && block_state.r#waterlogged == true { return 12333; }
        if block_state.r#level == 10 && block_state.r#waterlogged == false { return 12354; }
        if block_state.r#level == 11 && block_state.r#waterlogged == false { return 12356; }
        if block_state.r#level == 14 && block_state.r#waterlogged == true { return 12361; }
        if block_state.r#waterlogged == false && block_state.r#level == 4 { return 12342; }
        if block_state.r#level == 5 && block_state.r#waterlogged == false { return 12344; }
        if block_state.r#waterlogged == true && block_state.r#level == 15 { return 12363; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12338 {
            return Some(Light {
                r#level: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 12351 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 9,
            });
        }
        if state_id == 12352 {
            return Some(Light {
                r#level: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 12358 {
            return Some(Light {
                r#level: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 12339 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 3,
            });
        }
        if state_id == 12336 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 1,
            });
        }
        if state_id == 12341 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 4,
            });
        }
        if state_id == 12335 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 1,
            });
        }
        if state_id == 12347 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 7,
            });
        }
        if state_id == 12360 {
            return Some(Light {
                r#level: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 12340 {
            return Some(Light {
                r#level: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 12334 {
            return Some(Light {
                r#level: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 12350 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 8,
            });
        }
        if state_id == 12357 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 12,
            });
        }
        if state_id == 12359 {
            return Some(Light {
                r#level: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 12345 {
            return Some(Light {
                r#level: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 12346 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 6,
            });
        }
        if state_id == 12349 {
            return Some(Light {
                r#level: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 12355 {
            return Some(Light {
                r#level: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 12348 {
            return Some(Light {
                r#level: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 12364 {
            return Some(Light {
                r#level: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 12343 {
            return Some(Light {
                r#level: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 12337 {
            return Some(Light {
                r#level: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 12353 {
            return Some(Light {
                r#level: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 12362 {
            return Some(Light {
                r#level: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 12333 {
            return Some(Light {
                r#level: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 12354 {
            return Some(Light {
                r#level: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 12356 {
            return Some(Light {
                r#level: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 12361 {
            return Some(Light {
                r#level: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 12342 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 4,
            });
        }
        if state_id == 12344 {
            return Some(Light {
                r#level: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 12363 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 15,
            });
        }
        return None;
    }
}

