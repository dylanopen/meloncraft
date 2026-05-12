use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueBed {
    pub r#part: Part,
    pub occupied: bool,
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

impl BlockState for BlueBed {
    fn to_id(&self) -> i32 {
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::North { return 1910; }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Head { return 1913; }
        if self.r#facing == Facing::East && self.r#part == Part::Head && self.r#occupied == false { return 1921; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::East { return 1920; }
        if self.r#occupied == true && self.r#facing == Facing::West && self.r#part == Part::Foot { return 1916; }
        if self.r#facing == Facing::North && self.r#part == Part::Head && self.r#occupied == true { return 1907; }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::South { return 1911; }
        if self.r#occupied == false && self.r#facing == Facing::West && self.r#part == Part::Foot { return 1918; }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Foot { return 1914; }
        if self.r#occupied == true && self.r#facing == Facing::West && self.r#part == Part::Head { return 1915; }
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::East { return 1919; }
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Foot { return 1908; }
        if self.r#facing == Facing::North && self.r#occupied == false && self.r#part == Part::Head { return 1909; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::East { return 1922; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::South { return 1912; }
        if self.r#occupied == false && self.r#part == Part::Head && self.r#facing == Facing::West { return 1917; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1910 {
            return Some(BlueBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1913 {
            return Some(BlueBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1921 {
            return Some(BlueBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1920 {
            return Some(BlueBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1916 {
            return Some(BlueBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        if state_id == 1907 {
            return Some(BlueBed {
                r#facing: Facing::North,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1911 {
            return Some(BlueBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1918 {
            return Some(BlueBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        if state_id == 1914 {
            return Some(BlueBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1915 {
            return Some(BlueBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1919 {
            return Some(BlueBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1908 {
            return Some(BlueBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1909 {
            return Some(BlueBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1922 {
            return Some(BlueBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1912 {
            return Some(BlueBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1917 {
            return Some(BlueBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

