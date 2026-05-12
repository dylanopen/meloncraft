use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bamboo {
    pub stage: i32,
    pub r#leaves: Leaves,
    pub age: i32,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leaves {
    None,
    Small,
    Large,
}

impl BlockState for Bamboo {
    fn to_id(self) -> i32 {
        if block_state.r#leaves == Leaves::Small && block_state.r#stage == 0 && block_state.r#age == 1 { return 15085; }
        if block_state.r#stage == 1 && block_state.r#leaves == Leaves::Large && block_state.r#age == 0 { return 15082; }
        if block_state.r#leaves == Leaves::Large && block_state.r#stage == 0 && block_state.r#age == 1 { return 15087; }
        if block_state.r#age == 1 && block_state.r#leaves == Leaves::None && block_state.r#stage == 1 { return 15084; }
        if block_state.r#age == 0 && block_state.r#stage == 1 && block_state.r#leaves == Leaves::Small { return 15080; }
        if block_state.r#leaves == Leaves::Large && block_state.r#age == 1 && block_state.r#stage == 1 { return 15088; }
        if block_state.r#age == 0 && block_state.r#leaves == Leaves::None && block_state.r#stage == 0 { return 15077; }
        if block_state.r#age == 0 && block_state.r#leaves == Leaves::Large && block_state.r#stage == 0 { return 15081; }
        if block_state.r#stage == 1 && block_state.r#age == 1 && block_state.r#leaves == Leaves::Small { return 15086; }
        if block_state.r#stage == 1 && block_state.r#leaves == Leaves::None && block_state.r#age == 0 { return 15078; }
        if block_state.r#leaves == Leaves::None && block_state.r#stage == 0 && block_state.r#age == 1 { return 15083; }
        if block_state.r#age == 0 && block_state.r#stage == 0 && block_state.r#leaves == Leaves::Small { return 15079; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15085 {
            return Some(Bamboo {
                r#leaves: Leaves::Small,
                r#stage: 0,
                r#age: 1,
            });
        }
        if state_id == 15082 {
            return Some(Bamboo {
                r#stage: 1,
                r#leaves: Leaves::Large,
                r#age: 0,
            });
        }
        if state_id == 15087 {
            return Some(Bamboo {
                r#leaves: Leaves::Large,
                r#stage: 0,
                r#age: 1,
            });
        }
        if state_id == 15084 {
            return Some(Bamboo {
                r#age: 1,
                r#leaves: Leaves::None,
                r#stage: 1,
            });
        }
        if state_id == 15080 {
            return Some(Bamboo {
                r#age: 0,
                r#stage: 1,
                r#leaves: Leaves::Small,
            });
        }
        if state_id == 15088 {
            return Some(Bamboo {
                r#leaves: Leaves::Large,
                r#age: 1,
                r#stage: 1,
            });
        }
        if state_id == 15077 {
            return Some(Bamboo {
                r#age: 0,
                r#leaves: Leaves::None,
                r#stage: 0,
            });
        }
        if state_id == 15081 {
            return Some(Bamboo {
                r#age: 0,
                r#leaves: Leaves::Large,
                r#stage: 0,
            });
        }
        if state_id == 15086 {
            return Some(Bamboo {
                r#stage: 1,
                r#age: 1,
                r#leaves: Leaves::Small,
            });
        }
        if state_id == 15078 {
            return Some(Bamboo {
                r#stage: 1,
                r#leaves: Leaves::None,
                r#age: 0,
            });
        }
        if state_id == 15083 {
            return Some(Bamboo {
                r#leaves: Leaves::None,
                r#stage: 0,
                r#age: 1,
            });
        }
        if state_id == 15079 {
            return Some(Bamboo {
                r#age: 0,
                r#stage: 0,
                r#leaves: Leaves::Small,
            });
        }
        return None;
    }
}

