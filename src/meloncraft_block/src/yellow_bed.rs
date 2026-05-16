use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowBed {
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

impl BlockState for YellowBed {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head { return 1805; }
        if self.r#occupied == false && self.r#facing == Facing::South && self.r#part == Part::Foot { return 1802; }
        if self.r#facing == Facing::South && self.r#part == Part::Foot && self.r#occupied == true { return 1800; }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == true { return 1804; }
        if self.r#part == Part::Head && self.r#occupied == false && self.r#facing == Facing::East { return 1809; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::North { return 1798; }
        if self.r#occupied == true && self.r#facing == Facing::North && self.r#part == Part::Foot { return 1796; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::East { return 1808; }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Foot { return 1810; }
        if self.r#part == Part::Head && self.r#occupied == false && self.r#facing == Facing::South { return 1801; }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == false { return 1806; }
        if self.r#part == Part::Head && self.r#facing == Facing::South && self.r#occupied == true { return 1799; }
        if self.r#facing == Facing::West && self.r#part == Part::Head && self.r#occupied == true { return 1803; }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Head { return 1807; }
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Head { return 1795; }
        if self.r#part == Part::Head && self.r#occupied == false && self.r#facing == Facing::North { return 1797; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1805 {
            return Some(YellowBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1802 {
            return Some(YellowBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1800 {
            return Some(YellowBed {
                r#facing: Facing::South,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1804 {
            return Some(YellowBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1809 {
            return Some(YellowBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1798 {
            return Some(YellowBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1796 {
            return Some(YellowBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1808 {
            return Some(YellowBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1810 {
            return Some(YellowBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1801 {
            return Some(YellowBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1806 {
            return Some(YellowBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1799 {
            return Some(YellowBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1803 {
            return Some(YellowBed {
                r#facing: Facing::West,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1807 {
            return Some(YellowBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1795 {
            return Some(YellowBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1797 {
            return Some(YellowBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

