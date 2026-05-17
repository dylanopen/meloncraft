use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownBanner {
    pub rotation: i32,
}


impl BlockState for BrownBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 0 { return 12917; }
        if self.r#rotation == 3 { return 12920; }
        if self.r#rotation == 1 { return 12918; }
        if self.r#rotation == 10 { return 12927; }
        if self.r#rotation == 11 { return 12928; }
        if self.r#rotation == 5 { return 12922; }
        if self.r#rotation == 7 { return 12924; }
        if self.r#rotation == 12 { return 12929; }
        if self.r#rotation == 6 { return 12923; }
        if self.r#rotation == 13 { return 12930; }
        if self.r#rotation == 4 { return 12921; }
        if self.r#rotation == 14 { return 12931; }
        if self.r#rotation == 15 { return 12932; }
        if self.r#rotation == 9 { return 12926; }
        if self.r#rotation == 2 { return 12919; }
        if self.r#rotation == 8 { return 12925; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12917 {
            return Some(BrownBanner {
                r#rotation: 0,
            });
        }
        if state_id == 12920 {
            return Some(BrownBanner {
                r#rotation: 3,
            });
        }
        if state_id == 12918 {
            return Some(BrownBanner {
                r#rotation: 1,
            });
        }
        if state_id == 12927 {
            return Some(BrownBanner {
                r#rotation: 10,
            });
        }
        if state_id == 12928 {
            return Some(BrownBanner {
                r#rotation: 11,
            });
        }
        if state_id == 12922 {
            return Some(BrownBanner {
                r#rotation: 5,
            });
        }
        if state_id == 12924 {
            return Some(BrownBanner {
                r#rotation: 7,
            });
        }
        if state_id == 12929 {
            return Some(BrownBanner {
                r#rotation: 12,
            });
        }
        if state_id == 12923 {
            return Some(BrownBanner {
                r#rotation: 6,
            });
        }
        if state_id == 12930 {
            return Some(BrownBanner {
                r#rotation: 13,
            });
        }
        if state_id == 12921 {
            return Some(BrownBanner {
                r#rotation: 4,
            });
        }
        if state_id == 12931 {
            return Some(BrownBanner {
                r#rotation: 14,
            });
        }
        if state_id == 12932 {
            return Some(BrownBanner {
                r#rotation: 15,
            });
        }
        if state_id == 12926 {
            return Some(BrownBanner {
                r#rotation: 9,
            });
        }
        if state_id == 12919 {
            return Some(BrownBanner {
                r#rotation: 2,
            });
        }
        if state_id == 12925 {
            return Some(BrownBanner {
                r#rotation: 8,
            });
        }
        return None;
    }
}

