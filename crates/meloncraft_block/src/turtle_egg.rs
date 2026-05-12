use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurtleEgg {
    pub eggs: i32,
    pub hatch: i32,
}


impl BlockState for TurtleEgg {
    fn to_id(self) -> i32 {
        if block_state.r#eggs == 1 && block_state.r#hatch == 0 { return 14888; }
        if block_state.r#hatch == 1 && block_state.r#eggs == 2 { return 14892; }
        if block_state.r#hatch == 2 && block_state.r#eggs == 4 { return 14899; }
        if block_state.r#hatch == 2 && block_state.r#eggs == 1 { return 14890; }
        if block_state.r#eggs == 3 && block_state.r#hatch == 0 { return 14894; }
        if block_state.r#eggs == 3 && block_state.r#hatch == 2 { return 14896; }
        if block_state.r#hatch == 0 && block_state.r#eggs == 2 { return 14891; }
        if block_state.r#hatch == 1 && block_state.r#eggs == 1 { return 14889; }
        if block_state.r#eggs == 3 && block_state.r#hatch == 1 { return 14895; }
        if block_state.r#hatch == 0 && block_state.r#eggs == 4 { return 14897; }
        if block_state.r#hatch == 2 && block_state.r#eggs == 2 { return 14893; }
        if block_state.r#eggs == 4 && block_state.r#hatch == 1 { return 14898; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14888 {
            return Some(TurtleEgg {
                r#eggs: 1,
                r#hatch: 0,
            });
        }
        if state_id == 14892 {
            return Some(TurtleEgg {
                r#hatch: 1,
                r#eggs: 2,
            });
        }
        if state_id == 14899 {
            return Some(TurtleEgg {
                r#hatch: 2,
                r#eggs: 4,
            });
        }
        if state_id == 14890 {
            return Some(TurtleEgg {
                r#hatch: 2,
                r#eggs: 1,
            });
        }
        if state_id == 14894 {
            return Some(TurtleEgg {
                r#eggs: 3,
                r#hatch: 0,
            });
        }
        if state_id == 14896 {
            return Some(TurtleEgg {
                r#eggs: 3,
                r#hatch: 2,
            });
        }
        if state_id == 14891 {
            return Some(TurtleEgg {
                r#hatch: 0,
                r#eggs: 2,
            });
        }
        if state_id == 14889 {
            return Some(TurtleEgg {
                r#hatch: 1,
                r#eggs: 1,
            });
        }
        if state_id == 14895 {
            return Some(TurtleEgg {
                r#eggs: 3,
                r#hatch: 1,
            });
        }
        if state_id == 14897 {
            return Some(TurtleEgg {
                r#hatch: 0,
                r#eggs: 4,
            });
        }
        if state_id == 14893 {
            return Some(TurtleEgg {
                r#hatch: 2,
                r#eggs: 2,
            });
        }
        if state_id == 14898 {
            return Some(TurtleEgg {
                r#eggs: 4,
                r#hatch: 1,
            });
        }
        return None;
    }
}

