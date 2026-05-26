use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kelp {
    pub age: i32,
}

impl BlockState for Kelp {
    fn to_id(&self) -> i32 {
        if self.r#age == 20 {
            return 14880;
        }
        if self.r#age == 13 {
            return 14873;
        }
        if self.r#age == 15 {
            return 14875;
        }
        if self.r#age == 21 {
            return 14881;
        }
        if self.r#age == 8 {
            return 14868;
        }
        if self.r#age == 24 {
            return 14884;
        }
        if self.r#age == 2 {
            return 14862;
        }
        if self.r#age == 22 {
            return 14882;
        }
        if self.r#age == 12 {
            return 14872;
        }
        if self.r#age == 23 {
            return 14883;
        }
        if self.r#age == 7 {
            return 14867;
        }
        if self.r#age == 1 {
            return 14861;
        }
        if self.r#age == 10 {
            return 14870;
        }
        if self.r#age == 11 {
            return 14871;
        }
        if self.r#age == 3 {
            return 14863;
        }
        if self.r#age == 14 {
            return 14874;
        }
        if self.r#age == 16 {
            return 14876;
        }
        if self.r#age == 0 {
            return 14860;
        }
        if self.r#age == 18 {
            return 14878;
        }
        if self.r#age == 4 {
            return 14864;
        }
        if self.r#age == 6 {
            return 14866;
        }
        if self.r#age == 25 {
            return 14885;
        }
        if self.r#age == 9 {
            return 14869;
        }
        if self.r#age == 17 {
            return 14877;
        }
        if self.r#age == 19 {
            return 14879;
        }
        if self.r#age == 5 {
            return 14865;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14880 {
            return Some(Kelp { r#age: 20 });
        }
        if state_id == 14873 {
            return Some(Kelp { r#age: 13 });
        }
        if state_id == 14875 {
            return Some(Kelp { r#age: 15 });
        }
        if state_id == 14881 {
            return Some(Kelp { r#age: 21 });
        }
        if state_id == 14868 {
            return Some(Kelp { r#age: 8 });
        }
        if state_id == 14884 {
            return Some(Kelp { r#age: 24 });
        }
        if state_id == 14862 {
            return Some(Kelp { r#age: 2 });
        }
        if state_id == 14882 {
            return Some(Kelp { r#age: 22 });
        }
        if state_id == 14872 {
            return Some(Kelp { r#age: 12 });
        }
        if state_id == 14883 {
            return Some(Kelp { r#age: 23 });
        }
        if state_id == 14867 {
            return Some(Kelp { r#age: 7 });
        }
        if state_id == 14861 {
            return Some(Kelp { r#age: 1 });
        }
        if state_id == 14870 {
            return Some(Kelp { r#age: 10 });
        }
        if state_id == 14871 {
            return Some(Kelp { r#age: 11 });
        }
        if state_id == 14863 {
            return Some(Kelp { r#age: 3 });
        }
        if state_id == 14874 {
            return Some(Kelp { r#age: 14 });
        }
        if state_id == 14876 {
            return Some(Kelp { r#age: 16 });
        }
        if state_id == 14860 {
            return Some(Kelp { r#age: 0 });
        }
        if state_id == 14878 {
            return Some(Kelp { r#age: 18 });
        }
        if state_id == 14864 {
            return Some(Kelp { r#age: 4 });
        }
        if state_id == 14866 {
            return Some(Kelp { r#age: 6 });
        }
        if state_id == 14885 {
            return Some(Kelp { r#age: 25 });
        }
        if state_id == 14869 {
            return Some(Kelp { r#age: 9 });
        }
        if state_id == 14877 {
            return Some(Kelp { r#age: 17 });
        }
        if state_id == 14879 {
            return Some(Kelp { r#age: 19 });
        }
        if state_id == 14865 {
            return Some(Kelp { r#age: 5 });
        }
        return None;
    }
}
