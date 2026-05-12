use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkBed {
    pub r#part: Part,
    pub occupied: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Head,
    Foot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PinkBed {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1832; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::South && block_state.r#part == Part::Head { return 1833; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::East && block_state.r#occupied == true { return 1840; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::East && block_state.r#occupied == false { return 1842; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::North && block_state.r#part == Part::Foot { return 1828; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::East && block_state.r#part == Part::Head { return 1839; }
        if block_state.r#facing == Facing::South && block_state.r#part == Part::Head && block_state.r#occupied == true { return 1831; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1829; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1835; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1827; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::North { return 1830; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1838; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1836; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::South && block_state.r#part == Part::Foot { return 1834; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::West && block_state.r#occupied == false { return 1837; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1841; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1832 {
            return Some(PinkBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1833 {
            return Some(PinkBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Head,
            });
        }
        if state_id == 1840 {
            return Some(PinkBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: true,
            });
        }
        if state_id == 1842 {
            return Some(PinkBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1828 {
            return Some(PinkBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1839 {
            return Some(PinkBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Head,
            });
        }
        if state_id == 1831 {
            return Some(PinkBed {
                r#facing: Facing::South,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1829 {
            return Some(PinkBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1835 {
            return Some(PinkBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1827 {
            return Some(PinkBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1830 {
            return Some(PinkBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1838 {
            return Some(PinkBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1836 {
            return Some(PinkBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1834 {
            return Some(PinkBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1837 {
            return Some(PinkBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1841 {
            return Some(PinkBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

