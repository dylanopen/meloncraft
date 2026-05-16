use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedBed {
    pub r#part: Part,
    pub r#facing: Facing,
    pub occupied: bool,
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
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Foot { return 1956; }
        if self.r#occupied == true && self.r#facing == Facing::South && self.r#part == Part::Foot { return 1960; }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == true { return 1964; }
        if self.r#facing == Facing::North && self.r#part == Part::Head && self.r#occupied == true { return 1955; }
        if self.r#facing == Facing::South && self.r#occupied == true && self.r#part == Part::Head { return 1959; }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head { return 1965; }
        if self.r#part == Part::Foot && self.r#facing == Facing::East && self.r#occupied == false { return 1970; }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Head { return 1961; }
        if self.r#facing == Facing::North && self.r#occupied == false && self.r#part == Part::Foot { return 1958; }
        if self.r#part == Part::Head && self.r#facing == Facing::West && self.r#occupied == true { return 1963; }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Head { return 1967; }
        if self.r#occupied == false && self.r#facing == Facing::South && self.r#part == Part::Foot { return 1962; }
        if self.r#facing == Facing::North && self.r#part == Part::Head && self.r#occupied == false { return 1957; }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Head { return 1969; }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Foot { return 1968; }
        if self.r#occupied == false && self.r#facing == Facing::West && self.r#part == Part::Foot { return 1966; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1956 {
            return Some(RedBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1960 {
            return Some(RedBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1964 {
            return Some(RedBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1955 {
            return Some(RedBed {
                r#facing: Facing::North,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1959 {
            return Some(RedBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1965 {
            return Some(RedBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1970 {
            return Some(RedBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1961 {
            return Some(RedBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1958 {
            return Some(RedBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1963 {
            return Some(RedBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: true,
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
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1957 {
            return Some(RedBed {
                r#facing: Facing::North,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1969 {
            return Some(RedBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1968 {
            return Some(RedBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1966 {
            return Some(RedBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        return None;
    }
}

