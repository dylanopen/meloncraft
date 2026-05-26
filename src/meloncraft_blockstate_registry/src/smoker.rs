use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Smoker {
    pub r#facing: Facing,
    pub lit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Smoker {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#lit == true {
            return 20552;
        }
        if self.r#lit == false && self.r#facing == Facing::South {
            return 20555;
        }
        if self.r#facing == Facing::West && self.r#lit == true {
            return 20556;
        }
        if self.r#facing == Facing::West && self.r#lit == false {
            return 20557;
        }
        if self.r#facing == Facing::East && self.r#lit == false {
            return 20559;
        }
        if self.r#lit == true && self.r#facing == Facing::South {
            return 20554;
        }
        if self.r#facing == Facing::East && self.r#lit == true {
            return 20558;
        }
        if self.r#lit == false && self.r#facing == Facing::North {
            return 20553;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20552 {
            return Some(Smoker {
                r#facing: Facing::North,
                r#lit: true,
            });
        }
        if state_id == 20555 {
            return Some(Smoker {
                r#lit: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 20556 {
            return Some(Smoker {
                r#facing: Facing::West,
                r#lit: true,
            });
        }
        if state_id == 20557 {
            return Some(Smoker {
                r#facing: Facing::West,
                r#lit: false,
            });
        }
        if state_id == 20559 {
            return Some(Smoker {
                r#facing: Facing::East,
                r#lit: false,
            });
        }
        if state_id == 20554 {
            return Some(Smoker {
                r#lit: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 20558 {
            return Some(Smoker {
                r#facing: Facing::East,
                r#lit: true,
            });
        }
        if state_id == 20553 {
            return Some(Smoker {
                r#lit: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
