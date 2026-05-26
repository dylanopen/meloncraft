use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayBed {
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

impl BlockState for LightGrayBed {
    fn to_id(&self) -> i32 {
        if self.r#occupied == false && self.r#facing == Facing::North && self.r#part == Part::Foot {
            return 1862;
        }
        if self.r#occupied == true && self.r#facing == Facing::North && self.r#part == Part::Foot {
            return 1860;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::South && self.r#occupied == true {
            return 1864;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::South {
            return 1863;
        }
        if self.r#occupied == true && self.r#facing == Facing::East && self.r#part == Part::Head {
            return 1871;
        }
        if self.r#facing == Facing::East && self.r#part == Part::Head && self.r#occupied == false {
            return 1873;
        }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::East {
            return 1872;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::West {
            return 1867;
        }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head {
            return 1869;
        }
        if self.r#occupied == false && self.r#facing == Facing::West && self.r#part == Part::Foot {
            return 1870;
        }
        if self.r#occupied == false && self.r#facing == Facing::North && self.r#part == Part::Head {
            return 1861;
        }
        if self.r#part == Part::Head && self.r#facing == Facing::North && self.r#occupied == true {
            return 1859;
        }
        if self.r#part == Part::Foot && self.r#occupied == true && self.r#facing == Facing::West {
            return 1868;
        }
        if self.r#occupied == false && self.r#facing == Facing::East && self.r#part == Part::Foot {
            return 1874;
        }
        if self.r#part == Part::Foot && self.r#occupied == false && self.r#facing == Facing::South {
            return 1866;
        }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Head {
            return 1865;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1862 {
            return Some(LightGrayBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1860 {
            return Some(LightGrayBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1864 {
            return Some(LightGrayBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1863 {
            return Some(LightGrayBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1871 {
            return Some(LightGrayBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Head,
            });
        }
        if state_id == 1873 {
            return Some(LightGrayBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1872 {
            return Some(LightGrayBed {
                r#occupied: true,
                r#part: Part::Foot,
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
        if state_id == 1869 {
            return Some(LightGrayBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1870 {
            return Some(LightGrayBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        if state_id == 1861 {
            return Some(LightGrayBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1859 {
            return Some(LightGrayBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1868 {
            return Some(LightGrayBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 1874 {
            return Some(LightGrayBed {
                r#occupied: false,
                r#facing: Facing::East,
                r#part: Part::Foot,
            });
        }
        if state_id == 1866 {
            return Some(LightGrayBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1865 {
            return Some(LightGrayBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        return None;
    }
}
