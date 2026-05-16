use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayBed {
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

impl BlockState for GrayBed {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Head { return 1857; }
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::West { return 1851; }
        if self.r#facing == Facing::South && self.r#part == Part::Head && self.r#occupied == false { return 1849; }
        if self.r#occupied == true && self.r#facing == Facing::South && self.r#part == Part::Head { return 1847; }
        if self.r#part == Part::Head && self.r#facing == Facing::West && self.r#occupied == false { return 1853; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::West { return 1854; }
        if self.r#part == Part::Foot && self.r#occupied == false && self.r#facing == Facing::East { return 1858; }
        if self.r#facing == Facing::North && self.r#part == Part::Head && self.r#occupied == false { return 1845; }
        if self.r#facing == Facing::East && self.r#part == Part::Head && self.r#occupied == true { return 1855; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::South { return 1850; }
        if self.r#part == Part::Foot && self.r#facing == Facing::East && self.r#occupied == true { return 1856; }
        if self.r#facing == Facing::South && self.r#occupied == true && self.r#part == Part::Foot { return 1848; }
        if self.r#facing == Facing::North && self.r#part == Part::Foot && self.r#occupied == true { return 1844; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::North { return 1846; }
        if self.r#occupied == true && self.r#facing == Facing::West && self.r#part == Part::Foot { return 1852; }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::North { return 1843; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1857 {
            return Some(GrayBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1851 {
            return Some(GrayBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 1849 {
            return Some(GrayBed {
                r#facing: Facing::South,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1847 {
            return Some(GrayBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Head,
            });
        }
        if state_id == 1853 {
            return Some(GrayBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1854 {
            return Some(GrayBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1858 {
            return Some(GrayBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1845 {
            return Some(GrayBed {
                r#facing: Facing::North,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1855 {
            return Some(GrayBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1850 {
            return Some(GrayBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1856 {
            return Some(GrayBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: true,
            });
        }
        if state_id == 1848 {
            return Some(GrayBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1844 {
            return Some(GrayBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1846 {
            return Some(GrayBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1852 {
            return Some(GrayBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        if state_id == 1843 {
            return Some(GrayBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

