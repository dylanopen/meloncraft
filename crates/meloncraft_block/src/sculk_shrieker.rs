use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SculkShrieker {
    pub can_summon: bool,
    pub shrieking: bool,
    pub waterlogged: bool,
}


impl BlockState for SculkShrieker {
    fn to_id(self) -> i32 {
        if block_state.r#shrieking == false && block_state.r#waterlogged == true && block_state.r#can_summon == true { return 25101; }
        if block_state.r#waterlogged == true && block_state.r#shrieking == true && block_state.r#can_summon == false { return 25103; }
        if block_state.r#shrieking == false && block_state.r#waterlogged == false && block_state.r#can_summon == false { return 25106; }
        if block_state.r#waterlogged == true && block_state.r#shrieking == true && block_state.r#can_summon == true { return 25099; }
        if block_state.r#can_summon == true && block_state.r#waterlogged == false && block_state.r#shrieking == false { return 25102; }
        if block_state.r#shrieking == true && block_state.r#waterlogged == false && block_state.r#can_summon == false { return 25104; }
        if block_state.r#can_summon == true && block_state.r#waterlogged == false && block_state.r#shrieking == true { return 25100; }
        if block_state.r#shrieking == false && block_state.r#can_summon == false && block_state.r#waterlogged == true { return 25105; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25101 {
            return Some(SculkShrieker {
                r#shrieking: false,
                r#waterlogged: true,
                r#can_summon: true,
            });
        }
        if state_id == 25103 {
            return Some(SculkShrieker {
                r#waterlogged: true,
                r#shrieking: true,
                r#can_summon: false,
            });
        }
        if state_id == 25106 {
            return Some(SculkShrieker {
                r#shrieking: false,
                r#waterlogged: false,
                r#can_summon: false,
            });
        }
        if state_id == 25099 {
            return Some(SculkShrieker {
                r#waterlogged: true,
                r#shrieking: true,
                r#can_summon: true,
            });
        }
        if state_id == 25102 {
            return Some(SculkShrieker {
                r#can_summon: true,
                r#waterlogged: false,
                r#shrieking: false,
            });
        }
        if state_id == 25104 {
            return Some(SculkShrieker {
                r#shrieking: true,
                r#waterlogged: false,
                r#can_summon: false,
            });
        }
        if state_id == 25100 {
            return Some(SculkShrieker {
                r#can_summon: true,
                r#waterlogged: false,
                r#shrieking: true,
            });
        }
        if state_id == 25105 {
            return Some(SculkShrieker {
                r#shrieking: false,
                r#can_summon: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

