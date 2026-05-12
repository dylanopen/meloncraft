use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatorRail {
    pub r#shape: Shape,
    pub powered: bool,
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

impl BlockState for ActivatorRail {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#powered == false && self.r#shape == Shape::AscendingWest { return 11225; }
        if self.r#shape == Shape::NorthSouth && self.r#waterlogged == false && self.r#powered == true { return 11207; }
        if self.r#shape == Shape::AscendingEast && self.r#powered == true && self.r#waterlogged == false { return 11211; }
        if self.r#shape == Shape::AscendingNorth && self.r#waterlogged == false && self.r#powered == false { return 11227; }
        if self.r#waterlogged == false && self.r#powered == false && self.r#shape == Shape::AscendingSouth { return 11229; }
        if self.r#shape == Shape::AscendingSouth && self.r#waterlogged == true && self.r#powered == false { return 11228; }
        if self.r#shape == Shape::NorthSouth && self.r#waterlogged == false && self.r#powered == false { return 11219; }
        if self.r#shape == Shape::NorthSouth && self.r#waterlogged == true && self.r#powered == true { return 11206; }
        if self.r#shape == Shape::AscendingNorth && self.r#waterlogged == true && self.r#powered == false { return 11226; }
        if self.r#shape == Shape::EastWest && self.r#waterlogged == true && self.r#powered == true { return 11208; }
        if self.r#waterlogged == true && self.r#shape == Shape::AscendingWest && self.r#powered == true { return 11212; }
        if self.r#shape == Shape::NorthSouth && self.r#powered == false && self.r#waterlogged == true { return 11218; }
        if self.r#waterlogged == false && self.r#shape == Shape::EastWest && self.r#powered == false { return 11221; }
        if self.r#powered == true && self.r#shape == Shape::AscendingWest && self.r#waterlogged == false { return 11213; }
        if self.r#powered == true && self.r#shape == Shape::AscendingNorth && self.r#waterlogged == false { return 11215; }
        if self.r#powered == true && self.r#shape == Shape::AscendingNorth && self.r#waterlogged == true { return 11214; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#shape == Shape::EastWest { return 11209; }
        if self.r#shape == Shape::AscendingEast && self.r#waterlogged == true && self.r#powered == true { return 11210; }
        if self.r#powered == true && self.r#shape == Shape::AscendingSouth && self.r#waterlogged == true { return 11216; }
        if self.r#shape == Shape::AscendingSouth && self.r#waterlogged == false && self.r#powered == true { return 11217; }
        if self.r#powered == false && self.r#shape == Shape::AscendingEast && self.r#waterlogged == false { return 11223; }
        if self.r#waterlogged == true && self.r#shape == Shape::EastWest && self.r#powered == false { return 11220; }
        if self.r#waterlogged == true && self.r#powered == false && self.r#shape == Shape::AscendingEast { return 11222; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#shape == Shape::AscendingWest { return 11224; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11225 {
            return Some(ActivatorRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::AscendingWest,
            });
        }
        if state_id == 11207 {
            return Some(ActivatorRail {
                r#shape: Shape::NorthSouth,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 11211 {
            return Some(ActivatorRail {
                r#shape: Shape::AscendingEast,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11227 {
            return Some(ActivatorRail {
                r#shape: Shape::AscendingNorth,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 11229 {
            return Some(ActivatorRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 11228 {
            return Some(ActivatorRail {
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 11219 {
            return Some(ActivatorRail {
                r#shape: Shape::NorthSouth,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 11206 {
            return Some(ActivatorRail {
                r#shape: Shape::NorthSouth,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 11226 {
            return Some(ActivatorRail {
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 11208 {
            return Some(ActivatorRail {
                r#shape: Shape::EastWest,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 11212 {
            return Some(ActivatorRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingWest,
                r#powered: true,
            });
        }
        if state_id == 11218 {
            return Some(ActivatorRail {
                r#shape: Shape::NorthSouth,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11221 {
            return Some(ActivatorRail {
                r#waterlogged: false,
                r#shape: Shape::EastWest,
                r#powered: false,
            });
        }
        if state_id == 11213 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::AscendingWest,
                r#waterlogged: false,
            });
        }
        if state_id == 11215 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::AscendingNorth,
                r#waterlogged: false,
            });
        }
        if state_id == 11214 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
            });
        }
        if state_id == 11209 {
            return Some(ActivatorRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 11210 {
            return Some(ActivatorRail {
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 11216 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 11217 {
            return Some(ActivatorRail {
                r#shape: Shape::AscendingSouth,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 11223 {
            return Some(ActivatorRail {
                r#powered: false,
                r#shape: Shape::AscendingEast,
                r#waterlogged: false,
            });
        }
        if state_id == 11220 {
            return Some(ActivatorRail {
                r#waterlogged: true,
                r#shape: Shape::EastWest,
                r#powered: false,
            });
        }
        if state_id == 11222 {
            return Some(ActivatorRail {
                r#waterlogged: true,
                r#powered: false,
                r#shape: Shape::AscendingEast,
            });
        }
        if state_id == 11224 {
            return Some(ActivatorRail {
                r#powered: false,
                r#waterlogged: true,
                r#shape: Shape::AscendingWest,
            });
        }
        return None;
    }
}

