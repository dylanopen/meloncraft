use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwistingVines {
    pub age: i32,
}


impl BlockState for TwistingVines {
    fn to_id(&self) -> i32 {
        if self.r#age == 15 { return 20817; }
        if self.r#age == 17 { return 20819; }
        if self.r#age == 20 { return 20822; }
        if self.r#age == 21 { return 20823; }
        if self.r#age == 22 { return 20824; }
        if self.r#age == 2 { return 20804; }
        if self.r#age == 25 { return 20827; }
        if self.r#age == 7 { return 20809; }
        if self.r#age == 16 { return 20818; }
        if self.r#age == 9 { return 20811; }
        if self.r#age == 24 { return 20826; }
        if self.r#age == 8 { return 20810; }
        if self.r#age == 13 { return 20815; }
        if self.r#age == 23 { return 20825; }
        if self.r#age == 19 { return 20821; }
        if self.r#age == 18 { return 20820; }
        if self.r#age == 0 { return 20802; }
        if self.r#age == 4 { return 20806; }
        if self.r#age == 6 { return 20808; }
        if self.r#age == 11 { return 20813; }
        if self.r#age == 5 { return 20807; }
        if self.r#age == 3 { return 20805; }
        if self.r#age == 1 { return 20803; }
        if self.r#age == 12 { return 20814; }
        if self.r#age == 10 { return 20812; }
        if self.r#age == 14 { return 20816; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20817 {
            return Some(TwistingVines {
                r#age: 15,
            });
        }
        if state_id == 20819 {
            return Some(TwistingVines {
                r#age: 17,
            });
        }
        if state_id == 20822 {
            return Some(TwistingVines {
                r#age: 20,
            });
        }
        if state_id == 20823 {
            return Some(TwistingVines {
                r#age: 21,
            });
        }
        if state_id == 20824 {
            return Some(TwistingVines {
                r#age: 22,
            });
        }
        if state_id == 20804 {
            return Some(TwistingVines {
                r#age: 2,
            });
        }
        if state_id == 20827 {
            return Some(TwistingVines {
                r#age: 25,
            });
        }
        if state_id == 20809 {
            return Some(TwistingVines {
                r#age: 7,
            });
        }
        if state_id == 20818 {
            return Some(TwistingVines {
                r#age: 16,
            });
        }
        if state_id == 20811 {
            return Some(TwistingVines {
                r#age: 9,
            });
        }
        if state_id == 20826 {
            return Some(TwistingVines {
                r#age: 24,
            });
        }
        if state_id == 20810 {
            return Some(TwistingVines {
                r#age: 8,
            });
        }
        if state_id == 20815 {
            return Some(TwistingVines {
                r#age: 13,
            });
        }
        if state_id == 20825 {
            return Some(TwistingVines {
                r#age: 23,
            });
        }
        if state_id == 20821 {
            return Some(TwistingVines {
                r#age: 19,
            });
        }
        if state_id == 20820 {
            return Some(TwistingVines {
                r#age: 18,
            });
        }
        if state_id == 20802 {
            return Some(TwistingVines {
                r#age: 0,
            });
        }
        if state_id == 20806 {
            return Some(TwistingVines {
                r#age: 4,
            });
        }
        if state_id == 20808 {
            return Some(TwistingVines {
                r#age: 6,
            });
        }
        if state_id == 20813 {
            return Some(TwistingVines {
                r#age: 11,
            });
        }
        if state_id == 20807 {
            return Some(TwistingVines {
                r#age: 5,
            });
        }
        if state_id == 20805 {
            return Some(TwistingVines {
                r#age: 3,
            });
        }
        if state_id == 20803 {
            return Some(TwistingVines {
                r#age: 1,
            });
        }
        if state_id == 20814 {
            return Some(TwistingVines {
                r#age: 12,
            });
        }
        if state_id == 20812 {
            return Some(TwistingVines {
                r#age: 10,
            });
        }
        if state_id == 20816 {
            return Some(TwistingVines {
                r#age: 14,
            });
        }
        return None;
    }
}

