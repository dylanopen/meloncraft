use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lava {
    pub level: i32,
}


impl BlockState for Lava {
    fn to_id(&self) -> i32 {
        if self.r#level == 15 { return 117; }
        if self.r#level == 12 { return 114; }
        if self.r#level == 1 { return 103; }
        if self.r#level == 4 { return 106; }
        if self.r#level == 11 { return 113; }
        if self.r#level == 7 { return 109; }
        if self.r#level == 6 { return 108; }
        if self.r#level == 0 { return 102; }
        if self.r#level == 14 { return 116; }
        if self.r#level == 2 { return 104; }
        if self.r#level == 9 { return 111; }
        if self.r#level == 13 { return 115; }
        if self.r#level == 5 { return 107; }
        if self.r#level == 8 { return 110; }
        if self.r#level == 3 { return 105; }
        if self.r#level == 10 { return 112; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 117 {
            return Some(Lava {
                r#level: 15,
            });
        }
        if state_id == 114 {
            return Some(Lava {
                r#level: 12,
            });
        }
        if state_id == 103 {
            return Some(Lava {
                r#level: 1,
            });
        }
        if state_id == 106 {
            return Some(Lava {
                r#level: 4,
            });
        }
        if state_id == 113 {
            return Some(Lava {
                r#level: 11,
            });
        }
        if state_id == 109 {
            return Some(Lava {
                r#level: 7,
            });
        }
        if state_id == 108 {
            return Some(Lava {
                r#level: 6,
            });
        }
        if state_id == 102 {
            return Some(Lava {
                r#level: 0,
            });
        }
        if state_id == 116 {
            return Some(Lava {
                r#level: 14,
            });
        }
        if state_id == 104 {
            return Some(Lava {
                r#level: 2,
            });
        }
        if state_id == 111 {
            return Some(Lava {
                r#level: 9,
            });
        }
        if state_id == 115 {
            return Some(Lava {
                r#level: 13,
            });
        }
        if state_id == 107 {
            return Some(Lava {
                r#level: 5,
            });
        }
        if state_id == 110 {
            return Some(Lava {
                r#level: 8,
            });
        }
        if state_id == 105 {
            return Some(Lava {
                r#level: 3,
            });
        }
        if state_id == 112 {
            return Some(Lava {
                r#level: 10,
            });
        }
        return None;
    }
}

