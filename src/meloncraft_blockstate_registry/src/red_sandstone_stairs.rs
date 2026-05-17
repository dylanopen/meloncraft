use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedSandstoneStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#shape: Shape,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl BlockState for RedSandstoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 13112; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 13119; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 13097; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 13109; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 13077; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 13118; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 13092; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 13055; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 13048; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 13089; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top { return 13076; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North { return 13067; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 13087; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South { return 13086; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 13123; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 13094; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 13080; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 13126; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 13106; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom { return 13101; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 13058; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom { return 13078; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 13124; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::North { return 13050; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 13057; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 13068; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 13074; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 13088; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 13105; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == false { return 13079; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 13060; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false { return 13065; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 13066; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 13063; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 13053; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 13090; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 13091; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top { return 13116; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 13127; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 13059; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 13114; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South { return 13082; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 13070; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 13100; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North { return 13056; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false { return 13049; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::East { return 13121; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 13064; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 13108; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom { return 13081; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 13102; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false { return 13093; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top { return 13096; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top { return 13072; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North { return 13052; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 13122; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 13069; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 13104; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top { return 13073; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Top { return 13075; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 13103; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North { return 13051; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 13098; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 13125; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 13120; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::East { return 13115; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South { return 13071; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 13095; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 13113; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 13111; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 13062; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 13110; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East { return 13117; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 13099; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 13061; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 13107; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 13084; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false { return 13083; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 13054; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 13085; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13112 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 13119 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13097 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 13109 {
            return Some(RedSandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13077 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 13118 {
            return Some(RedSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13092 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 13055 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 13048 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13089 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 13076 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 13067 {
            return Some(RedSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 13087 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13086 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13123 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 13094 {
            return Some(RedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 13080 {
            return Some(RedSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 13126 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 13106 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13101 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13058 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13078 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13124 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 13050 {
            return Some(RedSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 13057 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 13068 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13074 {
            return Some(RedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 13088 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13105 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 13079 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 13060 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 13065 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 13066 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 13063 {
            return Some(RedSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 13053 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 13090 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13091 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 13116 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 13127 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13059 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13114 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 13082 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 13070 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13100 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13056 {
            return Some(RedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 13049 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13121 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13064 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 13108 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13081 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13102 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13093 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13096 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 13072 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 13052 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13122 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 13069 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13104 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13073 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 13075 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 13103 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 13051 {
            return Some(RedSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 13098 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13125 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 13120 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 13115 {
            return Some(RedSandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 13071 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13095 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 13113 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 13111 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 13062 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 13110 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13117 {
            return Some(RedSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 13099 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 13061 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13107 {
            return Some(RedSandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 13084 {
            return Some(RedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 13083 {
            return Some(RedSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13054 {
            return Some(RedSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 13085 {
            return Some(RedSandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

