use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayBed {
    pub r#facing: Facing,
    pub r#part: Part,
    pub occupied: bool,
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

impl BlockState for GrayBed {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1850; }
        if block_state.r#facing == Facing::East && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1857; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::East && block_state.r#occupied == false { return 1858; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1856; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::South { return 1849; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::South && block_state.r#part == Part::Foot { return 1848; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::West && block_state.r#occupied == true { return 1851; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1852; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1847; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1845; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::North { return 1843; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::West && block_state.r#part == Part::Head { return 1853; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1855; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1844; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::West && block_state.r#occupied == false { return 1854; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Foot { return 1846; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1850 {
            return Some(GrayBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1857 {
            return Some(GrayBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1858 {
            return Some(GrayBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1856 {
            return Some(GrayBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1849 {
            return Some(GrayBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1848 {
            return Some(GrayBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1851 {
            return Some(GrayBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1852 {
            return Some(GrayBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1847 {
            return Some(GrayBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1845 {
            return Some(GrayBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1843 {
            return Some(GrayBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1853 {
            return Some(GrayBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1855 {
            return Some(GrayBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1844 {
            return Some(GrayBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1854 {
            return Some(GrayBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1846 {
            return Some(GrayBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        return None;
    }
}

