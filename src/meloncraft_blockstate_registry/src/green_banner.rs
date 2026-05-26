use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenBanner {
    pub rotation: i32,
}

impl BlockState for GreenBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 6 {
            return 12939;
        }
        if self.r#rotation == 9 {
            return 12942;
        }
        if self.r#rotation == 5 {
            return 12938;
        }
        if self.r#rotation == 11 {
            return 12944;
        }
        if self.r#rotation == 2 {
            return 12935;
        }
        if self.r#rotation == 0 {
            return 12933;
        }
        if self.r#rotation == 3 {
            return 12936;
        }
        if self.r#rotation == 4 {
            return 12937;
        }
        if self.r#rotation == 14 {
            return 12947;
        }
        if self.r#rotation == 10 {
            return 12943;
        }
        if self.r#rotation == 12 {
            return 12945;
        }
        if self.r#rotation == 7 {
            return 12940;
        }
        if self.r#rotation == 13 {
            return 12946;
        }
        if self.r#rotation == 1 {
            return 12934;
        }
        if self.r#rotation == 15 {
            return 12948;
        }
        if self.r#rotation == 8 {
            return 12941;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12939 {
            return Some(GreenBanner { r#rotation: 6 });
        }
        if state_id == 12942 {
            return Some(GreenBanner { r#rotation: 9 });
        }
        if state_id == 12938 {
            return Some(GreenBanner { r#rotation: 5 });
        }
        if state_id == 12944 {
            return Some(GreenBanner { r#rotation: 11 });
        }
        if state_id == 12935 {
            return Some(GreenBanner { r#rotation: 2 });
        }
        if state_id == 12933 {
            return Some(GreenBanner { r#rotation: 0 });
        }
        if state_id == 12936 {
            return Some(GreenBanner { r#rotation: 3 });
        }
        if state_id == 12937 {
            return Some(GreenBanner { r#rotation: 4 });
        }
        if state_id == 12947 {
            return Some(GreenBanner { r#rotation: 14 });
        }
        if state_id == 12943 {
            return Some(GreenBanner { r#rotation: 10 });
        }
        if state_id == 12945 {
            return Some(GreenBanner { r#rotation: 12 });
        }
        if state_id == 12940 {
            return Some(GreenBanner { r#rotation: 7 });
        }
        if state_id == 12946 {
            return Some(GreenBanner { r#rotation: 13 });
        }
        if state_id == 12934 {
            return Some(GreenBanner { r#rotation: 1 });
        }
        if state_id == 12948 {
            return Some(GreenBanner { r#rotation: 15 });
        }
        if state_id == 12941 {
            return Some(GreenBanner { r#rotation: 8 });
        }
        return None;
    }
}
