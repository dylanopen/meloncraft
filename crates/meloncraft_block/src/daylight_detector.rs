use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaylightDetector {
    pub inverted: bool,
    pub power: i32,
}


impl BlockState for DaylightDetector {
    fn to_id(&self) -> i32 {
        if self.r#inverted == true && self.r#power == 14 { return 11091; }
        if self.r#inverted == true && self.r#power == 0 { return 11077; }
        if self.r#power == 8 && self.r#inverted == true { return 11085; }
        if self.r#inverted == true && self.r#power == 3 { return 11080; }
        if self.r#inverted == true && self.r#power == 2 { return 11079; }
        if self.r#power == 9 && self.r#inverted == false { return 11102; }
        if self.r#inverted == false && self.r#power == 4 { return 11097; }
        if self.r#inverted == false && self.r#power == 8 { return 11101; }
        if self.r#inverted == true && self.r#power == 6 { return 11083; }
        if self.r#inverted == false && self.r#power == 7 { return 11100; }
        if self.r#inverted == false && self.r#power == 0 { return 11093; }
        if self.r#inverted == false && self.r#power == 6 { return 11099; }
        if self.r#inverted == false && self.r#power == 12 { return 11105; }
        if self.r#inverted == false && self.r#power == 15 { return 11108; }
        if self.r#inverted == true && self.r#power == 13 { return 11090; }
        if self.r#power == 11 && self.r#inverted == true { return 11088; }
        if self.r#inverted == true && self.r#power == 15 { return 11092; }
        if self.r#inverted == true && self.r#power == 10 { return 11087; }
        if self.r#inverted == true && self.r#power == 4 { return 11081; }
        if self.r#power == 12 && self.r#inverted == true { return 11089; }
        if self.r#inverted == false && self.r#power == 5 { return 11098; }
        if self.r#inverted == false && self.r#power == 10 { return 11103; }
        if self.r#inverted == false && self.r#power == 14 { return 11107; }
        if self.r#inverted == true && self.r#power == 9 { return 11086; }
        if self.r#inverted == true && self.r#power == 1 { return 11078; }
        if self.r#power == 7 && self.r#inverted == true { return 11084; }
        if self.r#inverted == true && self.r#power == 5 { return 11082; }
        if self.r#power == 13 && self.r#inverted == false { return 11106; }
        if self.r#power == 3 && self.r#inverted == false { return 11096; }
        if self.r#power == 11 && self.r#inverted == false { return 11104; }
        if self.r#power == 2 && self.r#inverted == false { return 11095; }
        if self.r#power == 1 && self.r#inverted == false { return 11094; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11091 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 14,
            });
        }
        if state_id == 11077 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 0,
            });
        }
        if state_id == 11085 {
            return Some(DaylightDetector {
                r#power: 8,
                r#inverted: true,
            });
        }
        if state_id == 11080 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 3,
            });
        }
        if state_id == 11079 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 2,
            });
        }
        if state_id == 11102 {
            return Some(DaylightDetector {
                r#power: 9,
                r#inverted: false,
            });
        }
        if state_id == 11097 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 4,
            });
        }
        if state_id == 11101 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 8,
            });
        }
        if state_id == 11083 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 6,
            });
        }
        if state_id == 11100 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 7,
            });
        }
        if state_id == 11093 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 0,
            });
        }
        if state_id == 11099 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 6,
            });
        }
        if state_id == 11105 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 12,
            });
        }
        if state_id == 11108 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 15,
            });
        }
        if state_id == 11090 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 13,
            });
        }
        if state_id == 11088 {
            return Some(DaylightDetector {
                r#power: 11,
                r#inverted: true,
            });
        }
        if state_id == 11092 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 15,
            });
        }
        if state_id == 11087 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 10,
            });
        }
        if state_id == 11081 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 4,
            });
        }
        if state_id == 11089 {
            return Some(DaylightDetector {
                r#power: 12,
                r#inverted: true,
            });
        }
        if state_id == 11098 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 5,
            });
        }
        if state_id == 11103 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 10,
            });
        }
        if state_id == 11107 {
            return Some(DaylightDetector {
                r#inverted: false,
                r#power: 14,
            });
        }
        if state_id == 11086 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 9,
            });
        }
        if state_id == 11078 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 1,
            });
        }
        if state_id == 11084 {
            return Some(DaylightDetector {
                r#power: 7,
                r#inverted: true,
            });
        }
        if state_id == 11082 {
            return Some(DaylightDetector {
                r#inverted: true,
                r#power: 5,
            });
        }
        if state_id == 11106 {
            return Some(DaylightDetector {
                r#power: 13,
                r#inverted: false,
            });
        }
        if state_id == 11096 {
            return Some(DaylightDetector {
                r#power: 3,
                r#inverted: false,
            });
        }
        if state_id == 11104 {
            return Some(DaylightDetector {
                r#power: 11,
                r#inverted: false,
            });
        }
        if state_id == 11095 {
            return Some(DaylightDetector {
                r#power: 2,
                r#inverted: false,
            });
        }
        if state_id == 11094 {
            return Some(DaylightDetector {
                r#power: 1,
                r#inverted: false,
            });
        }
        return None;
    }
}

