use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZombieHead {
    pub powered: bool,
    pub rotation: i32,
}


impl BlockState for ZombieHead {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#rotation == 10 { return 10819; }
        if block_state.r#rotation == 3 && block_state.r#powered == false { return 10812; }
        if block_state.r#powered == true && block_state.r#rotation == 2 { return 10795; }
        if block_state.r#rotation == 7 && block_state.r#powered == false { return 10816; }
        if block_state.r#rotation == 11 && block_state.r#powered == true { return 10804; }
        if block_state.r#powered == false && block_state.r#rotation == 9 { return 10818; }
        if block_state.r#powered == false && block_state.r#rotation == 15 { return 10824; }
        if block_state.r#powered == false && block_state.r#rotation == 1 { return 10810; }
        if block_state.r#rotation == 8 && block_state.r#powered == false { return 10817; }
        if block_state.r#rotation == 3 && block_state.r#powered == true { return 10796; }
        if block_state.r#powered == false && block_state.r#rotation == 14 { return 10823; }
        if block_state.r#powered == true && block_state.r#rotation == 10 { return 10803; }
        if block_state.r#powered == true && block_state.r#rotation == 13 { return 10806; }
        if block_state.r#powered == false && block_state.r#rotation == 2 { return 10811; }
        if block_state.r#rotation == 5 && block_state.r#powered == false { return 10814; }
        if block_state.r#powered == false && block_state.r#rotation == 12 { return 10821; }
        if block_state.r#rotation == 15 && block_state.r#powered == true { return 10808; }
        if block_state.r#powered == true && block_state.r#rotation == 8 { return 10801; }
        if block_state.r#powered == true && block_state.r#rotation == 9 { return 10802; }
        if block_state.r#powered == true && block_state.r#rotation == 1 { return 10794; }
        if block_state.r#powered == true && block_state.r#rotation == 4 { return 10797; }
        if block_state.r#powered == true && block_state.r#rotation == 0 { return 10793; }
        if block_state.r#powered == true && block_state.r#rotation == 5 { return 10798; }
        if block_state.r#powered == true && block_state.r#rotation == 7 { return 10800; }
        if block_state.r#rotation == 0 && block_state.r#powered == false { return 10809; }
        if block_state.r#rotation == 12 && block_state.r#powered == true { return 10805; }
        if block_state.r#rotation == 6 && block_state.r#powered == false { return 10815; }
        if block_state.r#powered == true && block_state.r#rotation == 6 { return 10799; }
        if block_state.r#powered == true && block_state.r#rotation == 14 { return 10807; }
        if block_state.r#rotation == 13 && block_state.r#powered == false { return 10822; }
        if block_state.r#powered == false && block_state.r#rotation == 4 { return 10813; }
        if block_state.r#powered == false && block_state.r#rotation == 11 { return 10820; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10819 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10812 {
            return Some(ZombieHead {
                r#rotation: 3,
                r#powered: false,
            });
        }
        if state_id == 10795 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 2,
            });
        }
        if state_id == 10816 {
            return Some(ZombieHead {
                r#rotation: 7,
                r#powered: false,
            });
        }
        if state_id == 10804 {
            return Some(ZombieHead {
                r#rotation: 11,
                r#powered: true,
            });
        }
        if state_id == 10818 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 9,
            });
        }
        if state_id == 10824 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 15,
            });
        }
        if state_id == 10810 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 1,
            });
        }
        if state_id == 10817 {
            return Some(ZombieHead {
                r#rotation: 8,
                r#powered: false,
            });
        }
        if state_id == 10796 {
            return Some(ZombieHead {
                r#rotation: 3,
                r#powered: true,
            });
        }
        if state_id == 10823 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 14,
            });
        }
        if state_id == 10803 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 10,
            });
        }
        if state_id == 10806 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 13,
            });
        }
        if state_id == 10811 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 2,
            });
        }
        if state_id == 10814 {
            return Some(ZombieHead {
                r#rotation: 5,
                r#powered: false,
            });
        }
        if state_id == 10821 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 12,
            });
        }
        if state_id == 10808 {
            return Some(ZombieHead {
                r#rotation: 15,
                r#powered: true,
            });
        }
        if state_id == 10801 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 8,
            });
        }
        if state_id == 10802 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 9,
            });
        }
        if state_id == 10794 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10797 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 4,
            });
        }
        if state_id == 10793 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10798 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10800 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 7,
            });
        }
        if state_id == 10809 {
            return Some(ZombieHead {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10805 {
            return Some(ZombieHead {
                r#rotation: 12,
                r#powered: true,
            });
        }
        if state_id == 10815 {
            return Some(ZombieHead {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10799 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 6,
            });
        }
        if state_id == 10807 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10822 {
            return Some(ZombieHead {
                r#rotation: 13,
                r#powered: false,
            });
        }
        if state_id == 10813 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 4,
            });
        }
        if state_id == 10820 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 11,
            });
        }
        return None;
    }
}

