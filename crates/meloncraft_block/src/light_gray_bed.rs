use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayBed {
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

impl BlockState for LightGrayBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1861; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::West { return 1868; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == false && block_state.r#facing == Facing::East { return 1874; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1867; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1863; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::West && block_state.r#part == Part::Head { return 1869; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::South && block_state.r#occupied == true { return 1864; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1866; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::North && block_state.r#occupied == true { return 1859; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == false && block_state.r#facing == Facing::West { return 1870; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::North && block_state.r#occupied == true { return 1860; }
        if block_state.r#facing == Facing::South && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1865; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1871; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1872; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::East && block_state.r#occupied == false { return 1873; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Foot { return 1862; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1861 {
            return Some(LightGrayBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1868 {
            return Some(LightGrayBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1874 {
            return Some(LightGrayBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1867 {
            return Some(LightGrayBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1863 {
            return Some(LightGrayBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1869 {
            return Some(LightGrayBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1864 {
            return Some(LightGrayBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1866 {
            return Some(LightGrayBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1859 {
            return Some(LightGrayBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1870 {
            return Some(LightGrayBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 1860 {
            return Some(LightGrayBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1865 {
            return Some(LightGrayBed {
                r#facing: Facing::South,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1871 {
            return Some(LightGrayBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1872 {
            return Some(LightGrayBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1873 {
            return Some(LightGrayBed {
                r#part: Part::Head,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1862 {
            return Some(LightGrayBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        return None;
    }
}

