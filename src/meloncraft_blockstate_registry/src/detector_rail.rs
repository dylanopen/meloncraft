use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectorRail {
    pub powered: bool,
    pub r#shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
}

impl BlockState for DetectorRail {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::AscendingNorth
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2031;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#shape == Shape::AscendingSouth
        {
            return 2034;
        }
        if self.r#waterlogged == true && self.r#powered == true && self.r#shape == Shape::NorthSouth
        {
            return 2011;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#shape == Shape::AscendingEast
        {
            return 2016;
        }
        if self.r#shape == Shape::EastWest && self.r#waterlogged == false && self.r#powered == true
        {
            return 2014;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#shape == Shape::AscendingSouth
        {
            return 2021;
        }
        if self.r#powered == true
            && self.r#shape == Shape::AscendingWest
            && self.r#waterlogged == false
        {
            return 2018;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#shape == Shape::NorthSouth
        {
            return 2024;
        }
        if self.r#powered == false
            && self.r#shape == Shape::AscendingSouth
            && self.r#waterlogged == true
        {
            return 2033;
        }
        if self.r#shape == Shape::AscendingWest
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2017;
        }
        if self.r#shape == Shape::AscendingWest
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2030;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::AscendingNorth
            && self.r#powered == false
        {
            return 2032;
        }
        if self.r#shape == Shape::NorthSouth
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 2012;
        }
        if self.r#waterlogged == true && self.r#powered == true && self.r#shape == Shape::EastWest {
            return 2013;
        }
        if self.r#waterlogged == true && self.r#powered == false && self.r#shape == Shape::EastWest
        {
            return 2025;
        }
        if self.r#shape == Shape::AscendingEast
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2027;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::AscendingEast
            && self.r#powered == true
        {
            return 2015;
        }
        if self.r#shape == Shape::AscendingSouth
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 2022;
        }
        if self.r#powered == false
            && self.r#shape == Shape::NorthSouth
            && self.r#waterlogged == true
        {
            return 2023;
        }
        if self.r#powered == false
            && self.r#shape == Shape::AscendingWest
            && self.r#waterlogged == true
        {
            return 2029;
        }
        if self.r#powered == false && self.r#shape == Shape::EastWest && self.r#waterlogged == false
        {
            return 2026;
        }
        if self.r#shape == Shape::AscendingNorth
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 2020;
        }
        if self.r#shape == Shape::AscendingNorth
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2019;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::AscendingEast
            && self.r#powered == false
        {
            return 2028;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2031 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2034 {
            return Some(DetectorRail {
                r#powered: false,
                r#waterlogged: false,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 2011 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#powered: true,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 2016 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::AscendingEast,
            });
        }
        if state_id == 2014 {
            return Some(DetectorRail {
                r#shape: Shape::EastWest,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2021 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#powered: true,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 2018 {
            return Some(DetectorRail {
                r#powered: true,
                r#shape: Shape::AscendingWest,
                r#waterlogged: false,
            });
        }
        if state_id == 2024 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 2033 {
            return Some(DetectorRail {
                r#powered: false,
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 2017 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingWest,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2030 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingWest,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2032 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#shape: Shape::AscendingNorth,
                r#powered: false,
            });
        }
        if state_id == 2012 {
            return Some(DetectorRail {
                r#shape: Shape::NorthSouth,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2013 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#powered: true,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 2025 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#powered: false,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 2027 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2015 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingEast,
                r#powered: true,
            });
        }
        if state_id == 2022 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingSouth,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2023 {
            return Some(DetectorRail {
                r#powered: false,
                r#shape: Shape::NorthSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 2029 {
            return Some(DetectorRail {
                r#powered: false,
                r#shape: Shape::AscendingWest,
                r#waterlogged: true,
            });
        }
        if state_id == 2026 {
            return Some(DetectorRail {
                r#powered: false,
                r#shape: Shape::EastWest,
                r#waterlogged: false,
            });
        }
        if state_id == 2020 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingNorth,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2019 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2028 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#shape: Shape::AscendingEast,
                r#powered: false,
            });
        }
        return None;
    }
}
