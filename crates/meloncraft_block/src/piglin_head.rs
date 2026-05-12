use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiglinHead {
    pub rotation: i32,
    pub powered: bool,
}


impl BlockState for PiglinHead {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#rotation == 2 { return 10971; }
        if self.r#powered == true && self.r#rotation == 3 { return 10956; }
        if self.r#powered == true && self.r#rotation == 13 { return 10966; }
        if self.r#powered == true && self.r#rotation == 9 { return 10962; }
        if self.r#powered == true && self.r#rotation == 11 { return 10964; }
        if self.r#rotation == 2 && self.r#powered == true { return 10955; }
        if self.r#powered == true && self.r#rotation == 12 { return 10965; }
        if self.r#powered == true && self.r#rotation == 14 { return 10967; }
        if self.r#rotation == 9 && self.r#powered == false { return 10978; }
        if self.r#powered == false && self.r#rotation == 11 { return 10980; }
        if self.r#powered == false && self.r#rotation == 14 { return 10983; }
        if self.r#rotation == 6 && self.r#powered == false { return 10975; }
        if self.r#powered == false && self.r#rotation == 1 { return 10970; }
        if self.r#powered == false && self.r#rotation == 4 { return 10973; }
        if self.r#powered == false && self.r#rotation == 5 { return 10974; }
        if self.r#rotation == 8 && self.r#powered == false { return 10977; }
        if self.r#powered == false && self.r#rotation == 10 { return 10979; }
        if self.r#rotation == 10 && self.r#powered == true { return 10963; }
        if self.r#rotation == 12 && self.r#powered == false { return 10981; }
        if self.r#powered == false && self.r#rotation == 13 { return 10982; }
        if self.r#powered == true && self.r#rotation == 4 { return 10957; }
        if self.r#powered == true && self.r#rotation == 0 { return 10953; }
        if self.r#rotation == 7 && self.r#powered == true { return 10960; }
        if self.r#powered == true && self.r#rotation == 5 { return 10958; }
        if self.r#rotation == 15 && self.r#powered == true { return 10968; }
        if self.r#powered == false && self.r#rotation == 15 { return 10984; }
        if self.r#rotation == 8 && self.r#powered == true { return 10961; }
        if self.r#rotation == 0 && self.r#powered == false { return 10969; }
        if self.r#rotation == 7 && self.r#powered == false { return 10976; }
        if self.r#powered == true && self.r#rotation == 6 { return 10959; }
        if self.r#powered == true && self.r#rotation == 1 { return 10954; }
        if self.r#powered == false && self.r#rotation == 3 { return 10972; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10971 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 2,
            });
        }
        if state_id == 10956 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 3,
            });
        }
        if state_id == 10966 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 13,
            });
        }
        if state_id == 10962 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 9,
            });
        }
        if state_id == 10964 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 11,
            });
        }
        if state_id == 10955 {
            return Some(PiglinHead {
                r#rotation: 2,
                r#powered: true,
            });
        }
        if state_id == 10965 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 12,
            });
        }
        if state_id == 10967 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10978 {
            return Some(PiglinHead {
                r#rotation: 9,
                r#powered: false,
            });
        }
        if state_id == 10980 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 11,
            });
        }
        if state_id == 10983 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 14,
            });
        }
        if state_id == 10975 {
            return Some(PiglinHead {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10970 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 1,
            });
        }
        if state_id == 10973 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 4,
            });
        }
        if state_id == 10974 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 5,
            });
        }
        if state_id == 10977 {
            return Some(PiglinHead {
                r#rotation: 8,
                r#powered: false,
            });
        }
        if state_id == 10979 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 10,
            });
        }
        if state_id == 10963 {
            return Some(PiglinHead {
                r#rotation: 10,
                r#powered: true,
            });
        }
        if state_id == 10981 {
            return Some(PiglinHead {
                r#rotation: 12,
                r#powered: false,
            });
        }
        if state_id == 10982 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 13,
            });
        }
        if state_id == 10957 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 4,
            });
        }
        if state_id == 10953 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 0,
            });
        }
        if state_id == 10960 {
            return Some(PiglinHead {
                r#rotation: 7,
                r#powered: true,
            });
        }
        if state_id == 10958 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10968 {
            return Some(PiglinHead {
                r#rotation: 15,
                r#powered: true,
            });
        }
        if state_id == 10984 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 15,
            });
        }
        if state_id == 10961 {
            return Some(PiglinHead {
                r#rotation: 8,
                r#powered: true,
            });
        }
        if state_id == 10969 {
            return Some(PiglinHead {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10976 {
            return Some(PiglinHead {
                r#rotation: 7,
                r#powered: false,
            });
        }
        if state_id == 10959 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 6,
            });
        }
        if state_id == 10954 {
            return Some(PiglinHead {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10972 {
            return Some(PiglinHead {
                r#powered: false,
                r#rotation: 3,
            });
        }
        return None;
    }
}

