use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryHangingSign {
    pub waterlogged: bool,
    pub attached: bool,
    pub rotation: i32,
}


impl BlockState for CherryHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#attached == true && block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5981; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 0 { return 5962; }
        if block_state.r#attached == false && block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5997; }
        if block_state.r#attached == true && block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5964; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 6000; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5991; }
        if block_state.r#rotation == 10 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6015; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5985; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5984; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5965; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 4 { return 6003; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5983; }
        if block_state.r#attached == false && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 6012; }
        if block_state.r#attached == false && block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 6022; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6023; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 7 { return 5976; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5967; }
        if block_state.r#attached == false && block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 6004; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 13 { return 6020; }
        if block_state.r#attached == false && block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 6018; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 8 { return 5979; }
        if block_state.r#rotation == 12 && block_state.r#attached == true && block_state.r#waterlogged == true { return 5986; }
        if block_state.r#rotation == 8 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6010; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 5966; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5998; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 15 && block_state.r#attached == true { return 5993; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 4 { return 6002; }
        if block_state.r#attached == false && block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 6019; }
        if block_state.r#rotation == 7 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6008; }
        if block_state.r#attached == true && block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 5982; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6021; }
        if block_state.r#attached == false && block_state.r#rotation == 7 && block_state.r#waterlogged == false { return 6009; }
        if block_state.r#attached == true && block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5968; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 9 { return 6013; }
        if block_state.r#attached == false && block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 6017; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 && block_state.r#attached == true { return 5977; }
        if block_state.r#attached == true && block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 5978; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 3 { return 5969; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true && block_state.r#attached == true { return 5990; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 13 { return 5989; }
        if block_state.r#attached == true && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5963; }
        if block_state.r#attached == true && block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5972; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true && block_state.r#attached == false { return 5994; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 6 { return 5974; }
        if block_state.r#attached == true && block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5980; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 15 { return 5992; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 8 { return 6011; }
        if block_state.r#rotation == 15 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6025; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 2 { return 5999; }
        if block_state.r#attached == false && block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 6005; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 4 { return 5971; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5987; }
        if block_state.r#attached == false && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5995; }
        if block_state.r#attached == false && block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5996; }
        if block_state.r#rotation == 6 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6006; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6024; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 6 && block_state.r#attached == true { return 5975; }
        if block_state.r#attached == true && block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 5970; }
        if block_state.r#attached == true && block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5988; }
        if block_state.r#attached == false && block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 6007; }
        if block_state.r#rotation == 11 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6016; }
        if block_state.r#rotation == 3 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6001; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false && block_state.r#attached == true { return 5973; }
        if block_state.r#attached == false && block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 6014; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5981 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5962 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 5997 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5964 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 6000 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5991 {
            return Some(CherryHangingSign {
                r#rotation: 14,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6015 {
            return Some(CherryHangingSign {
                r#rotation: 10,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5985 {
            return Some(CherryHangingSign {
                r#rotation: 11,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5984 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5965 {
            return Some(CherryHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6003 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 5983 {
            return Some(CherryHangingSign {
                r#rotation: 10,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6012 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6022 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 6023 {
            return Some(CherryHangingSign {
                r#rotation: 14,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 5976 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 7,
            });
        }
        if state_id == 5967 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 6004 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 6020 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 6018 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 12,
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
        if state_id == 5986 {
            return Some(CherryHangingSign {
                r#rotation: 12,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6010 {
            return Some(CherryHangingSign {
                r#rotation: 8,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5966 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5998 {
            return Some(CherryHangingSign {
                r#rotation: 2,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5993 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#rotation: 15,
                r#attached: true,
            });
        }
        if state_id == 6002 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 6019 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 6008 {
            return Some(CherryHangingSign {
                r#rotation: 7,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 5982 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 6021 {
            return Some(CherryHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6009 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5968 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 6013 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 6017 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5977 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#rotation: 7,
                r#attached: true,
            });
        }
        if state_id == 5978 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5969 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 3,
            });
        }
        if state_id == 5990 {
            return Some(CherryHangingSign {
                r#rotation: 14,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 5989 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 13,
            });
        }
        if state_id == 5963 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5972 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5994 {
            return Some(CherryHangingSign {
                r#rotation: 0,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5974 {
            return Some(CherryHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 6,
            });
        }
        if state_id == 5980 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 9,
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
        if state_id == 6011 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 8,
            });
        }
        if state_id == 6025 {
            return Some(CherryHangingSign {
                r#rotation: 15,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5999 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 2,
            });
        }
        if state_id == 6005 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5971 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 5987 {
            return Some(CherryHangingSign {
                r#rotation: 12,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 5995 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5996 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 6006 {
            return Some(CherryHangingSign {
                r#rotation: 6,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6024 {
            return Some(CherryHangingSign {
                r#rotation: 15,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 5975 {
            return Some(CherryHangingSign {
                r#waterlogged: false,
                r#rotation: 6,
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
        if state_id == 5988 {
            return Some(CherryHangingSign {
                r#attached: true,
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 6007 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 6016 {
            return Some(CherryHangingSign {
                r#rotation: 11,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6001 {
            return Some(CherryHangingSign {
                r#rotation: 3,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 5973 {
            return Some(CherryHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6014 {
            return Some(CherryHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

