use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedBed {
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

impl BlockState for RedBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == true && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1955; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1969; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::South && block_state.r#occupied == true { return 1959; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::South { return 1961; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1968; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1970; }
        if block_state.r#facing == Facing::South && block_state.r#part == Part::Foot && block_state.r#occupied == true { return 1960; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::West && block_state.r#part == Part::Head { return 1963; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1967; }
        if block_state.r#facing == Facing::South && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1962; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Foot && block_state.r#occupied == true { return 1964; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1965; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::West { return 1966; }
        if block_state.r#facing == Facing::North && block_state.r#part == Part::Foot && block_state.r#occupied == true { return 1956; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::North && block_state.r#occupied == false { return 1957; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1958; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1955 {
            return Some(RedBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1969 {
            return Some(RedBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1959 {
            return Some(RedBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1961 {
            return Some(RedBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1968 {
            return Some(RedBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1970 {
            return Some(RedBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1960 {
            return Some(RedBed {
                r#facing: Facing::South,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1963 {
            return Some(RedBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1967 {
            return Some(RedBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1962 {
            return Some(RedBed {
                r#facing: Facing::South,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1964 {
            return Some(RedBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1965 {
            return Some(RedBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1966 {
            return Some(RedBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1956 {
            return Some(RedBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1957 {
            return Some(RedBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1958 {
            return Some(RedBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        return None;
    }
}

