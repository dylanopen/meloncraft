use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tnt {
    pub unstable: bool,
}


impl BlockState for Tnt {
    fn to_id(self) -> i32 {
        if block_state.r#unstable == true { return 2140; }
        if block_state.r#unstable == false { return 2141; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2140 {
            return Some(Tnt {
                r#unstable: true,
            });
        }
        if state_id == 2141 {
            return Some(Tnt {
                r#unstable: false,
            });
        }
        return None;
    }
}

