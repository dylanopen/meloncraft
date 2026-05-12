use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackBed {
    pub occupied: bool,
    pub r#part: Part,
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

impl BlockState for BlackBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1973; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::West && block_state.r#part == Part::Head { return 1979; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1980; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::South && block_state.r#occupied == false { return 1977; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::South && block_state.r#occupied == true { return 1976; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1982; }
        if block_state.r#facing == Facing::North && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1974; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::South && block_state.r#occupied == false { return 1978; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1986; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1985; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::East && block_state.r#part == Part::Foot { return 1984; }
        if block_state.r#part == Part::Head && block_state.r#occupied == true && block_state.r#facing == Facing::East { return 1983; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1975; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1972; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::North && block_state.r#occupied == true { return 1971; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::West && block_state.r#occupied == false { return 1981; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1973 {
            return Some(BlackBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1979 {
            return Some(BlackBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1980 {
            return Some(BlackBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1977 {
            return Some(BlackBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1976 {
            return Some(BlackBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1982 {
            return Some(BlackBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1974 {
            return Some(BlackBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1978 {
            return Some(BlackBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1986 {
            return Some(BlackBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1985 {
            return Some(BlackBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1984 {
            return Some(BlackBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Foot,
            });
        }
        if state_id == 1983 {
            return Some(BlackBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1975 {
            return Some(BlackBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1972 {
            return Some(BlackBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1971 {
            return Some(BlackBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1981 {
            return Some(BlackBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        return None;
    }
}

