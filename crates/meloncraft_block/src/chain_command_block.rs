use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainCommandBlock {
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

impl BlockState for ChainCommandBlock {
    fn to_id(&self) -> i32 {
        if self.r#conditional == true && self.r#facing == Facing::Down { return 14632; }
        if self.r#conditional == true && self.r#facing == Facing::Up { return 14631; }
        if self.r#conditional == false && self.r#facing == Facing::East { return 14634; }
        if self.r#facing == Facing::West && self.r#conditional == true { return 14630; }
        if self.r#conditional == false && self.r#facing == Facing::South { return 14635; }
        if self.r#conditional == false && self.r#facing == Facing::West { return 14636; }
        if self.r#facing == Facing::South && self.r#conditional == true { return 14629; }
        if self.r#facing == Facing::Up && self.r#conditional == false { return 14637; }
        if self.r#facing == Facing::North && self.r#conditional == false { return 14633; }
        if self.r#conditional == false && self.r#facing == Facing::Down { return 14638; }
        if self.r#conditional == true && self.r#facing == Facing::East { return 14628; }
        if self.r#facing == Facing::North && self.r#conditional == true { return 14627; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14632 {
            return Some(ChainCommandBlock {
                r#conditional: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14631 {
            return Some(ChainCommandBlock {
                r#conditional: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 14634 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14630 {
            return Some(ChainCommandBlock {
                r#facing: Facing::West,
                r#conditional: true,
            });
        }
        if state_id == 14635 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14636 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14629 {
            return Some(ChainCommandBlock {
                r#facing: Facing::South,
                r#conditional: true,
            });
        }
        if state_id == 14637 {
            return Some(ChainCommandBlock {
                r#facing: Facing::Up,
                r#conditional: false,
            });
        }
        if state_id == 14633 {
            return Some(ChainCommandBlock {
                r#facing: Facing::North,
                r#conditional: false,
            });
        }
        if state_id == 14638 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14628 {
            return Some(ChainCommandBlock {
                r#conditional: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14627 {
            return Some(ChainCommandBlock {
                r#facing: Facing::North,
                r#conditional: true,
            });
        }
        return None;
    }
}

