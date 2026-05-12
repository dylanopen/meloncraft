use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownBed {
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

impl BlockState for BrownBed {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1933; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::South && block_state.r#part == Part::Head { return 1927; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1930; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::West && block_state.r#occupied == false { return 1934; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1938; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1925; }
        if block_state.r#facing == Facing::South && block_state.r#part == Part::Foot && block_state.r#occupied == true { return 1928; }
        if block_state.r#facing == Facing::East && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1937; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::West && block_state.r#occupied == true { return 1932; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == true && block_state.r#facing == Facing::North { return 1924; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::South { return 1929; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1936; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::North { return 1926; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1935; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::West && block_state.r#occupied == true { return 1931; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1923; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1933 {
            return Some(BrownBed {
                r#facing: Facing::West,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1927 {
            return Some(BrownBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Head,
            });
        }
        if state_id == 1930 {
            return Some(BrownBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1934 {
            return Some(BrownBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1938 {
            return Some(BrownBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1925 {
            return Some(BrownBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1928 {
            return Some(BrownBed {
                r#facing: Facing::South,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1937 {
            return Some(BrownBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1932 {
            return Some(BrownBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1924 {
            return Some(BrownBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 1929 {
            return Some(BrownBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1936 {
            return Some(BrownBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1926 {
            return Some(BrownBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1935 {
            return Some(BrownBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1931 {
            return Some(BrownBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1923 {
            return Some(BrownBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        return None;
    }
}

