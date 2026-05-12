use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurtleEgg {
    pub eggs: i32,
    pub hatch: i32,
}


impl BlockState for TurtleEgg {
    fn to_id(&self) -> i32 {
        if self.r#hatch == 0 && self.r#eggs == 4 { return 14897; }
        if self.r#hatch == 0 && self.r#eggs == 1 { return 14888; }
        if self.r#eggs == 3 && self.r#hatch == 1 { return 14895; }
        if self.r#eggs == 2 && self.r#hatch == 2 { return 14893; }
        if self.r#hatch == 1 && self.r#eggs == 1 { return 14889; }
        if self.r#eggs == 4 && self.r#hatch == 2 { return 14899; }
        if self.r#hatch == 0 && self.r#eggs == 3 { return 14894; }
        if self.r#hatch == 0 && self.r#eggs == 2 { return 14891; }
        if self.r#eggs == 3 && self.r#hatch == 2 { return 14896; }
        if self.r#hatch == 1 && self.r#eggs == 2 { return 14892; }
        if self.r#hatch == 2 && self.r#eggs == 1 { return 14890; }
        if self.r#eggs == 4 && self.r#hatch == 1 { return 14898; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14897 {
            return Some(TurtleEgg {
                r#hatch: 0,
                r#eggs: 4,
            });
        }
        if state_id == 14888 {
            return Some(TurtleEgg {
                r#hatch: 0,
                r#eggs: 1,
            });
        }
        if state_id == 14895 {
            return Some(TurtleEgg {
                r#eggs: 3,
                r#hatch: 1,
            });
        }
        if state_id == 14893 {
            return Some(TurtleEgg {
                r#eggs: 2,
                r#hatch: 2,
            });
        }
        if state_id == 14889 {
            return Some(TurtleEgg {
                r#hatch: 1,
                r#eggs: 1,
            });
        }
        if state_id == 14899 {
            return Some(TurtleEgg {
                r#eggs: 4,
                r#hatch: 2,
            });
        }
        if state_id == 14894 {
            return Some(TurtleEgg {
                r#hatch: 0,
                r#eggs: 3,
            });
        }
        if state_id == 14891 {
            return Some(TurtleEgg {
                r#hatch: 0,
                r#eggs: 2,
            });
        }
        if state_id == 14896 {
            return Some(TurtleEgg {
                r#eggs: 3,
                r#hatch: 2,
            });
        }
        if state_id == 14892 {
            return Some(TurtleEgg {
                r#hatch: 1,
                r#eggs: 2,
            });
        }
        if state_id == 14890 {
            return Some(TurtleEgg {
                r#hatch: 2,
                r#eggs: 1,
            });
        }
        if state_id == 14898 {
            return Some(TurtleEgg {
                r#eggs: 4,
                r#hatch: 1,
            });
        }
        return None;
    }
}

