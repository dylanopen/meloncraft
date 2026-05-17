use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatingCommandBlock {
    pub r#facing: Facing,
    pub conditional: bool,
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

impl BlockState for RepeatingCommandBlock {
    fn to_id(&self) -> i32 {
        if self.r#conditional == false && self.r#facing == Facing::Up { return 14625; }
        if self.r#conditional == true && self.r#facing == Facing::South { return 14617; }
        if self.r#conditional == true && self.r#facing == Facing::West { return 14618; }
        if self.r#conditional == false && self.r#facing == Facing::North { return 14621; }
        if self.r#conditional == true && self.r#facing == Facing::Up { return 14619; }
        if self.r#conditional == true && self.r#facing == Facing::Down { return 14620; }
        if self.r#conditional == true && self.r#facing == Facing::East { return 14616; }
        if self.r#facing == Facing::West && self.r#conditional == false { return 14624; }
        if self.r#facing == Facing::North && self.r#conditional == true { return 14615; }
        if self.r#conditional == false && self.r#facing == Facing::Down { return 14626; }
        if self.r#conditional == false && self.r#facing == Facing::East { return 14622; }
        if self.r#conditional == false && self.r#facing == Facing::South { return 14623; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14625 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 14617 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14618 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14621 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14619 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 14620 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14616 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14624 {
            return Some(RepeatingCommandBlock {
                r#facing: Facing::West,
                r#conditional: false,
            });
        }
        if state_id == 14615 {
            return Some(RepeatingCommandBlock {
                r#facing: Facing::North,
                r#conditional: true,
            });
        }
        if state_id == 14626 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14622 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14623 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

