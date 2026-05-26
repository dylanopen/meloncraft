use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeBanner {
    pub rotation: i32,
}

impl BlockState for LimeBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 11 {
            return 12816;
        }
        if self.r#rotation == 2 {
            return 12807;
        }
        if self.r#rotation == 12 {
            return 12817;
        }
        if self.r#rotation == 5 {
            return 12810;
        }
        if self.r#rotation == 0 {
            return 12805;
        }
        if self.r#rotation == 1 {
            return 12806;
        }
        if self.r#rotation == 8 {
            return 12813;
        }
        if self.r#rotation == 10 {
            return 12815;
        }
        if self.r#rotation == 9 {
            return 12814;
        }
        if self.r#rotation == 7 {
            return 12812;
        }
        if self.r#rotation == 15 {
            return 12820;
        }
        if self.r#rotation == 6 {
            return 12811;
        }
        if self.r#rotation == 3 {
            return 12808;
        }
        if self.r#rotation == 14 {
            return 12819;
        }
        if self.r#rotation == 13 {
            return 12818;
        }
        if self.r#rotation == 4 {
            return 12809;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12816 {
            return Some(LimeBanner { r#rotation: 11 });
        }
        if state_id == 12807 {
            return Some(LimeBanner { r#rotation: 2 });
        }
        if state_id == 12817 {
            return Some(LimeBanner { r#rotation: 12 });
        }
        if state_id == 12810 {
            return Some(LimeBanner { r#rotation: 5 });
        }
        if state_id == 12805 {
            return Some(LimeBanner { r#rotation: 0 });
        }
        if state_id == 12806 {
            return Some(LimeBanner { r#rotation: 1 });
        }
        if state_id == 12813 {
            return Some(LimeBanner { r#rotation: 8 });
        }
        if state_id == 12815 {
            return Some(LimeBanner { r#rotation: 10 });
        }
        if state_id == 12814 {
            return Some(LimeBanner { r#rotation: 9 });
        }
        if state_id == 12812 {
            return Some(LimeBanner { r#rotation: 7 });
        }
        if state_id == 12820 {
            return Some(LimeBanner { r#rotation: 15 });
        }
        if state_id == 12811 {
            return Some(LimeBanner { r#rotation: 6 });
        }
        if state_id == 12808 {
            return Some(LimeBanner { r#rotation: 3 });
        }
        if state_id == 12819 {
            return Some(LimeBanner { r#rotation: 14 });
        }
        if state_id == 12818 {
            return Some(LimeBanner { r#rotation: 13 });
        }
        if state_id == 12809 {
            return Some(LimeBanner { r#rotation: 4 });
        }
        return None;
    }
}
