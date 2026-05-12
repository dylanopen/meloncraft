use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub power: i32,
}


impl BlockState for Target {
    fn to_id(self) -> i32 {
        if block_state.r#power == 7 { return 21557; }
        if block_state.r#power == 13 { return 21563; }
        if block_state.r#power == 14 { return 21564; }
        if block_state.r#power == 6 { return 21556; }
        if block_state.r#power == 5 { return 21555; }
        if block_state.r#power == 2 { return 21552; }
        if block_state.r#power == 8 { return 21558; }
        if block_state.r#power == 9 { return 21559; }
        if block_state.r#power == 12 { return 21562; }
        if block_state.r#power == 3 { return 21553; }
        if block_state.r#power == 1 { return 21551; }
        if block_state.r#power == 10 { return 21560; }
        if block_state.r#power == 15 { return 21565; }
        if block_state.r#power == 0 { return 21550; }
        if block_state.r#power == 11 { return 21561; }
        if block_state.r#power == 4 { return 21554; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21557 {
            return Some(Target {
                r#power: 7,
            });
        }
        if state_id == 21563 {
            return Some(Target {
                r#power: 13,
            });
        }
        if state_id == 21564 {
            return Some(Target {
                r#power: 14,
            });
        }
        if state_id == 21556 {
            return Some(Target {
                r#power: 6,
            });
        }
        if state_id == 21555 {
            return Some(Target {
                r#power: 5,
            });
        }
        if state_id == 21552 {
            return Some(Target {
                r#power: 2,
            });
        }
        if state_id == 21558 {
            return Some(Target {
                r#power: 8,
            });
        }
        if state_id == 21559 {
            return Some(Target {
                r#power: 9,
            });
        }
        if state_id == 21562 {
            return Some(Target {
                r#power: 12,
            });
        }
        if state_id == 21553 {
            return Some(Target {
                r#power: 3,
            });
        }
        if state_id == 21551 {
            return Some(Target {
                r#power: 1,
            });
        }
        if state_id == 21560 {
            return Some(Target {
                r#power: 10,
            });
        }
        if state_id == 21565 {
            return Some(Target {
                r#power: 15,
            });
        }
        if state_id == 21550 {
            return Some(Target {
                r#power: 0,
            });
        }
        if state_id == 21561 {
            return Some(Target {
                r#power: 11,
            });
        }
        if state_id == 21554 {
            return Some(Target {
                r#power: 4,
            });
        }
        return None;
    }
}

