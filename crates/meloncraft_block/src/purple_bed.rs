use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleBed {
    pub occupied: bool,
    pub r#facing: Facing,
    pub r#part: Part,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Head,
    Foot,
}

impl BlockState for PurpleBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::North { return 1894; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::North && block_state.r#occupied == false { return 1893; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::West && block_state.r#part == Part::Foot { return 1902; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1901; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1905; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1895; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1903; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::South { return 1897; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1891; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == true && block_state.r#facing == Facing::West { return 1900; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::North && block_state.r#part == Part::Foot { return 1892; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::South && block_state.r#part == Part::Foot { return 1898; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == true && block_state.r#facing == Facing::East { return 1904; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::South && block_state.r#occupied == true { return 1896; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1899; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::East && block_state.r#occupied == false { return 1906; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1894 {
            return Some(PurpleBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1893 {
            return Some(PurpleBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1902 {
            return Some(PurpleBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        if state_id == 1901 {
            return Some(PurpleBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1905 {
            return Some(PurpleBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1895 {
            return Some(PurpleBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1903 {
            return Some(PurpleBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1897 {
            return Some(PurpleBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1891 {
            return Some(PurpleBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1900 {
            return Some(PurpleBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 1892 {
            return Some(PurpleBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1898 {
            return Some(PurpleBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1904 {
            return Some(PurpleBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1896 {
            return Some(PurpleBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1899 {
            return Some(PurpleBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1906 {
            return Some(PurpleBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        return None;
    }
}

