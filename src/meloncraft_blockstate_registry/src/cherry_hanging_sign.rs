use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryHangingSign {
    pub attached: bool,
    pub rotation: i32,
    pub waterlogged: bool,
}

impl BlockState for CherryHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 13 {
            return 6021;
        }
        if self.r#rotation == 9 && self.r#waterlogged == false && self.r#attached == false {
            return 6013;
        }
        if self.r#rotation == 6 && self.r#attached == true && self.r#waterlogged == false {
            return 5975;
        }
        if self.r#rotation == 6 && self.r#waterlogged == true && self.r#attached == true {
            return 5974;
        }
        if self.r#attached == true && self.r#rotation == 4 && self.r#waterlogged == true {
            return 5970;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 4 {
            return 6003;
        }
        if self.r#rotation == 15 && self.r#waterlogged == false && self.r#attached == true {
            return 5993;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 14 {
            return 6022;
        }
        if self.r#rotation == 7 && self.r#attached == true && self.r#waterlogged == false {
            return 5977;
        }
        if self.r#rotation == 13 && self.r#attached == true && self.r#waterlogged == true {
            return 5988;
        }
        if self.r#waterlogged == false && self.r#rotation == 3 && self.r#attached == false {
            return 6001;
        }
        if self.r#rotation == 15 && self.r#waterlogged == true && self.r#attached == false {
            return 6024;
        }
        if self.r#rotation == 1 && self.r#waterlogged == true && self.r#attached == true {
            return 5964;
        }
        if self.r#attached == true && self.r#rotation == 12 && self.r#waterlogged == false {
            return 5987;
        }
        if self.r#attached == true && self.r#rotation == 13 && self.r#waterlogged == false {
            return 5989;
        }
        if self.r#rotation == 10 && self.r#waterlogged == true && self.r#attached == false {
            return 6014;
        }
        if self.r#rotation == 2 && self.r#attached == false && self.r#waterlogged == true {
            return 5998;
        }
        if self.r#attached == false && self.r#rotation == 15 && self.r#waterlogged == false {
            return 6025;
        }
        if self.r#rotation == 3 && self.r#waterlogged == true && self.r#attached == true {
            return 5968;
        }
        if self.r#rotation == 1 && self.r#waterlogged == true && self.r#attached == false {
            return 5996;
        }
        if self.r#attached == false && self.r#rotation == 4 && self.r#waterlogged == true {
            return 6002;
        }
        if self.r#waterlogged == true && self.r#rotation == 8 && self.r#attached == true {
            return 5978;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 6 {
            return 6007;
        }
        if self.r#rotation == 10 && self.r#waterlogged == false && self.r#attached == false {
            return 6015;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 7 {
            return 6009;
        }
        if self.r#attached == false && self.r#rotation == 11 && self.r#waterlogged == false {
            return 6017;
        }
        if self.r#rotation == 2 && self.r#waterlogged == false && self.r#attached == true {
            return 5967;
        }
        if self.r#waterlogged == true && self.r#rotation == 13 && self.r#attached == false {
            return 6020;
        }
        if self.r#rotation == 0 && self.r#waterlogged == false && self.r#attached == true {
            return 5963;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 2 {
            return 5999;
        }
        if self.r#attached == true && self.r#waterlogged == false && self.r#rotation == 14 {
            return 5991;
        }
        if self.r#rotation == 9 && self.r#waterlogged == true && self.r#attached == false {
            return 6012;
        }
        if self.r#attached == false && self.r#rotation == 12 && self.r#waterlogged == false {
            return 6019;
        }
        if self.r#attached == false && self.r#rotation == 3 && self.r#waterlogged == true {
            return 6000;
        }
        if self.r#attached == true && self.r#rotation == 9 && self.r#waterlogged == true {
            return 5980;
        }
        if self.r#attached == false && self.r#rotation == 0 && self.r#waterlogged == true {
            return 5994;
        }
        if self.r#attached == false && self.r#rotation == 5 && self.r#waterlogged == true {
            return 6004;
        }
        if self.r#waterlogged == false && self.r#rotation == 8 && self.r#attached == false {
            return 6011;
        }
        if self.r#attached == true && self.r#rotation == 10 && self.r#waterlogged == true {
            return 5982;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 15 {
            return 5992;
        }
        if self.r#rotation == 9 && self.r#waterlogged == false && self.r#attached == true {
            return 5981;
        }
        if self.r#rotation == 7 && self.r#attached == false && self.r#waterlogged == true {
            return 6008;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 8 {
            return 5979;
        }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 5 {
            return 6005;
        }
        if self.r#waterlogged == true && self.r#rotation == 6 && self.r#attached == false {
            return 6006;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 1 {
            return 5965;
        }
        if self.r#rotation == 10 && self.r#attached == true && self.r#waterlogged == false {
            return 5983;
        }
        if self.r#attached == true && self.r#rotation == 12 && self.r#waterlogged == true {
            return 5986;
        }
        if self.r#attached == true && self.r#rotation == 14 && self.r#waterlogged == true {
            return 5990;
        }
        if self.r#attached == true && self.r#waterlogged == false && self.r#rotation == 3 {
            return 5969;
        }
        if self.r#rotation == 11 && self.r#attached == true && self.r#waterlogged == false {
            return 5985;
        }
        if self.r#attached == false && self.r#rotation == 12 && self.r#waterlogged == true {
            return 6018;
        }
        if self.r#attached == true && self.r#rotation == 7 && self.r#waterlogged == true {
            return 5976;
        }
        if self.r#rotation == 5 && self.r#waterlogged == false && self.r#attached == true {
            return 5973;
        }
        if self.r#rotation == 5 && self.r#waterlogged == true && self.r#attached == true {
            return 5972;
        }
        if self.r#waterlogged == false && self.r#rotation == 0 && self.r#attached == false {
            return 5995;
        }
        if self.r#rotation == 11 && self.r#waterlogged == true && self.r#attached == false {
            return 6016;
        }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 0 {
            return 5962;
        }
        if self.r#attached == true && self.r#rotation == 11 && self.r#waterlogged == true {
            return 5984;
        }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 8 {
            return 6010;
        }
        if self.r#attached == true && self.r#rotation == 2 && self.r#waterlogged == true {
            return 5966;
        }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 4 {
            return 5971;
        }
        if self.r#attached == false && self.r#rotation == 1 && self.r#waterlogged == false {
            return 5997;
        }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 14 {
            return 6023;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6021 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 6013 {
            return Some(CherryHangingSign {
                r#rotation: 9,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5975 {
            return Some(CherryHangingSign {
                r#rotation: 6,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5974 {
            return Some(CherryHangingSign {
                r#rotation: 6,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5970 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 6003 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 5993 {
            return Some(CherryHangingSign {
                r#rotation: 15,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6022 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 14,
            });
        }
        if state_id == 5977 {
            return Some(CherryHangingSign {
                r#rotation: 7,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5988 {
            return Some(CherryHangingSign {
                r#rotation: 13,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6001 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#rotation: 3,
                r#attached: false,
            });
        }
        if state_id == 6024 {
            return Some(CherryHangingSign {
                r#rotation: 15,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5964 {
            return Some(CherryHangingSign {
                r#rotation: 1,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5987 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5989 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 6014 {
            return Some(CherryHangingSign {
                r#rotation: 10,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5998 {
            return Some(CherryHangingSign {
                r#rotation: 2,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6025 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5968 {
            return Some(CherryHangingSign {
                r#rotation: 3,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5996 {
            return Some(CherryHangingSign {
                r#rotation: 1,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6002 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5978 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#rotation: 8,
                r#attached: true,
            });
        }
        if state_id == 6007 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 6,
            });
        }
        if state_id == 6015 {
            return Some(CherryHangingSign {
                r#rotation: 10,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6009 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 7,
            });
        }
        if state_id == 6017 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5967 {
            return Some(CherryHangingSign {
                r#rotation: 2,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6020 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#rotation: 13,
                r#attached: false,
            });
        }
        if state_id == 5963 {
            return Some(CherryHangingSign {
                r#rotation: 0,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5999 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 5991 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 6012 {
            return Some(CherryHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6019 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 6000 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5980 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5994 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 6004 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 6011 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#rotation: 8,
                r#attached: false,
            });
        }
        if state_id == 5982 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5992 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 15,
            });
        }
        if state_id == 5981 {
            return Some(CherryHangingSign {
                r#rotation: 9,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6008 {
            return Some(CherryHangingSign {
                r#rotation: 7,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5979 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 8,
            });
        }
        if state_id == 6005 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 6006 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#rotation: 6,
                r#attached: false,
            });
        }
        if state_id == 5965 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 1,
            });
        }
        if state_id == 5983 {
            return Some(CherryHangingSign {
                r#rotation: 10,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 5986 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5990 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5969 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5985 {
            return Some(CherryHangingSign {
                r#rotation: 11,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6018 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5976 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5973 {
            return Some(CherryHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5972 {
            return Some(CherryHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5995 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#rotation: 0,
                r#attached: false,
            });
        }
        if state_id == 6016 {
            return Some(CherryHangingSign {
                r#rotation: 11,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5962 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 0,
            });
        }
        if state_id == 5984 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 11,
                r#waterlogged: true,
            });
        }
        if state_id == 6010 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 8,
            });
        }
        if state_id == 5966 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5971 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 5997 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 6023 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 14,
            });
        }
        return None;
    }
}
