use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreeperHead {
    pub rotation: i32,
    pub powered: bool,
}


impl BlockState for CreeperHead {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#rotation == 9 { return 10898; }
        if block_state.r#powered == false && block_state.r#rotation == 1 { return 10890; }
        if block_state.r#rotation == 1 && block_state.r#powered == true { return 10874; }
        if block_state.r#rotation == 12 && block_state.r#powered == false { return 10901; }
        if block_state.r#rotation == 13 && block_state.r#powered == false { return 10902; }
        if block_state.r#powered == true && block_state.r#rotation == 8 { return 10881; }
        if block_state.r#powered == false && block_state.r#rotation == 4 { return 10893; }
        if block_state.r#rotation == 11 && block_state.r#powered == false { return 10900; }
        if block_state.r#powered == true && block_state.r#rotation == 0 { return 10873; }
        if block_state.r#powered == false && block_state.r#rotation == 14 { return 10903; }
        if block_state.r#powered == false && block_state.r#rotation == 15 { return 10904; }
        if block_state.r#rotation == 6 && block_state.r#powered == true { return 10879; }
        if block_state.r#powered == true && block_state.r#rotation == 12 { return 10885; }
        if block_state.r#powered == true && block_state.r#rotation == 4 { return 10877; }
        if block_state.r#powered == false && block_state.r#rotation == 2 { return 10891; }
        if block_state.r#powered == false && block_state.r#rotation == 7 { return 10896; }
        if block_state.r#powered == false && block_state.r#rotation == 8 { return 10897; }
        if block_state.r#rotation == 13 && block_state.r#powered == true { return 10886; }
        if block_state.r#rotation == 10 && block_state.r#powered == false { return 10899; }
        if block_state.r#rotation == 14 && block_state.r#powered == true { return 10887; }
        if block_state.r#powered == true && block_state.r#rotation == 5 { return 10878; }
        if block_state.r#rotation == 10 && block_state.r#powered == true { return 10883; }
        if block_state.r#rotation == 0 && block_state.r#powered == false { return 10889; }
        if block_state.r#rotation == 15 && block_state.r#powered == true { return 10888; }
        if block_state.r#rotation == 2 && block_state.r#powered == true { return 10875; }
        if block_state.r#rotation == 3 && block_state.r#powered == true { return 10876; }
        if block_state.r#rotation == 9 && block_state.r#powered == true { return 10882; }
        if block_state.r#powered == false && block_state.r#rotation == 3 { return 10892; }
        if block_state.r#rotation == 6 && block_state.r#powered == false { return 10895; }
        if block_state.r#rotation == 7 && block_state.r#powered == true { return 10880; }
        if block_state.r#powered == false && block_state.r#rotation == 5 { return 10894; }
        if block_state.r#rotation == 11 && block_state.r#powered == true { return 10884; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10898 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 9,
            });
        }
        if state_id == 10890 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 1,
            });
        }
        if state_id == 10874 {
            return Some(CreeperHead {
                r#rotation: 1,
                r#powered: true,
            });
        }
        if state_id == 10901 {
            return Some(CreeperHead {
                r#rotation: 12,
                r#powered: false,
            });
        }
        if state_id == 10902 {
            return Some(CreeperHead {
                r#rotation: 13,
                r#powered: false,
            });
        }
        if state_id == 10881 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 8,
            });
        }
        if state_id == 10893 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 4,
            });
        }
        if state_id == 10900 {
            return Some(CreeperHead {
                r#rotation: 11,
                r#powered: false,
            });
        }
        if state_id == 10873 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10903 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 14,
            });
        }
        if state_id == 10904 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 15,
            });
        }
        if state_id == 10879 {
            return Some(CreeperHead {
                r#rotation: 6,
                r#powered: true,
            });
        }
        if state_id == 10885 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 12,
            });
        }
        if state_id == 10877 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 4,
            });
        }
        if state_id == 10891 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 2,
            });
        }
        if state_id == 10896 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 7,
            });
        }
        if state_id == 10897 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 8,
            });
        }
        if state_id == 10886 {
            return Some(CreeperHead {
                r#rotation: 13,
                r#powered: true,
            });
        }
        if state_id == 10899 {
            return Some(CreeperHead {
                r#rotation: 10,
                r#powered: false,
            });
        }
        if state_id == 10887 {
            return Some(CreeperHead {
                r#rotation: 14,
                r#powered: true,
            });
        }
        if state_id == 10878 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10883 {
            return Some(CreeperHead {
                r#rotation: 10,
                r#powered: true,
            });
        }
        if state_id == 10889 {
            return Some(CreeperHead {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10888 {
            return Some(CreeperHead {
                r#rotation: 15,
                r#powered: true,
            });
        }
        if state_id == 10875 {
            return Some(CreeperHead {
                r#rotation: 2,
                r#powered: true,
            });
        }
        if state_id == 10876 {
            return Some(CreeperHead {
                r#rotation: 3,
                r#powered: true,
            });
        }
        if state_id == 10882 {
            return Some(CreeperHead {
                r#rotation: 9,
                r#powered: true,
            });
        }
        if state_id == 10892 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10895 {
            return Some(CreeperHead {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10880 {
            return Some(CreeperHead {
                r#rotation: 7,
                r#powered: true,
            });
        }
        if state_id == 10894 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 5,
            });
        }
        if state_id == 10884 {
            return Some(CreeperHead {
                r#rotation: 11,
                r#powered: true,
            });
        }
        return None;
    }
}

