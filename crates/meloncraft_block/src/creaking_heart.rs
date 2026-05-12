use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreakingHeart {
    pub r#axis: Axis,
    pub r#creaking_heart_state: CreakingHeartState,
    pub natural: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreakingHeartState {
    Uprooted,
    Dormant,
    Awake,
}

impl BlockState for CreakingHeart {
    fn to_id(self) -> i32 {
        if block_state.r#creaking_heart_state == CreakingHeartState::Dormant && block_state.r#axis == Axis::Z && block_state.r#natural == false { return 3703; }
        if block_state.r#natural == true && block_state.r#axis == Axis::X && block_state.r#creaking_heart_state == CreakingHeartState::Dormant { return 3690; }
        if block_state.r#creaking_heart_state == CreakingHeartState::Uprooted && block_state.r#axis == Axis::X && block_state.r#natural == true { return 3688; }
        if block_state.r#axis == Axis::Y && block_state.r#creaking_heart_state == CreakingHeartState::Awake && block_state.r#natural == true { return 3698; }
        if block_state.r#creaking_heart_state == CreakingHeartState::Uprooted && block_state.r#natural == false && block_state.r#axis == Axis::Y { return 3695; }
        if block_state.r#axis == Axis::Y && block_state.r#creaking_heart_state == CreakingHeartState::Uprooted && block_state.r#natural == true { return 3694; }
        if block_state.r#natural == false && block_state.r#creaking_heart_state == CreakingHeartState::Dormant && block_state.r#axis == Axis::Y { return 3697; }
        if block_state.r#natural == true && block_state.r#creaking_heart_state == CreakingHeartState::Awake && block_state.r#axis == Axis::Z { return 3704; }
        if block_state.r#creaking_heart_state == CreakingHeartState::Awake && block_state.r#axis == Axis::Y && block_state.r#natural == false { return 3699; }
        if block_state.r#natural == true && block_state.r#axis == Axis::X && block_state.r#creaking_heart_state == CreakingHeartState::Awake { return 3692; }
        if block_state.r#axis == Axis::Z && block_state.r#creaking_heart_state == CreakingHeartState::Uprooted && block_state.r#natural == true { return 3700; }
        if block_state.r#natural == false && block_state.r#creaking_heart_state == CreakingHeartState::Uprooted && block_state.r#axis == Axis::X { return 3689; }
        if block_state.r#natural == true && block_state.r#axis == Axis::Y && block_state.r#creaking_heart_state == CreakingHeartState::Dormant { return 3696; }
        if block_state.r#natural == false && block_state.r#axis == Axis::Z && block_state.r#creaking_heart_state == CreakingHeartState::Awake { return 3705; }
        if block_state.r#natural == false && block_state.r#creaking_heart_state == CreakingHeartState::Uprooted && block_state.r#axis == Axis::Z { return 3701; }
        if block_state.r#natural == true && block_state.r#creaking_heart_state == CreakingHeartState::Dormant && block_state.r#axis == Axis::Z { return 3702; }
        if block_state.r#creaking_heart_state == CreakingHeartState::Awake && block_state.r#axis == Axis::X && block_state.r#natural == false { return 3693; }
        if block_state.r#creaking_heart_state == CreakingHeartState::Dormant && block_state.r#natural == false && block_state.r#axis == Axis::X { return 3691; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3703 {
            return Some(CreakingHeart {
                r#creaking_heart_state: CreakingHeartState::Dormant,
                r#axis: Axis::Z,
                r#natural: false,
            });
        }
        if state_id == 3690 {
            return Some(CreakingHeart {
                r#natural: true,
                r#axis: Axis::X,
                r#creaking_heart_state: CreakingHeartState::Dormant,
            });
        }
        if state_id == 3688 {
            return Some(CreakingHeart {
                r#creaking_heart_state: CreakingHeartState::Uprooted,
                r#axis: Axis::X,
                r#natural: true,
            });
        }
        if state_id == 3698 {
            return Some(CreakingHeart {
                r#axis: Axis::Y,
                r#creaking_heart_state: CreakingHeartState::Awake,
                r#natural: true,
            });
        }
        if state_id == 3695 {
            return Some(CreakingHeart {
                r#creaking_heart_state: CreakingHeartState::Uprooted,
                r#natural: false,
                r#axis: Axis::Y,
            });
        }
        if state_id == 3694 {
            return Some(CreakingHeart {
                r#axis: Axis::Y,
                r#creaking_heart_state: CreakingHeartState::Uprooted,
                r#natural: true,
            });
        }
        if state_id == 3697 {
            return Some(CreakingHeart {
                r#natural: false,
                r#creaking_heart_state: CreakingHeartState::Dormant,
                r#axis: Axis::Y,
            });
        }
        if state_id == 3704 {
            return Some(CreakingHeart {
                r#natural: true,
                r#creaking_heart_state: CreakingHeartState::Awake,
                r#axis: Axis::Z,
            });
        }
        if state_id == 3699 {
            return Some(CreakingHeart {
                r#creaking_heart_state: CreakingHeartState::Awake,
                r#axis: Axis::Y,
                r#natural: false,
            });
        }
        if state_id == 3692 {
            return Some(CreakingHeart {
                r#natural: true,
                r#axis: Axis::X,
                r#creaking_heart_state: CreakingHeartState::Awake,
            });
        }
        if state_id == 3700 {
            return Some(CreakingHeart {
                r#axis: Axis::Z,
                r#creaking_heart_state: CreakingHeartState::Uprooted,
                r#natural: true,
            });
        }
        if state_id == 3689 {
            return Some(CreakingHeart {
                r#natural: false,
                r#creaking_heart_state: CreakingHeartState::Uprooted,
                r#axis: Axis::X,
            });
        }
        if state_id == 3696 {
            return Some(CreakingHeart {
                r#natural: true,
                r#axis: Axis::Y,
                r#creaking_heart_state: CreakingHeartState::Dormant,
            });
        }
        if state_id == 3705 {
            return Some(CreakingHeart {
                r#natural: false,
                r#axis: Axis::Z,
                r#creaking_heart_state: CreakingHeartState::Awake,
            });
        }
        if state_id == 3701 {
            return Some(CreakingHeart {
                r#natural: false,
                r#creaking_heart_state: CreakingHeartState::Uprooted,
                r#axis: Axis::Z,
            });
        }
        if state_id == 3702 {
            return Some(CreakingHeart {
                r#natural: true,
                r#creaking_heart_state: CreakingHeartState::Dormant,
                r#axis: Axis::Z,
            });
        }
        if state_id == 3693 {
            return Some(CreakingHeart {
                r#creaking_heart_state: CreakingHeartState::Awake,
                r#axis: Axis::X,
                r#natural: false,
            });
        }
        if state_id == 3691 {
            return Some(CreakingHeart {
                r#creaking_heart_state: CreakingHeartState::Dormant,
                r#natural: false,
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

