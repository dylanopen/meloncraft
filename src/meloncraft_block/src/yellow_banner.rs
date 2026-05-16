use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowBanner {
    pub rotation: i32,
}


impl BlockState for YellowBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 4 { return 12793; }
        if self.r#rotation == 15 { return 12804; }
        if self.r#rotation == 3 { return 12792; }
        if self.r#rotation == 8 { return 12797; }
        if self.r#rotation == 1 { return 12790; }
        if self.r#rotation == 2 { return 12791; }
        if self.r#rotation == 9 { return 12798; }
        if self.r#rotation == 11 { return 12800; }
        if self.r#rotation == 6 { return 12795; }
        if self.r#rotation == 14 { return 12803; }
        if self.r#rotation == 7 { return 12796; }
        if self.r#rotation == 5 { return 12794; }
        if self.r#rotation == 13 { return 12802; }
        if self.r#rotation == 0 { return 12789; }
        if self.r#rotation == 12 { return 12801; }
        if self.r#rotation == 10 { return 12799; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12793 {
            return Some(YellowBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12804 {
            return Some(YellowBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12792 {
            return Some(YellowBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12797 {
            return Some(YellowBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12790 {
            return Some(YellowBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12791 {
            return Some(YellowBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12798 {
            return Some(YellowBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12800 {
            return Some(YellowBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12795 {
            return Some(YellowBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12803 {
            return Some(YellowBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12796 {
            return Some(YellowBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12794 {
            return Some(YellowBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12802 {
            return Some(YellowBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12789 {
            return Some(YellowBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12801 {
            return Some(YellowBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12799 {
            return Some(YellowBanner {
                r#rotation: 10,
            });
        }
        return None;
    }
}

