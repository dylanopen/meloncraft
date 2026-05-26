use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadHornCoralWallFan {
    pub waterlogged: bool,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for DeadHornCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::South {
            return 15020;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 15018;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East {
            return 15023;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == false {
            return 15022;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::West {
            return 15021;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 15019;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 15017;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::East {
            return 15024;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15020 {
            return Some(DeadHornCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15018 {
            return Some(DeadHornCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15023 {
            return Some(DeadHornCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15022 {
            return Some(DeadHornCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15021 {
            return Some(DeadHornCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15019 {
            return Some(DeadHornCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15017 {
            return Some(DeadHornCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15024 {
            return Some(DeadHornCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
