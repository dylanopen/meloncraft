use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comparator {
    pub r#facing: Facing,
    pub r#mode: Mode,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Compare,
    Subtract,
}

impl BlockState for Comparator {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#mode == Mode::Compare && block_state.r#powered == false { return 11062; }
        if block_state.r#facing == Facing::South && block_state.r#mode == Mode::Compare && block_state.r#powered == true { return 11065; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#mode == Mode::Subtract { return 11067; }
        if block_state.r#facing == Facing::East && block_state.r#mode == Mode::Compare && block_state.r#powered == true { return 11073; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#mode == Mode::Compare { return 11061; }
        if block_state.r#facing == Facing::East && block_state.r#mode == Mode::Subtract && block_state.r#powered == false { return 11076; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#mode == Mode::Compare { return 11069; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#mode == Mode::Subtract { return 11075; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#mode == Mode::Subtract { return 11064; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#mode == Mode::Compare { return 11074; }
        if block_state.r#powered == false && block_state.r#mode == Mode::Subtract && block_state.r#facing == Facing::South { return 11068; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#mode == Mode::Compare { return 11066; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#mode == Mode::Subtract { return 11063; }
        if block_state.r#facing == Facing::West && block_state.r#mode == Mode::Compare && block_state.r#powered == false { return 11070; }
        if block_state.r#facing == Facing::West && block_state.r#mode == Mode::Subtract && block_state.r#powered == true { return 11071; }
        if block_state.r#mode == Mode::Subtract && block_state.r#facing == Facing::West && block_state.r#powered == false { return 11072; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11062 {
            return Some(Comparator {
                r#facing: Facing::North,
                r#mode: Mode::Compare,
                r#powered: false,
            });
        }
        if state_id == 11065 {
            return Some(Comparator {
                r#facing: Facing::South,
                r#mode: Mode::Compare,
                r#powered: true,
            });
        }
        if state_id == 11067 {
            return Some(Comparator {
                r#powered: true,
                r#facing: Facing::South,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11073 {
            return Some(Comparator {
                r#facing: Facing::East,
                r#mode: Mode::Compare,
                r#powered: true,
            });
        }
        if state_id == 11061 {
            return Some(Comparator {
                r#powered: true,
                r#facing: Facing::North,
                r#mode: Mode::Compare,
            });
        }
        if state_id == 11076 {
            return Some(Comparator {
                r#facing: Facing::East,
                r#mode: Mode::Subtract,
                r#powered: false,
            });
        }
        if state_id == 11069 {
            return Some(Comparator {
                r#powered: true,
                r#facing: Facing::West,
                r#mode: Mode::Compare,
            });
        }
        if state_id == 11075 {
            return Some(Comparator {
                r#facing: Facing::East,
                r#powered: true,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11064 {
            return Some(Comparator {
                r#facing: Facing::North,
                r#powered: false,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11074 {
            return Some(Comparator {
                r#powered: false,
                r#facing: Facing::East,
                r#mode: Mode::Compare,
            });
        }
        if state_id == 11068 {
            return Some(Comparator {
                r#powered: false,
                r#mode: Mode::Subtract,
                r#facing: Facing::South,
            });
        }
        if state_id == 11066 {
            return Some(Comparator {
                r#powered: false,
                r#facing: Facing::South,
                r#mode: Mode::Compare,
            });
        }
        if state_id == 11063 {
            return Some(Comparator {
                r#facing: Facing::North,
                r#powered: true,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11070 {
            return Some(Comparator {
                r#facing: Facing::West,
                r#mode: Mode::Compare,
                r#powered: false,
            });
        }
        if state_id == 11071 {
            return Some(Comparator {
                r#facing: Facing::West,
                r#mode: Mode::Subtract,
                r#powered: true,
            });
        }
        if state_id == 11072 {
            return Some(Comparator {
                r#mode: Mode::Subtract,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        return None;
    }
}

