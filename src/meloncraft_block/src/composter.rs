use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composter {
    pub level: i32,
}


impl BlockState for Composter {
    fn to_id(&self) -> i32 {
        if self.r#level == 6 { return 21547; }
        if self.r#level == 8 { return 21549; }
        if self.r#level == 4 { return 21545; }
        if self.r#level == 7 { return 21548; }
        if self.r#level == 2 { return 21543; }
        if self.r#level == 0 { return 21541; }
        if self.r#level == 1 { return 21542; }
        if self.r#level == 3 { return 21544; }
        if self.r#level == 5 { return 21546; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21547 {
            return Some(Composter {
                r#level: 6,
            });
        }
        if state_id == 21549 {
            return Some(Composter {
                r#level: 8,
            });
        }
        if state_id == 21545 {
            return Some(Composter {
                r#level: 4,
            });
        }
        if state_id == 21548 {
            return Some(Composter {
                r#level: 7,
            });
        }
        if state_id == 21543 {
            return Some(Composter {
                r#level: 2,
            });
        }
        if state_id == 21541 {
            return Some(Composter {
                r#level: 0,
            });
        }
        if state_id == 21542 {
            return Some(Composter {
                r#level: 1,
            });
        }
        if state_id == 21544 {
            return Some(Composter {
                r#level: 3,
            });
        }
        if state_id == 21546 {
            return Some(Composter {
                r#level: 5,
            });
        }
        return None;
    }
}

