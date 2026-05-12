use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreeperHead {
    pub rotation: i32,
    pub powered: bool,
}


impl BlockState for CreeperHead {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#rotation == 14 { return 10903; }
        if self.r#powered == true && self.r#rotation == 3 { return 10876; }
        if self.r#powered == true && self.r#rotation == 5 { return 10878; }
        if self.r#powered == true && self.r#rotation == 11 { return 10884; }
        if self.r#rotation == 11 && self.r#powered == false { return 10900; }
        if self.r#powered == false && self.r#rotation == 10 { return 10899; }
        if self.r#powered == true && self.r#rotation == 4 { return 10877; }
        if self.r#rotation == 8 && self.r#powered == true { return 10881; }
        if self.r#rotation == 15 && self.r#powered == false { return 10904; }
        if self.r#powered == false && self.r#rotation == 12 { return 10901; }
        if self.r#powered == true && self.r#rotation == 0 { return 10873; }
        if self.r#rotation == 13 && self.r#powered == true { return 10886; }
        if self.r#powered == true && self.r#rotation == 15 { return 10888; }
        if self.r#rotation == 0 && self.r#powered == false { return 10889; }
        if self.r#powered == true && self.r#rotation == 7 { return 10880; }
        if self.r#powered == true && self.r#rotation == 9 { return 10882; }
        if self.r#powered == true && self.r#rotation == 14 { return 10887; }
        if self.r#powered == false && self.r#rotation == 6 { return 10895; }
        if self.r#powered == false && self.r#rotation == 7 { return 10896; }
        if self.r#rotation == 2 && self.r#powered == false { return 10891; }
        if self.r#rotation == 2 && self.r#powered == true { return 10875; }
        if self.r#powered == true && self.r#rotation == 10 { return 10883; }
        if self.r#powered == true && self.r#rotation == 12 { return 10885; }
        if self.r#rotation == 5 && self.r#powered == false { return 10894; }
        if self.r#rotation == 1 && self.r#powered == false { return 10890; }
        if self.r#rotation == 9 && self.r#powered == false { return 10898; }
        if self.r#powered == true && self.r#rotation == 1 { return 10874; }
        if self.r#powered == false && self.r#rotation == 3 { return 10892; }
        if self.r#rotation == 6 && self.r#powered == true { return 10879; }
        if self.r#rotation == 13 && self.r#powered == false { return 10902; }
        if self.r#rotation == 4 && self.r#powered == false { return 10893; }
        if self.r#powered == false && self.r#rotation == 8 { return 10897; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10903 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 14,
            });
        }
        if state_id == 10876 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 3,
            });
        }
        if state_id == 10878 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10884 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 11,
            });
        }
        if state_id == 10900 {
            return Some(CreeperHead {
                r#rotation: 11,
                r#powered: false,
            });
        }
        if state_id == 10899 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10877 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 4,
            });
        }
        if state_id == 10881 {
            return Some(CreeperHead {
                r#rotation: 8,
                r#powered: true,
            });
        }
        if state_id == 10904 {
            return Some(CreeperHead {
                r#rotation: 15,
                r#powered: false,
            });
        }
        if state_id == 10901 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 12,
            });
        }
        if state_id == 10873 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10886 {
            return Some(CreeperHead {
                r#rotation: 13,
                r#powered: true,
            });
        }
        if state_id == 10888 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 15,
            });
        }
        if state_id == 10889 {
            return Some(CreeperHead {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10880 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 7,
            });
        }
        if state_id == 10882 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 9,
            });
        }
        if state_id == 10887 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10895 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 6,
            });
        }
        if state_id == 10896 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 7,
            });
        }
        if state_id == 10891 {
            return Some(CreeperHead {
                r#rotation: 2,
                r#powered: false,
            });
        }
        if state_id == 10875 {
            return Some(CreeperHead {
                r#rotation: 2,
                r#powered: true,
            });
        }
        if state_id == 10883 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 10,
            });
        }
        if state_id == 10885 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 12,
            });
        }
        if state_id == 10894 {
            return Some(CreeperHead {
                r#rotation: 5,
                r#powered: false,
            });
        }
        if state_id == 10890 {
            return Some(CreeperHead {
                r#rotation: 1,
                r#powered: false,
            });
        }
        if state_id == 10898 {
            return Some(CreeperHead {
                r#rotation: 9,
                r#powered: false,
            });
        }
        if state_id == 10874 {
            return Some(CreeperHead {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10892 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10879 {
            return Some(CreeperHead {
                r#rotation: 6,
                r#powered: true,
            });
        }
        if state_id == 10902 {
            return Some(CreeperHead {
                r#rotation: 13,
                r#powered: false,
            });
        }
        if state_id == 10893 {
            return Some(CreeperHead {
                r#rotation: 4,
                r#powered: false,
            });
        }
        if state_id == 10897 {
            return Some(CreeperHead {
                r#powered: false,
                r#rotation: 8,
            });
        }
        return None;
    }
}

