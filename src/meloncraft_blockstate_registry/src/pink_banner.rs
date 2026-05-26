use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkBanner {
    pub rotation: i32,
}

impl BlockState for PinkBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 0 {
            return 12821;
        }
        if self.r#rotation == 1 {
            return 12822;
        }
        if self.r#rotation == 8 {
            return 12829;
        }
        if self.r#rotation == 13 {
            return 12834;
        }
        if self.r#rotation == 6 {
            return 12827;
        }
        if self.r#rotation == 9 {
            return 12830;
        }
        if self.r#rotation == 11 {
            return 12832;
        }
        if self.r#rotation == 5 {
            return 12826;
        }
        if self.r#rotation == 15 {
            return 12836;
        }
        if self.r#rotation == 7 {
            return 12828;
        }
        if self.r#rotation == 12 {
            return 12833;
        }
        if self.r#rotation == 4 {
            return 12825;
        }
        if self.r#rotation == 10 {
            return 12831;
        }
        if self.r#rotation == 2 {
            return 12823;
        }
        if self.r#rotation == 14 {
            return 12835;
        }
        if self.r#rotation == 3 {
            return 12824;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12821 {
            return Some(PinkBanner { r#rotation: 0 });
        }
        if state_id == 12822 {
            return Some(PinkBanner { r#rotation: 1 });
        }
        if state_id == 12829 {
            return Some(PinkBanner { r#rotation: 8 });
        }
        if state_id == 12834 {
            return Some(PinkBanner { r#rotation: 13 });
        }
        if state_id == 12827 {
            return Some(PinkBanner { r#rotation: 6 });
        }
        if state_id == 12830 {
            return Some(PinkBanner { r#rotation: 9 });
        }
        if state_id == 12832 {
            return Some(PinkBanner { r#rotation: 11 });
        }
        if state_id == 12826 {
            return Some(PinkBanner { r#rotation: 5 });
        }
        if state_id == 12836 {
            return Some(PinkBanner { r#rotation: 15 });
        }
        if state_id == 12828 {
            return Some(PinkBanner { r#rotation: 7 });
        }
        if state_id == 12833 {
            return Some(PinkBanner { r#rotation: 12 });
        }
        if state_id == 12825 {
            return Some(PinkBanner { r#rotation: 4 });
        }
        if state_id == 12831 {
            return Some(PinkBanner { r#rotation: 10 });
        }
        if state_id == 12823 {
            return Some(PinkBanner { r#rotation: 2 });
        }
        if state_id == 12835 {
            return Some(PinkBanner { r#rotation: 14 });
        }
        if state_id == 12824 {
            return Some(PinkBanner { r#rotation: 3 });
        }
        return None;
    }
}
