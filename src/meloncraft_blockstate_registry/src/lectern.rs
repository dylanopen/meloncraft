use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lectern {
    pub has_book: bool,
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Lectern {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#has_book == true && self.r#powered == true { return 20590; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#has_book == false { return 20584; }
        if self.r#has_book == false && self.r#powered == false && self.r#facing == Facing::West { return 20593; }
        if self.r#powered == false && self.r#has_book == true && self.r#facing == Facing::North { return 20583; }
        if self.r#facing == Facing::South && self.r#has_book == true && self.r#powered == false { return 20587; }
        if self.r#facing == Facing::North && self.r#has_book == false && self.r#powered == false { return 20585; }
        if self.r#facing == Facing::South && self.r#has_book == false && self.r#powered == true { return 20588; }
        if self.r#facing == Facing::East && self.r#has_book == true && self.r#powered == false { return 20595; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#has_book == true { return 20582; }
        if self.r#has_book == true && self.r#facing == Facing::South && self.r#powered == true { return 20586; }
        if self.r#powered == false && self.r#has_book == false && self.r#facing == Facing::East { return 20597; }
        if self.r#has_book == false && self.r#facing == Facing::West && self.r#powered == true { return 20592; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#has_book == false { return 20589; }
        if self.r#has_book == false && self.r#powered == true && self.r#facing == Facing::East { return 20596; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#has_book == true { return 20594; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#has_book == true { return 20591; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20590 {
            return Some(Lectern {
                r#facing: Facing::West,
                r#has_book: true,
                r#powered: true,
            });
        }
        if state_id == 20584 {
            return Some(Lectern {
                r#facing: Facing::North,
                r#powered: true,
                r#has_book: false,
            });
        }
        if state_id == 20593 {
            return Some(Lectern {
                r#has_book: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20583 {
            return Some(Lectern {
                r#powered: false,
                r#has_book: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 20587 {
            return Some(Lectern {
                r#facing: Facing::South,
                r#has_book: true,
                r#powered: false,
            });
        }
        if state_id == 20585 {
            return Some(Lectern {
                r#facing: Facing::North,
                r#has_book: false,
                r#powered: false,
            });
        }
        if state_id == 20588 {
            return Some(Lectern {
                r#facing: Facing::South,
                r#has_book: false,
                r#powered: true,
            });
        }
        if state_id == 20595 {
            return Some(Lectern {
                r#facing: Facing::East,
                r#has_book: true,
                r#powered: false,
            });
        }
        if state_id == 20582 {
            return Some(Lectern {
                r#facing: Facing::North,
                r#powered: true,
                r#has_book: true,
            });
        }
        if state_id == 20586 {
            return Some(Lectern {
                r#has_book: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 20597 {
            return Some(Lectern {
                r#powered: false,
                r#has_book: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20592 {
            return Some(Lectern {
                r#has_book: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 20589 {
            return Some(Lectern {
                r#powered: false,
                r#facing: Facing::South,
                r#has_book: false,
            });
        }
        if state_id == 20596 {
            return Some(Lectern {
                r#has_book: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 20594 {
            return Some(Lectern {
                r#powered: true,
                r#facing: Facing::East,
                r#has_book: true,
            });
        }
        if state_id == 20591 {
            return Some(Lectern {
                r#facing: Facing::West,
                r#powered: false,
                r#has_book: true,
            });
        }
        return None;
    }
}

