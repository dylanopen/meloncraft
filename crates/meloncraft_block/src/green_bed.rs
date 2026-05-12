use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenBed {
    pub r#facing: Facing,
    pub occupied: bool,
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

impl BlockState for GreenBed {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1953; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1949; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::South && block_state.r#occupied == false { return 1945; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1947; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1951; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::North { return 1940; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == false && block_state.r#facing == Facing::South { return 1946; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == true && block_state.r#facing == Facing::South { return 1944; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1948; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1954; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1939; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1942; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::North { return 1941; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == true && block_state.r#facing == Facing::East { return 1952; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::West && block_state.r#part == Part::Foot { return 1950; }
        if block_state.r#facing == Facing::South && block_state.r#part == Part::Head && block_state.r#occupied == true { return 1943; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1953 {
            return Some(GreenBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1949 {
            return Some(GreenBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1945 {
            return Some(GreenBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1947 {
            return Some(GreenBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1951 {
            return Some(GreenBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1940 {
            return Some(GreenBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1946 {
            return Some(GreenBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1944 {
            return Some(GreenBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 1948 {
            return Some(GreenBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1954 {
            return Some(GreenBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1939 {
            return Some(GreenBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1942 {
            return Some(GreenBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1941 {
            return Some(GreenBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 1952 {
            return Some(GreenBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1950 {
            return Some(GreenBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        if state_id == 1943 {
            return Some(GreenBed {
                r#facing: Facing::South,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        return None;
    }
}

