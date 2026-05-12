use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueBed {
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

impl BlockState for LightBlueBed {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Foot { return 1786; }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head { return 1789; }
        if self.r#facing == Facing::West && self.r#part == Part::Foot && self.r#occupied == true { return 1788; }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Head { return 1791; }
        if self.r#facing == Facing::South && self.r#occupied == true && self.r#part == Part::Head { return 1783; }
        if self.r#occupied == false && self.r#part == Part::Head && self.r#facing == Facing::South { return 1785; }
        if self.r#facing == Facing::West && self.r#part == Part::Head && self.r#occupied == true { return 1787; }
        if self.r#facing == Facing::East && self.r#part == Part::Foot && self.r#occupied == false { return 1794; }
        if self.r#part == Part::Foot && self.r#facing == Facing::North && self.r#occupied == false { return 1782; }
        if self.r#part == Part::Foot && self.r#occupied == true && self.r#facing == Facing::South { return 1784; }
        if self.r#part == Part::Head && self.r#facing == Facing::East && self.r#occupied == false { return 1793; }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == false { return 1790; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::East { return 1792; }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::North { return 1779; }
        if self.r#part == Part::Head && self.r#facing == Facing::North && self.r#occupied == false { return 1781; }
        if self.r#facing == Facing::North && self.r#part == Part::Foot && self.r#occupied == true { return 1780; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1786 {
            return Some(LightBlueBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1789 {
            return Some(LightBlueBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1788 {
            return Some(LightBlueBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1791 {
            return Some(LightBlueBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1783 {
            return Some(LightBlueBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1785 {
            return Some(LightBlueBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1787 {
            return Some(LightBlueBed {
                r#facing: Facing::West,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1794 {
            return Some(LightBlueBed {
                r#facing: Facing::East,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1782 {
            return Some(LightBlueBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1784 {
            return Some(LightBlueBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 1793 {
            return Some(LightBlueBed {
                r#part: Part::Head,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1790 {
            return Some(LightBlueBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1792 {
            return Some(LightBlueBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1779 {
            return Some(LightBlueBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1781 {
            return Some(LightBlueBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1780 {
            return Some(LightBlueBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        return None;
    }
}

