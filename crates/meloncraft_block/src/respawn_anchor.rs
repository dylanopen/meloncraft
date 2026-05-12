use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespawnAnchor {
    pub charges: i32,
}


impl BlockState for RespawnAnchor {
    fn to_id(&self) -> i32 {
        if self.r#charges == 2 { return 21621; }
        if self.r#charges == 0 { return 21619; }
        if self.r#charges == 1 { return 21620; }
        if self.r#charges == 3 { return 21622; }
        if self.r#charges == 4 { return 21623; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21621 {
            return Some(RespawnAnchor {
                r#charges: 2,
            });
        }
        if state_id == 21619 {
            return Some(RespawnAnchor {
                r#charges: 0,
            });
        }
        if state_id == 21620 {
            return Some(RespawnAnchor {
                r#charges: 1,
            });
        }
        if state_id == 21622 {
            return Some(RespawnAnchor {
                r#charges: 3,
            });
        }
        if state_id == 21623 {
            return Some(RespawnAnchor {
                r#charges: 4,
            });
        }
        return None;
    }
}

