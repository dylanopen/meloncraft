use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectorRail {
    pub waterlogged: bool,
    pub powered: bool,
    pub r#shape: Shape,
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
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#shape == Shape::EastWest { return 2026; }
        if block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == false && block_state.r#powered == true { return 2022; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#shape == Shape::AscendingEast { return 2016; }
        if block_state.r#powered == true && block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == false { return 2014; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == true { return 2015; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingNorth && block_state.r#powered == true { return 2019; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#shape == Shape::AscendingNorth { return 2020; }
        if block_state.r#shape == Shape::AscendingWest && block_state.r#powered == true && block_state.r#waterlogged == false { return 2018; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == true { return 2021; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#shape == Shape::NorthSouth { return 2023; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingEast && block_state.r#powered == false { return 2027; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#shape == Shape::AscendingNorth { return 2031; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#shape == Shape::EastWest { return 2013; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == true { return 2033; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::AscendingSouth && block_state.r#powered == false { return 2034; }
        if block_state.r#powered == true && block_state.r#shape == Shape::NorthSouth && block_state.r#waterlogged == true { return 2011; }
        if block_state.r#shape == Shape::AscendingWest && block_state.r#powered == false && block_state.r#waterlogged == false { return 2030; }
        if block_state.r#shape == Shape::AscendingNorth && block_state.r#powered == false && block_state.r#waterlogged == false { return 2032; }
        if block_state.r#shape == Shape::NorthSouth && block_state.r#powered == true && block_state.r#waterlogged == false { return 2012; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingWest && block_state.r#waterlogged == true { return 2029; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#shape == Shape::EastWest { return 2025; }
        if block_state.r#shape == Shape::AscendingWest && block_state.r#waterlogged == true && block_state.r#powered == true { return 2017; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#shape == Shape::NorthSouth { return 2024; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#shape == Shape::AscendingEast { return 2028; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2026 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 2022 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingSouth,
                r#waterlogged: false,
                r#powered: true,
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
                r#powered: true,
                r#shape: Shape::EastWest,
                r#waterlogged: false,
            });
        }
        if state_id == 2015 {
            return Some(DetectorRail {
                r#powered: true,
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
            });
        }
        if state_id == 2019 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingNorth,
                r#powered: true,
            });
        }
        if state_id == 2020 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::AscendingNorth,
            });
        }
        if state_id == 2018 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingWest,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2021 {
            return Some(DetectorRail {
                r#powered: true,
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 2023 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#powered: false,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 2027 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingEast,
                r#powered: false,
            });
        }
        if state_id == 2031 {
            return Some(DetectorRail {
                r#waterlogged: true,
                r#powered: false,
                r#shape: Shape::AscendingNorth,
            });
        }
        if state_id == 2013 {
            return Some(DetectorRail {
                r#powered: true,
                r#waterlogged: true,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 2033 {
            return Some(DetectorRail {
                r#powered: false,
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 2034 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#shape: Shape::AscendingSouth,
                r#powered: false,
            });
        }
        if state_id == 2011 {
            return Some(DetectorRail {
                r#powered: true,
                r#shape: Shape::NorthSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 2030 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingWest,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2032 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingNorth,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2012 {
            return Some(DetectorRail {
                r#shape: Shape::NorthSouth,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2029 {
            return Some(DetectorRail {
                r#powered: false,
                r#shape: Shape::AscendingWest,
                r#waterlogged: true,
            });
        }
        if state_id == 2025 {
            return Some(DetectorRail {
                r#powered: false,
                r#waterlogged: true,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 2017 {
            return Some(DetectorRail {
                r#shape: Shape::AscendingWest,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2024 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 2028 {
            return Some(DetectorRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::AscendingEast,
            });
        }
        return None;
    }
}

