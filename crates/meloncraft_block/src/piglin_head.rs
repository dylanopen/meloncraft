use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiglinHead {
    pub powered: bool,
    pub rotation: i32,
}


impl BlockState for PiglinHead {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#rotation == 1 { return 10954; }
        if block_state.r#rotation == 2 && block_state.r#powered == false { return 10971; }
        if block_state.r#rotation == 9 && block_state.r#powered == true { return 10962; }
        if block_state.r#rotation == 3 && block_state.r#powered == false { return 10972; }
        if block_state.r#rotation == 12 && block_state.r#powered == true { return 10965; }
        if block_state.r#rotation == 15 && block_state.r#powered == true { return 10968; }
        if block_state.r#rotation == 12 && block_state.r#powered == false { return 10981; }
        if block_state.r#powered == true && block_state.r#rotation == 13 { return 10966; }
        if block_state.r#rotation == 14 && block_state.r#powered == true { return 10967; }
        if block_state.r#powered == false && block_state.r#rotation == 7 { return 10976; }
        if block_state.r#powered == true && block_state.r#rotation == 10 { return 10963; }
        if block_state.r#powered == true && block_state.r#rotation == 2 { return 10955; }
        if block_state.r#rotation == 4 && block_state.r#powered == false { return 10973; }
        if block_state.r#rotation == 14 && block_state.r#powered == false { return 10983; }
        if block_state.r#rotation == 5 && block_state.r#powered == false { return 10974; }
        if block_state.r#rotation == 15 && block_state.r#powered == false { return 10984; }
        if block_state.r#powered == true && block_state.r#rotation == 3 { return 10956; }
        if block_state.r#rotation == 6 && block_state.r#powered == false { return 10975; }
        if block_state.r#rotation == 11 && block_state.r#powered == true { return 10964; }
        if block_state.r#rotation == 9 && block_state.r#powered == false { return 10978; }
        if block_state.r#rotation == 0 && block_state.r#powered == true { return 10953; }
        if block_state.r#powered == false && block_state.r#rotation == 8 { return 10977; }
        if block_state.r#powered == false && block_state.r#rotation == 10 { return 10979; }
        if block_state.r#powered == true && block_state.r#rotation == 5 { return 10958; }
        if block_state.r#rotation == 0 && block_state.r#powered == false { return 10969; }
        if block_state.r#rotation == 13 && block_state.r#powered == false { return 10982; }
        if block_state.r#powered == true && block_state.r#rotation == 7 { return 10960; }
        if block_state.r#rotation == 11 && block_state.r#powered == false { return 10980; }
        if block_state.r#powered == true && block_state.r#rotation == 8 { return 10961; }
        if block_state.r#rotation == 6 && block_state.r#powered == true { return 10959; }
        if block_state.r#powered == false && block_state.r#rotation == 1 { return 10970; }
        if block_state.r#powered == true && block_state.r#rotation == 4 { return 10957; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10954 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10971 {
            return Some(PiglinHead {
                r#rotation: 2,
                r#powered: false,
            });
        }
        if state_id == 10962 {
            return Some(PiglinHead {
                r#rotation: 9,
                r#powered: true,
            });
        }
        if state_id == 10972 {
            return Some(PiglinHead {
                r#rotation: 3,
                r#powered: false,
            });
        }
        if state_id == 10965 {
            return Some(PiglinHead {
                r#rotation: 12,
                r#powered: true,
            });
        }
        if state_id == 10968 {
            return Some(PiglinHead {
                r#rotation: 15,
                r#powered: true,
            });
        }
        if state_id == 10981 {
            return Some(PiglinHead {
                r#rotation: 12,
                r#powered: false,
            });
        }
        if state_id == 10966 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 13,
            });
        }
        if state_id == 10967 {
            return Some(PiglinHead {
                r#rotation: 14,
                r#powered: true,
            });
        }
        if state_id == 10976 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 7,
            });
        }
        if state_id == 10963 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 10,
            });
        }
        if state_id == 10955 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 2,
            });
        }
        if state_id == 10973 {
            return Some(PiglinHead {
                r#rotation: 4,
                r#powered: false,
            });
        }
        if state_id == 10983 {
            return Some(PiglinHead {
                r#rotation: 14,
                r#powered: false,
            });
        }
        if state_id == 10974 {
            return Some(PiglinHead {
                r#rotation: 5,
                r#powered: false,
            });
        }
        if state_id == 10984 {
            return Some(PiglinHead {
                r#rotation: 15,
                r#powered: false,
            });
        }
        if state_id == 10956 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 3,
            });
        }
        if state_id == 10975 {
            return Some(PiglinHead {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10964 {
            return Some(PiglinHead {
                r#rotation: 11,
                r#powered: true,
            });
        }
        if state_id == 10978 {
            return Some(PiglinHead {
                r#rotation: 9,
                r#powered: false,
            });
        }
        if state_id == 10953 {
            return Some(PiglinHead {
                r#rotation: 0,
                r#powered: true,
            });
        }
        if state_id == 10977 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 8,
            });
        }
        if state_id == 10979 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10958 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10969 {
            return Some(PiglinHead {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10982 {
            return Some(PiglinHead {
                r#rotation: 13,
                r#powered: false,
            });
        }
        if state_id == 10960 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 7,
            });
        }
        if state_id == 10980 {
            return Some(PiglinHead {
                r#rotation: 11,
                r#powered: false,
            });
        }
        if state_id == 10961 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 8,
            });
        }
        if state_id == 10959 {
            return Some(PiglinHead {
                r#rotation: 6,
                r#powered: true,
            });
        }
        if state_id == 10970 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 1,
            });
        }
        if state_id == 10957 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 4,
            });
        }
        return None;
    }
}

