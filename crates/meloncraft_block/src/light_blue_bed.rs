use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueBed {
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

impl BlockState for LightBlueBed {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1779; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::North { return 1780; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::West { return 1788; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::South && block_state.r#part == Part::Foot { return 1786; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1783; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1794; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::South { return 1785; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::North { return 1781; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == true && block_state.r#facing == Facing::East { return 1792; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1790; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::South && block_state.r#part == Part::Foot { return 1784; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::North && block_state.r#occupied == false { return 1782; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::West && block_state.r#part == Part::Head { return 1787; }
        if block_state.r#facing == Facing::East && block_state.r#part == Part::Head && block_state.r#occupied == true { return 1791; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1793; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1789; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1779 {
            return Some(LightBlueBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1780 {
            return Some(LightBlueBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1788 {
            return Some(LightBlueBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1786 {
            return Some(LightBlueBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1783 {
            return Some(LightBlueBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1794 {
            return Some(LightBlueBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1785 {
            return Some(LightBlueBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1781 {
            return Some(LightBlueBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1792 {
            return Some(LightBlueBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1790 {
            return Some(LightBlueBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1784 {
            return Some(LightBlueBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1782 {
            return Some(LightBlueBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1787 {
            return Some(LightBlueBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1791 {
            return Some(LightBlueBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1793 {
            return Some(LightBlueBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1789 {
            return Some(LightBlueBed {
                r#facing: Facing::West,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        return None;
    }
}

