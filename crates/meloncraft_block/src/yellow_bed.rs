use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowBed {
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

impl BlockState for YellowBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == true && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1795; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1798; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::South && block_state.r#occupied == false { return 1801; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1807; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Foot && block_state.r#occupied == true { return 1804; }
        if block_state.r#facing == Facing::South && block_state.r#part == Part::Foot && block_state.r#occupied == true { return 1800; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1805; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1799; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::East && block_state.r#occupied == false { return 1809; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == false && block_state.r#part == Part::Foot { return 1810; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1806; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1808; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::West && block_state.r#occupied == true { return 1803; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1797; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == false && block_state.r#facing == Facing::South { return 1802; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::North { return 1796; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1795 {
            return Some(YellowBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1798 {
            return Some(YellowBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1801 {
            return Some(YellowBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1807 {
            return Some(YellowBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1804 {
            return Some(YellowBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1800 {
            return Some(YellowBed {
                r#facing: Facing::South,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1805 {
            return Some(YellowBed {
                r#facing: Facing::West,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1799 {
            return Some(YellowBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1809 {
            return Some(YellowBed {
                r#part: Part::Head,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1810 {
            return Some(YellowBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1806 {
            return Some(YellowBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1808 {
            return Some(YellowBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1803 {
            return Some(YellowBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1797 {
            return Some(YellowBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1802 {
            return Some(YellowBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1796 {
            return Some(YellowBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

