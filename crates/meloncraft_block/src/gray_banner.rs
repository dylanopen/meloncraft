use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayBanner {
    pub rotation: i32,
}


impl BlockState for GrayBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 15 { return 12852; }
        if self.r#rotation == 3 { return 12840; }
        if self.r#rotation == 11 { return 12848; }
        if self.r#rotation == 12 { return 12849; }
        if self.r#rotation == 14 { return 12851; }
        if self.r#rotation == 5 { return 12842; }
        if self.r#rotation == 0 { return 12837; }
        if self.r#rotation == 4 { return 12841; }
        if self.r#rotation == 7 { return 12844; }
        if self.r#rotation == 8 { return 12845; }
        if self.r#rotation == 9 { return 12846; }
        if self.r#rotation == 1 { return 12838; }
        if self.r#rotation == 10 { return 12847; }
        if self.r#rotation == 2 { return 12839; }
        if self.r#rotation == 6 { return 12843; }
        if self.r#rotation == 13 { return 12850; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12852 {
            return Some(GrayBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12840 {
            return Some(GrayBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12848 {
            return Some(GrayBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12849 {
            return Some(GrayBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12851 {
            return Some(GrayBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12842 {
            return Some(GrayBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12837 {
            return Some(GrayBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12841 {
            return Some(GrayBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12844 {
            return Some(GrayBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12845 {
            return Some(GrayBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12846 {
            return Some(GrayBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12838 {
            return Some(GrayBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12847 {
            return Some(GrayBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12839 {
            return Some(GrayBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12843 {
            return Some(GrayBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12850 {
            return Some(GrayBanner {
                r#rotation: 13,
            });
        }
        return None;
    }
}

