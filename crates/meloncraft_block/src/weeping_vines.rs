use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeepingVines {
    pub age: i32,
}


impl BlockState for WeepingVines {
    fn to_id(self) -> i32 {
        if block_state.r#age == 4 { return 20779; }
        if block_state.r#age == 8 { return 20783; }
        if block_state.r#age == 3 { return 20778; }
        if block_state.r#age == 10 { return 20785; }
        if block_state.r#age == 16 { return 20791; }
        if block_state.r#age == 2 { return 20777; }
        if block_state.r#age == 11 { return 20786; }
        if block_state.r#age == 14 { return 20789; }
        if block_state.r#age == 17 { return 20792; }
        if block_state.r#age == 22 { return 20797; }
        if block_state.r#age == 13 { return 20788; }
        if block_state.r#age == 18 { return 20793; }
        if block_state.r#age == 19 { return 20794; }
        if block_state.r#age == 25 { return 20800; }
        if block_state.r#age == 1 { return 20776; }
        if block_state.r#age == 21 { return 20796; }
        if block_state.r#age == 5 { return 20780; }
        if block_state.r#age == 24 { return 20799; }
        if block_state.r#age == 12 { return 20787; }
        if block_state.r#age == 7 { return 20782; }
        if block_state.r#age == 23 { return 20798; }
        if block_state.r#age == 0 { return 20775; }
        if block_state.r#age == 6 { return 20781; }
        if block_state.r#age == 15 { return 20790; }
        if block_state.r#age == 9 { return 20784; }
        if block_state.r#age == 20 { return 20795; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20779 {
            return Some(WeepingVines {
                r#age: 4,
            });
        }
        if state_id == 20783 {
            return Some(WeepingVines {
                r#age: 8,
            });
        }
        if state_id == 20778 {
            return Some(WeepingVines {
                r#age: 3,
            });
        }
        if state_id == 20785 {
            return Some(WeepingVines {
                r#age: 10,
            });
        }
        if state_id == 20791 {
            return Some(WeepingVines {
                r#age: 16,
            });
        }
        if state_id == 20777 {
            return Some(WeepingVines {
                r#age: 2,
            });
        }
        if state_id == 20786 {
            return Some(WeepingVines {
                r#age: 11,
            });
        }
        if state_id == 20789 {
            return Some(WeepingVines {
                r#age: 14,
            });
        }
        if state_id == 20792 {
            return Some(WeepingVines {
                r#age: 17,
            });
        }
        if state_id == 20797 {
            return Some(WeepingVines {
                r#age: 22,
            });
        }
        if state_id == 20788 {
            return Some(WeepingVines {
                r#age: 13,
            });
        }
        if state_id == 20793 {
            return Some(WeepingVines {
                r#age: 18,
            });
        }
        if state_id == 20794 {
            return Some(WeepingVines {
                r#age: 19,
            });
        }
        if state_id == 20800 {
            return Some(WeepingVines {
                r#age: 25,
            });
        }
        if state_id == 20776 {
            return Some(WeepingVines {
                r#age: 1,
            });
        }
        if state_id == 20796 {
            return Some(WeepingVines {
                r#age: 21,
            });
        }
        if state_id == 20780 {
            return Some(WeepingVines {
                r#age: 5,
            });
        }
        if state_id == 20799 {
            return Some(WeepingVines {
                r#age: 24,
            });
        }
        if state_id == 20787 {
            return Some(WeepingVines {
                r#age: 12,
            });
        }
        if state_id == 20782 {
            return Some(WeepingVines {
                r#age: 7,
            });
        }
        if state_id == 20798 {
            return Some(WeepingVines {
                r#age: 23,
            });
        }
        if state_id == 20775 {
            return Some(WeepingVines {
                r#age: 0,
            });
        }
        if state_id == 20781 {
            return Some(WeepingVines {
                r#age: 6,
            });
        }
        if state_id == 20790 {
            return Some(WeepingVines {
                r#age: 15,
            });
        }
        if state_id == 20784 {
            return Some(WeepingVines {
                r#age: 9,
            });
        }
        if state_id == 20795 {
            return Some(WeepingVines {
                r#age: 20,
            });
        }
        return None;
    }
}

