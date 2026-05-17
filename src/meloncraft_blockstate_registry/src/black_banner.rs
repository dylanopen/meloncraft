use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackBanner {
    pub rotation: i32,
}


impl BlockState for BlackBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 11 { return 12976; }
        if self.r#rotation == 2 { return 12967; }
        if self.r#rotation == 3 { return 12968; }
        if self.r#rotation == 7 { return 12972; }
        if self.r#rotation == 4 { return 12969; }
        if self.r#rotation == 10 { return 12975; }
        if self.r#rotation == 8 { return 12973; }
        if self.r#rotation == 1 { return 12966; }
        if self.r#rotation == 14 { return 12979; }
        if self.r#rotation == 6 { return 12971; }
        if self.r#rotation == 13 { return 12978; }
        if self.r#rotation == 15 { return 12980; }
        if self.r#rotation == 5 { return 12970; }
        if self.r#rotation == 12 { return 12977; }
        if self.r#rotation == 9 { return 12974; }
        if self.r#rotation == 0 { return 12965; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12976 {
            return Some(BlackBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12967 {
            return Some(BlackBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12968 {
            return Some(BlackBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12972 {
            return Some(BlackBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12969 {
            return Some(BlackBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12975 {
            return Some(BlackBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12973 {
            return Some(BlackBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12966 {
            return Some(BlackBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12979 {
            return Some(BlackBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12971 {
            return Some(BlackBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12978 {
            return Some(BlackBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12980 {
            return Some(BlackBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12970 {
            return Some(BlackBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12977 {
            return Some(BlackBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12974 {
            return Some(BlackBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12965 {
            return Some(BlackBanner {
                r#rotation: 0,
            });
        }
        return None;
    }
}

