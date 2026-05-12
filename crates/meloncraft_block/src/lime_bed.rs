use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeBed {
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

impl BlockState for LimeBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1819; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1813; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::South && block_state.r#part == Part::Foot { return 1818; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::West { return 1822; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::South && block_state.r#part == Part::Head { return 1815; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1817; }
        if block_state.r#facing == Facing::North && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1814; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::North && block_state.r#part == Part::Head { return 1811; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::North && block_state.r#occupied == true { return 1812; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1821; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::East && block_state.r#occupied == true { return 1824; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::West { return 1820; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1823; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::East { return 1825; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::East { return 1826; }
        if block_state.r#facing == Facing::South && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1816; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1819 {
            return Some(LimeBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1813 {
            return Some(LimeBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1818 {
            return Some(LimeBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1822 {
            return Some(LimeBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1815 {
            return Some(LimeBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Head,
            });
        }
        if state_id == 1817 {
            return Some(LimeBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1814 {
            return Some(LimeBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1811 {
            return Some(LimeBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1812 {
            return Some(LimeBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1821 {
            return Some(LimeBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1824 {
            return Some(LimeBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: true,
            });
        }
        if state_id == 1820 {
            return Some(LimeBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1823 {
            return Some(LimeBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1825 {
            return Some(LimeBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1826 {
            return Some(LimeBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1816 {
            return Some(LimeBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        return None;
    }
}

