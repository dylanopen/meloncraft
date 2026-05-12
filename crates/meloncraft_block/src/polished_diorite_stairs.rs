use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDioriteStairs {
    pub r#half: Half,
    pub r#shape: Shape,
    pub waterlogged: bool,
    pub r#facing: Facing,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PolishedDioriteStairs {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South { return 15371; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 15401; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 15342; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::West { return 15375; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 15391; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 15409; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true { return 15338; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 15384; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15410; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15343; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East { return 15413; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 15356; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 15368; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15405; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 15353; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 15361; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == true { return 15408; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West { return 15380; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 15340; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 15366; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 15399; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 15411; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 15381; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top { return 15363; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 15394; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 15345; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 15339; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15385; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 15392; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 15352; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 15370; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 15350; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Top { return 15376; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 15341; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 15395; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 15396; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 15369; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 15379; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 15398; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 15337; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 15374; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 15404; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 15412; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 15351; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 15367; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 15358; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West { return 15386; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 15403; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 15347; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false { return 15357; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 15362; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 15359; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 15389; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 15378; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15372; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 15393; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::North { return 15334; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15365; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false { return 15349; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 15383; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 15388; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#half == Half::Top { return 15382; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 15354; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 15377; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15390; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == false { return 15355; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::North { return 15348; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top { return 15335; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 15387; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 15344; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 15364; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true { return 15360; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 15336; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft { return 15407; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 15397; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft { return 15406; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 15373; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15402; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top { return 15400; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15346; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15371 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 15401 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15342 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15375 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 15391 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15409 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15338 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15384 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15410 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15343 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15413 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15356 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15368 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15405 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15353 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15361 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15408 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15380 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 15340 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15366 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15399 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 15411 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15381 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15363 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15394 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15345 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15339 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15385 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15392 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15352 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15370 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 15350 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15376 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15341 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15395 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15396 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15369 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15379 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15398 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15337 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15374 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15404 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15412 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15351 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15367 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15358 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15386 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15403 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15347 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15357 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15362 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15359 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15389 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15378 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15372 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15393 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15334 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15365 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15349 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15383 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15388 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15382 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15354 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15377 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15390 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15355 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15348 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15335 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15387 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15344 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 15364 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15360 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15336 {
            return Some(PolishedDioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15407 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15397 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 15406 {
            return Some(PolishedDioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15373 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15402 {
            return Some(PolishedDioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15400 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15346 {
            return Some(PolishedDioriteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

