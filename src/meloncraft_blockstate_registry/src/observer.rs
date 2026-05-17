use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observer {
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for Observer {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#powered == true { return 14652; }
        if self.r#facing == Facing::Up && self.r#powered == true { return 14658; }
        if self.r#facing == Facing::Up && self.r#powered == false { return 14659; }
        if self.r#facing == Facing::West && self.r#powered == true { return 14656; }
        if self.r#facing == Facing::North && self.r#powered == false { return 14651; }
        if self.r#facing == Facing::South && self.r#powered == false { return 14655; }
        if self.r#powered == true && self.r#facing == Facing::South { return 14654; }
        if self.r#powered == true && self.r#facing == Facing::Down { return 14660; }
        if self.r#powered == false && self.r#facing == Facing::Down { return 14661; }
        if self.r#powered == false && self.r#facing == Facing::West { return 14657; }
        if self.r#facing == Facing::East && self.r#powered == false { return 14653; }
        if self.r#powered == true && self.r#facing == Facing::North { return 14650; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14652 {
            return Some(Observer {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 14658 {
            return Some(Observer {
                r#facing: Facing::Up,
                r#powered: true,
            });
        }
        if state_id == 14659 {
            return Some(Observer {
                r#facing: Facing::Up,
                r#powered: false,
            });
        }
        if state_id == 14656 {
            return Some(Observer {
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 14651 {
            return Some(Observer {
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14655 {
            return Some(Observer {
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14654 {
            return Some(Observer {
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14660 {
            return Some(Observer {
                r#powered: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14661 {
            return Some(Observer {
                r#powered: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14657 {
            return Some(Observer {
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14653 {
            return Some(Observer {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14650 {
            return Some(Observer {
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

