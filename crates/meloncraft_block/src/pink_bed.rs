use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkBed {
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

impl BlockState for PinkBed {
    fn to_id(&self) -> i32 {
        if self.r#part == Part::Foot && self.r#facing == Facing::North && self.r#occupied == false { return 1830; }
        if self.r#part == Part::Head && self.r#facing == Facing::North && self.r#occupied == false { return 1829; }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::East { return 1839; }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Head { return 1841; }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::North { return 1827; }
        if self.r#part == Part::Foot && self.r#occupied == false && self.r#facing == Facing::East { return 1842; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::West { return 1838; }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head { return 1837; }
        if self.r#facing == Facing::South && self.r#occupied == true && self.r#part == Part::Foot { return 1832; }
        if self.r#part == Part::Head && self.r#facing == Facing::South && self.r#occupied == false { return 1833; }
        if self.r#occupied == true && self.r#facing == Facing::North && self.r#part == Part::Foot { return 1828; }
        if self.r#occupied == true && self.r#facing == Facing::South && self.r#part == Part::Head { return 1831; }
        if self.r#part == Part::Head && self.r#facing == Facing::West && self.r#occupied == true { return 1835; }
        if self.r#occupied == true && self.r#facing == Facing::East && self.r#part == Part::Foot { return 1840; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::West { return 1836; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::South { return 1834; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1830 {
            return Some(PinkBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1829 {
            return Some(PinkBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1839 {
            return Some(PinkBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1841 {
            return Some(PinkBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1827 {
            return Some(PinkBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1842 {
            return Some(PinkBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1838 {
            return Some(PinkBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1837 {
            return Some(PinkBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1832 {
            return Some(PinkBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1833 {
            return Some(PinkBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1828 {
            return Some(PinkBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1831 {
            return Some(PinkBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Head,
            });
        }
        if state_id == 1835 {
            return Some(PinkBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1840 {
            return Some(PinkBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Foot,
            });
        }
        if state_id == 1836 {
            return Some(PinkBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1834 {
            return Some(PinkBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

