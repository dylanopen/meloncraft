use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comparator {
    pub r#facing: Facing,
    pub powered: bool,
    pub r#mode: Mode,
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
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#mode == Mode::Compare && self.r#powered == false
        {
            return 11066;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#mode == Mode::Compare
        {
            return 11062;
        }
        if self.r#mode == Mode::Compare && self.r#powered == false && self.r#facing == Facing::East
        {
            return 11074;
        }
        if self.r#facing == Facing::East && self.r#mode == Mode::Compare && self.r#powered == true {
            return 11073;
        }
        if self.r#powered == true && self.r#mode == Mode::Subtract && self.r#facing == Facing::East
        {
            return 11075;
        }
        if self.r#mode == Mode::Compare && self.r#powered == true && self.r#facing == Facing::North
        {
            return 11061;
        }
        if self.r#mode == Mode::Subtract && self.r#powered == false && self.r#facing == Facing::East
        {
            return 11076;
        }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#mode == Mode::Subtract
        {
            return 11071;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#mode == Mode::Subtract
        {
            return 11064;
        }
        if self.r#mode == Mode::Subtract && self.r#powered == true && self.r#facing == Facing::North
        {
            return 11063;
        }
        if self.r#facing == Facing::South && self.r#mode == Mode::Compare && self.r#powered == true
        {
            return 11065;
        }
        if self.r#powered == false && self.r#mode == Mode::Compare && self.r#facing == Facing::West
        {
            return 11070;
        }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#mode == Mode::Subtract
        {
            return 11072;
        }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#mode == Mode::Subtract
        {
            return 11067;
        }
        if self.r#mode == Mode::Subtract
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 11068;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#mode == Mode::Compare {
            return 11069;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11066 {
            return Some(Comparator {
                r#facing: Facing::South,
                r#mode: Mode::Compare,
                r#powered: false,
            });
        }
        if state_id == 11062 {
            return Some(Comparator {
                r#facing: Facing::North,
                r#powered: false,
                r#mode: Mode::Compare,
            });
        }
        if state_id == 11074 {
            return Some(Comparator {
                r#mode: Mode::Compare,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 11073 {
            return Some(Comparator {
                r#facing: Facing::East,
                r#mode: Mode::Compare,
                r#powered: true,
            });
        }
        if state_id == 11075 {
            return Some(Comparator {
                r#powered: true,
                r#mode: Mode::Subtract,
                r#facing: Facing::East,
            });
        }
        if state_id == 11061 {
            return Some(Comparator {
                r#mode: Mode::Compare,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 11076 {
            return Some(Comparator {
                r#mode: Mode::Subtract,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 11071 {
            return Some(Comparator {
                r#powered: true,
                r#facing: Facing::West,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11064 {
            return Some(Comparator {
                r#powered: false,
                r#facing: Facing::North,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11063 {
            return Some(Comparator {
                r#mode: Mode::Subtract,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 11065 {
            return Some(Comparator {
                r#facing: Facing::South,
                r#mode: Mode::Compare,
                r#powered: true,
            });
        }
        if state_id == 11070 {
            return Some(Comparator {
                r#powered: false,
                r#mode: Mode::Compare,
                r#facing: Facing::West,
            });
        }
        if state_id == 11072 {
            return Some(Comparator {
                r#powered: false,
                r#facing: Facing::West,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11067 {
            return Some(Comparator {
                r#facing: Facing::South,
                r#powered: true,
                r#mode: Mode::Subtract,
            });
        }
        if state_id == 11068 {
            return Some(Comparator {
                r#mode: Mode::Subtract,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 11069 {
            return Some(Comparator {
                r#facing: Facing::West,
                r#powered: true,
                r#mode: Mode::Compare,
            });
        }
        return None;
    }
}
