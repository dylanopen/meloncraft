use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueBanner {
    pub rotation: i32,
}


impl BlockState for LightBlueBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 11 { return 12784; }
        if self.r#rotation == 13 { return 12786; }
        if self.r#rotation == 2 { return 12775; }
        if self.r#rotation == 14 { return 12787; }
        if self.r#rotation == 15 { return 12788; }
        if self.r#rotation == 1 { return 12774; }
        if self.r#rotation == 10 { return 12783; }
        if self.r#rotation == 0 { return 12773; }
        if self.r#rotation == 12 { return 12785; }
        if self.r#rotation == 5 { return 12778; }
        if self.r#rotation == 8 { return 12781; }
        if self.r#rotation == 7 { return 12780; }
        if self.r#rotation == 4 { return 12777; }
        if self.r#rotation == 3 { return 12776; }
        if self.r#rotation == 6 { return 12779; }
        if self.r#rotation == 9 { return 12782; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12784 {
            return Some(LightBlueBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12786 {
            return Some(LightBlueBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12775 {
            return Some(LightBlueBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12787 {
            return Some(LightBlueBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12788 {
            return Some(LightBlueBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12774 {
            return Some(LightBlueBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12783 {
            return Some(LightBlueBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12773 {
            return Some(LightBlueBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12785 {
            return Some(LightBlueBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12778 {
            return Some(LightBlueBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12781 {
            return Some(LightBlueBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12780 {
            return Some(LightBlueBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12777 {
            return Some(LightBlueBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12776 {
            return Some(LightBlueBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12779 {
            return Some(LightBlueBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12782 {
            return Some(LightBlueBanner {
                r#rotation: 9,
            });
        }
        return None;
    }
}

