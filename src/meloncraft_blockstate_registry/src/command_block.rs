use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBlock {
    pub conditional: bool,
    pub r#facing: Facing,
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

impl BlockState for CommandBlock {
    fn to_id(&self) -> i32 {
        if self.r#conditional == true && self.r#facing == Facing::South {
            return 9769;
        }
        if self.r#facing == Facing::West && self.r#conditional == true {
            return 9770;
        }
        if self.r#conditional == false && self.r#facing == Facing::East {
            return 9774;
        }
        if self.r#conditional == false && self.r#facing == Facing::Up {
            return 9777;
        }
        if self.r#facing == Facing::East && self.r#conditional == true {
            return 9768;
        }
        if self.r#conditional == false && self.r#facing == Facing::West {
            return 9776;
        }
        if self.r#conditional == true && self.r#facing == Facing::Down {
            return 9772;
        }
        if self.r#conditional == false && self.r#facing == Facing::North {
            return 9773;
        }
        if self.r#conditional == true && self.r#facing == Facing::Up {
            return 9771;
        }
        if self.r#conditional == false && self.r#facing == Facing::South {
            return 9775;
        }
        if self.r#conditional == false && self.r#facing == Facing::Down {
            return 9778;
        }
        if self.r#facing == Facing::North && self.r#conditional == true {
            return 9767;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9769 {
            return Some(CommandBlock {
                r#conditional: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9770 {
            return Some(CommandBlock {
                r#facing: Facing::West,
                r#conditional: true,
            });
        }
        if state_id == 9774 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 9777 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 9768 {
            return Some(CommandBlock {
                r#facing: Facing::East,
                r#conditional: true,
            });
        }
        if state_id == 9776 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9772 {
            return Some(CommandBlock {
                r#conditional: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 9773 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 9771 {
            return Some(CommandBlock {
                r#conditional: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 9775 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 9778 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 9767 {
            return Some(CommandBlock {
                r#facing: Facing::North,
                r#conditional: true,
            });
        }
        return None;
    }
}
