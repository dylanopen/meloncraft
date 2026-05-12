use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanBed {
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

impl BlockState for CyanBed {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1885; }
        if block_state.r#part == Part::Head && block_state.r#occupied == true && block_state.r#facing == Facing::North { return 1875; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == false && block_state.r#facing == Facing::South { return 1882; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::East { return 1889; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::East && block_state.r#occupied == false { return 1890; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1888; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1880; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::West { return 1886; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::East && block_state.r#part == Part::Head { return 1887; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1876; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1884; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1879; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::North && block_state.r#occupied == false { return 1877; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::West && block_state.r#occupied == true { return 1883; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::South { return 1881; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::North { return 1878; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1885 {
            return Some(CyanBed {
                r#facing: Facing::West,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1875 {
            return Some(CyanBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 1882 {
            return Some(CyanBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1889 {
            return Some(CyanBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1890 {
            return Some(CyanBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1888 {
            return Some(CyanBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1880 {
            return Some(CyanBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1886 {
            return Some(CyanBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1887 {
            return Some(CyanBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Head,
            });
        }
        if state_id == 1876 {
            return Some(CyanBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1884 {
            return Some(CyanBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1879 {
            return Some(CyanBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1877 {
            return Some(CyanBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1883 {
            return Some(CyanBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1881 {
            return Some(CyanBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1878 {
            return Some(CyanBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

