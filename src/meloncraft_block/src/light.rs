use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Light {
    pub waterlogged: bool,
    pub level: i32,
}


impl BlockState for Light {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#level == 2 { return 12338; }
        if self.r#level == 15 && self.r#waterlogged == true { return 12363; }
        if self.r#waterlogged == true && self.r#level == 5 { return 12343; }
        if self.r#level == 8 && self.r#waterlogged == false { return 12350; }
        if self.r#waterlogged == true && self.r#level == 4 { return 12341; }
        if self.r#level == 7 && self.r#waterlogged == true { return 12347; }
        if self.r#waterlogged == false && self.r#level == 4 { return 12342; }
        if self.r#waterlogged == false && self.r#level == 13 { return 12360; }
        if self.r#level == 2 && self.r#waterlogged == true { return 12337; }
        if self.r#waterlogged == true && self.r#level == 9 { return 12351; }
        if self.r#waterlogged == true && self.r#level == 1 { return 12335; }
        if self.r#level == 6 && self.r#waterlogged == false { return 12346; }
        if self.r#waterlogged == true && self.r#level == 0 { return 12333; }
        if self.r#level == 6 && self.r#waterlogged == true { return 12345; }
        if self.r#waterlogged == true && self.r#level == 12 { return 12357; }
        if self.r#level == 1 && self.r#waterlogged == false { return 12336; }
        if self.r#waterlogged == true && self.r#level == 8 { return 12349; }
        if self.r#level == 9 && self.r#waterlogged == false { return 12352; }
        if self.r#waterlogged == false && self.r#level == 5 { return 12344; }
        if self.r#level == 14 && self.r#waterlogged == true { return 12361; }
        if self.r#level == 0 && self.r#waterlogged == false { return 12334; }
        if self.r#level == 13 && self.r#waterlogged == true { return 12359; }
        if self.r#waterlogged == false && self.r#level == 10 { return 12354; }
        if self.r#waterlogged == false && self.r#level == 7 { return 12348; }
        if self.r#waterlogged == false && self.r#level == 14 { return 12362; }
        if self.r#level == 3 && self.r#waterlogged == false { return 12340; }
        if self.r#level == 12 && self.r#waterlogged == false { return 12358; }
        if self.r#level == 3 && self.r#waterlogged == true { return 12339; }
        if self.r#level == 10 && self.r#waterlogged == true { return 12353; }
        if self.r#level == 11 && self.r#waterlogged == true { return 12355; }
        if self.r#level == 15 && self.r#waterlogged == false { return 12364; }
        if self.r#waterlogged == false && self.r#level == 11 { return 12356; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12338 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 2,
            });
        }
        if state_id == 12363 {
            return Some(Light {
                r#level: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 12343 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 5,
            });
        }
        if state_id == 12350 {
            return Some(Light {
                r#level: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 12341 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 4,
            });
        }
        if state_id == 12347 {
            return Some(Light {
                r#level: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 12342 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 4,
            });
        }
        if state_id == 12360 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 13,
            });
        }
        if state_id == 12337 {
            return Some(Light {
                r#level: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 12351 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 9,
            });
        }
        if state_id == 12335 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 1,
            });
        }
        if state_id == 12346 {
            return Some(Light {
                r#level: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 12333 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 0,
            });
        }
        if state_id == 12345 {
            return Some(Light {
                r#level: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 12357 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 12,
            });
        }
        if state_id == 12336 {
            return Some(Light {
                r#level: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 12349 {
            return Some(Light {
                r#waterlogged: true,
                r#level: 8,
            });
        }
        if state_id == 12352 {
            return Some(Light {
                r#level: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 12344 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 5,
            });
        }
        if state_id == 12361 {
            return Some(Light {
                r#level: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 12334 {
            return Some(Light {
                r#level: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 12359 {
            return Some(Light {
                r#level: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 12354 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 10,
            });
        }
        if state_id == 12348 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 7,
            });
        }
        if state_id == 12362 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 14,
            });
        }
        if state_id == 12340 {
            return Some(Light {
                r#level: 3,
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
                r#level: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 12353 {
            return Some(Light {
                r#level: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 12355 {
            return Some(Light {
                r#level: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 12364 {
            return Some(Light {
                r#level: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 12356 {
            return Some(Light {
                r#waterlogged: false,
                r#level: 11,
            });
        }
        return None;
    }
}

