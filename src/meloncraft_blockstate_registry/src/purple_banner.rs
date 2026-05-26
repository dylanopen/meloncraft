use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleBanner {
    pub rotation: i32,
}

impl BlockState for PurpleBanner {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 0 {
            return 12885;
        }
        if self.r#rotation == 4 {
            return 12889;
        }
        if self.r#rotation == 7 {
            return 12892;
        }
        if self.r#rotation == 10 {
            return 12895;
        }
        if self.r#rotation == 6 {
            return 12891;
        }
        if self.r#rotation == 15 {
            return 12900;
        }
        if self.r#rotation == 11 {
            return 12896;
        }
        if self.r#rotation == 5 {
            return 12890;
        }
        if self.r#rotation == 1 {
            return 12886;
        }
        if self.r#rotation == 9 {
            return 12894;
        }
        if self.r#rotation == 13 {
            return 12898;
        }
        if self.r#rotation == 2 {
            return 12887;
        }
        if self.r#rotation == 3 {
            return 12888;
        }
        if self.r#rotation == 14 {
            return 12899;
        }
        if self.r#rotation == 12 {
            return 12897;
        }
        if self.r#rotation == 8 {
            return 12893;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12885 {
            return Some(PurpleBanner { r#rotation: 0 });
        }
        if state_id == 12889 {
            return Some(PurpleBanner { r#rotation: 4 });
        }
        if state_id == 12892 {
            return Some(PurpleBanner { r#rotation: 7 });
        }
        if state_id == 12895 {
            return Some(PurpleBanner { r#rotation: 10 });
        }
        if state_id == 12891 {
            return Some(PurpleBanner { r#rotation: 6 });
        }
        if state_id == 12900 {
            return Some(PurpleBanner { r#rotation: 15 });
        }
        if state_id == 12896 {
            return Some(PurpleBanner { r#rotation: 11 });
        }
        if state_id == 12890 {
            return Some(PurpleBanner { r#rotation: 5 });
        }
        if state_id == 12886 {
            return Some(PurpleBanner { r#rotation: 1 });
        }
        if state_id == 12894 {
            return Some(PurpleBanner { r#rotation: 9 });
        }
        if state_id == 12898 {
            return Some(PurpleBanner { r#rotation: 13 });
        }
        if state_id == 12887 {
            return Some(PurpleBanner { r#rotation: 2 });
        }
        if state_id == 12888 {
            return Some(PurpleBanner { r#rotation: 3 });
        }
        if state_id == 12899 {
            return Some(PurpleBanner { r#rotation: 14 });
        }
        if state_id == 12897 {
            return Some(PurpleBanner { r#rotation: 12 });
        }
        if state_id == 12893 {
            return Some(PurpleBanner { r#rotation: 8 });
        }
        return None;
    }
}
