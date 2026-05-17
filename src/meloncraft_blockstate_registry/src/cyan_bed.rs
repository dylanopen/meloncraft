use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanBed {
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

impl BlockState for CyanBed {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Head { return 1881; }
        if self.r#part == Part::Head && self.r#facing == Facing::North && self.r#occupied == true { return 1875; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::West { return 1884; }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head { return 1885; }
        if self.r#occupied == true && self.r#facing == Facing::East && self.r#part == Part::Foot { return 1888; }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::South { return 1879; }
        if self.r#facing == Facing::East && self.r#part == Part::Head && self.r#occupied == false { return 1889; }
        if self.r#facing == Facing::North && self.r#occupied == false && self.r#part == Part::Foot { return 1878; }
        if self.r#occupied == true && self.r#facing == Facing::North && self.r#part == Part::Foot { return 1876; }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Foot { return 1882; }
        if self.r#facing == Facing::West && self.r#part == Part::Foot && self.r#occupied == false { return 1886; }
        if self.r#occupied == false && self.r#part == Part::Head && self.r#facing == Facing::North { return 1877; }
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::East { return 1887; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::East { return 1890; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::South { return 1880; }
        if self.r#facing == Facing::West && self.r#occupied == true && self.r#part == Part::Head { return 1883; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1881 {
            return Some(CyanBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1875 {
            return Some(CyanBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1884 {
            return Some(CyanBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1885 {
            return Some(CyanBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1888 {
            return Some(CyanBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Foot,
            });
        }
        if state_id == 1879 {
            return Some(CyanBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1889 {
            return Some(CyanBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1878 {
            return Some(CyanBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1876 {
            return Some(CyanBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1882 {
            return Some(CyanBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1886 {
            return Some(CyanBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1877 {
            return Some(CyanBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1887 {
            return Some(CyanBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1890 {
            return Some(CyanBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1880 {
            return Some(CyanBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1883 {
            return Some(CyanBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        return None;
    }
}

