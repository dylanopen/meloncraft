use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bamboo {
    pub r#leaves: Leaves,
    pub age: i32,
    pub stage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leaves {
    None,
    Small,
    Large,
}

impl BlockState for Bamboo {
    fn to_id(&self) -> i32 {
        if self.r#stage == 1 && self.r#age == 1 && self.r#leaves == Leaves::Large {
            return 15088;
        }
        if self.r#stage == 1 && self.r#age == 0 && self.r#leaves == Leaves::Small {
            return 15080;
        }
        if self.r#age == 1 && self.r#leaves == Leaves::None && self.r#stage == 1 {
            return 15084;
        }
        if self.r#stage == 0 && self.r#age == 0 && self.r#leaves == Leaves::Small {
            return 15079;
        }
        if self.r#leaves == Leaves::None && self.r#stage == 0 && self.r#age == 1 {
            return 15083;
        }
        if self.r#age == 1 && self.r#leaves == Leaves::Small && self.r#stage == 1 {
            return 15086;
        }
        if self.r#stage == 0 && self.r#leaves == Leaves::Large && self.r#age == 1 {
            return 15087;
        }
        if self.r#age == 1 && self.r#leaves == Leaves::Small && self.r#stage == 0 {
            return 15085;
        }
        if self.r#leaves == Leaves::Large && self.r#stage == 1 && self.r#age == 0 {
            return 15082;
        }
        if self.r#stage == 0 && self.r#leaves == Leaves::None && self.r#age == 0 {
            return 15077;
        }
        if self.r#stage == 1 && self.r#age == 0 && self.r#leaves == Leaves::None {
            return 15078;
        }
        if self.r#leaves == Leaves::Large && self.r#stage == 0 && self.r#age == 0 {
            return 15081;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15088 {
            return Some(Bamboo {
                r#stage: 1,
                r#age: 1,
                r#leaves: Leaves::Large,
            });
        }
        if state_id == 15080 {
            return Some(Bamboo {
                r#stage: 1,
                r#age: 0,
                r#leaves: Leaves::Small,
            });
        }
        if state_id == 15084 {
            return Some(Bamboo {
                r#age: 1,
                r#leaves: Leaves::None,
                r#stage: 1,
            });
        }
        if state_id == 15079 {
            return Some(Bamboo {
                r#stage: 0,
                r#age: 0,
                r#leaves: Leaves::Small,
            });
        }
        if state_id == 15083 {
            return Some(Bamboo {
                r#leaves: Leaves::None,
                r#stage: 0,
                r#age: 1,
            });
        }
        if state_id == 15086 {
            return Some(Bamboo {
                r#age: 1,
                r#leaves: Leaves::Small,
                r#stage: 1,
            });
        }
        if state_id == 15087 {
            return Some(Bamboo {
                r#stage: 0,
                r#leaves: Leaves::Large,
                r#age: 1,
            });
        }
        if state_id == 15085 {
            return Some(Bamboo {
                r#age: 1,
                r#leaves: Leaves::Small,
                r#stage: 0,
            });
        }
        if state_id == 15082 {
            return Some(Bamboo {
                r#leaves: Leaves::Large,
                r#stage: 1,
                r#age: 0,
            });
        }
        if state_id == 15077 {
            return Some(Bamboo {
                r#stage: 0,
                r#leaves: Leaves::None,
                r#age: 0,
            });
        }
        if state_id == 15078 {
            return Some(Bamboo {
                r#stage: 1,
                r#age: 0,
                r#leaves: Leaves::None,
            });
        }
        if state_id == 15081 {
            return Some(Bamboo {
                r#leaves: Leaves::Large,
                r#stage: 0,
                r#age: 0,
            });
        }
        return None;
    }
}
