use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayBanner {
    pub rotation: i32,
}


impl BlockState for LightGrayBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 14 { return 12867; }
        if self.r#rotation == 6 { return 12859; }
        if self.r#rotation == 7 { return 12860; }
        if self.r#rotation == 3 { return 12856; }
        if self.r#rotation == 2 { return 12855; }
        if self.r#rotation == 12 { return 12865; }
        if self.r#rotation == 15 { return 12868; }
        if self.r#rotation == 9 { return 12862; }
        if self.r#rotation == 0 { return 12853; }
        if self.r#rotation == 4 { return 12857; }
        if self.r#rotation == 13 { return 12866; }
        if self.r#rotation == 1 { return 12854; }
        if self.r#rotation == 8 { return 12861; }
        if self.r#rotation == 11 { return 12864; }
        if self.r#rotation == 5 { return 12858; }
        if self.r#rotation == 10 { return 12863; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12867 {
            return Some(LightGrayBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12859 {
            return Some(LightGrayBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12860 {
            return Some(LightGrayBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12856 {
            return Some(LightGrayBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12855 {
            return Some(LightGrayBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12865 {
            return Some(LightGrayBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12868 {
            return Some(LightGrayBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12862 {
            return Some(LightGrayBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12853 {
            return Some(LightGrayBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12857 {
            return Some(LightGrayBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12866 {
            return Some(LightGrayBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12854 {
            return Some(LightGrayBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12861 {
            return Some(LightGrayBanner {
                r#rotation: 8,
            });
        }
        if state_id == 12864 {
            return Some(LightGrayBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12858 {
            return Some(LightGrayBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12863 {
            return Some(LightGrayBanner {
                r#rotation: 10,
            });
        }
        return None;
    }
}

