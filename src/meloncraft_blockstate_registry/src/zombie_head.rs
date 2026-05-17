use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZombieHead {
    pub powered: bool,
    pub rotation: i32,
}


impl BlockState for ZombieHead {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 14 && self.r#powered == true { return 10807; }
        if self.r#rotation == 6 && self.r#powered == false { return 10815; }
        if self.r#rotation == 1 && self.r#powered == false { return 10810; }
        if self.r#rotation == 10 && self.r#powered == true { return 10803; }
        if self.r#rotation == 11 && self.r#powered == true { return 10804; }
        if self.r#powered == true && self.r#rotation == 13 { return 10806; }
        if self.r#powered == true && self.r#rotation == 7 { return 10800; }
        if self.r#powered == true && self.r#rotation == 9 { return 10802; }
        if self.r#powered == false && self.r#rotation == 12 { return 10821; }
        if self.r#rotation == 5 && self.r#powered == false { return 10814; }
        if self.r#rotation == 4 && self.r#powered == false { return 10813; }
        if self.r#powered == true && self.r#rotation == 6 { return 10799; }
        if self.r#powered == false && self.r#rotation == 8 { return 10817; }
        if self.r#rotation == 8 && self.r#powered == true { return 10801; }
        if self.r#powered == true && self.r#rotation == 2 { return 10795; }
        if self.r#powered == true && self.r#rotation == 0 { return 10793; }
        if self.r#powered == false && self.r#rotation == 7 { return 10816; }
        if self.r#powered == false && self.r#rotation == 10 { return 10819; }
        if self.r#powered == false && self.r#rotation == 14 { return 10823; }
        if self.r#rotation == 11 && self.r#powered == false { return 10820; }
        if self.r#powered == false && self.r#rotation == 15 { return 10824; }
        if self.r#rotation == 9 && self.r#powered == false { return 10818; }
        if self.r#powered == true && self.r#rotation == 15 { return 10808; }
        if self.r#rotation == 3 && self.r#powered == true { return 10796; }
        if self.r#rotation == 4 && self.r#powered == true { return 10797; }
        if self.r#powered == false && self.r#rotation == 3 { return 10812; }
        if self.r#powered == true && self.r#rotation == 1 { return 10794; }
        if self.r#powered == true && self.r#rotation == 5 { return 10798; }
        if self.r#powered == false && self.r#rotation == 13 { return 10822; }
        if self.r#rotation == 2 && self.r#powered == false { return 10811; }
        if self.r#rotation == 0 && self.r#powered == false { return 10809; }
        if self.r#rotation == 12 && self.r#powered == true { return 10805; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10807 {
            return Some(ZombieHead {
                r#rotation: 14,
                r#powered: true,
            });
        }
        if state_id == 10815 {
            return Some(ZombieHead {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10810 {
            return Some(ZombieHead {
                r#rotation: 1,
                r#powered: false,
            });
        }
        if state_id == 10803 {
            return Some(ZombieHead {
                r#rotation: 10,
                r#powered: true,
            });
        }
        if state_id == 10804 {
            return Some(ZombieHead {
                r#rotation: 11,
                r#powered: true,
            });
        }
        if state_id == 10806 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 13,
            });
        }
        if state_id == 10800 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 7,
            });
        }
        if state_id == 10802 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 9,
            });
        }
        if state_id == 10821 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 12,
            });
        }
        if state_id == 10814 {
            return Some(ZombieHead {
                r#rotation: 5,
                r#powered: false,
            });
        }
        if state_id == 10813 {
            return Some(ZombieHead {
                r#rotation: 4,
                r#powered: false,
            });
        }
        if state_id == 10799 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 6,
            });
        }
        if state_id == 10817 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 8,
            });
        }
        if state_id == 10801 {
            return Some(ZombieHead {
                r#rotation: 8,
                r#powered: true,
            });
        }
        if state_id == 10795 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 2,
            });
        }
        if state_id == 10793 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10816 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 7,
            });
        }
        if state_id == 10819 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10823 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 14,
            });
        }
        if state_id == 10820 {
            return Some(ZombieHead {
                r#rotation: 11,
                r#powered: false,
            });
        }
        if state_id == 10824 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 15,
            });
        }
        if state_id == 10818 {
            return Some(ZombieHead {
                r#rotation: 9,
                r#powered: false,
            });
        }
        if state_id == 10808 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 15,
            });
        }
        if state_id == 10796 {
            return Some(ZombieHead {
                r#rotation: 3,
                r#powered: true,
            });
        }
        if state_id == 10797 {
            return Some(ZombieHead {
                r#rotation: 4,
                r#powered: true,
            });
        }
        if state_id == 10812 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10794 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10798 {
            return Some(ZombieHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10822 {
            return Some(ZombieHead {
                r#powered: false,
                r#rotation: 13,
            });
        }
        if state_id == 10811 {
            return Some(ZombieHead {
                r#rotation: 2,
                r#powered: false,
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
        return None;
    }
}

