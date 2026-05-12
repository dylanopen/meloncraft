use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCutCopperStairs {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#shape: Shape,
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

impl BlockState for ExposedCutCopperStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 25302; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 25341; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South { return 25319; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 25332; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true { return 25291; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 25286; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 25299; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 25347; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false { return 25330; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false { return 25312; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 25294; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 25342; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25363; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 25364; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 25301; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 25326; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West { return 25331; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true { return 25351; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 25297; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 25358; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 25346; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 25356; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top { return 25288; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 25362; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false { return 25300; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 25316; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 25318; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Top { return 25313; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South { return 25314; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 25334; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 25293; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == true { return 25349; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25361; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 25317; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 25295; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 25309; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 25339; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight { return 25343; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false { return 25290; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 25348; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true { return 25287; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 25336; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25315; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 25320; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25321; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 25327; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top { return 25311; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 25340; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 25296; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 25308; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 25322; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 25338; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true { return 25345; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East { return 25350; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 25360; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::South { return 25310; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25355; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 25344; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 25292; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 25305; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 25303; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 25324; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 25329; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 25298; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 25304; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::North { return 25289; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 25306; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 25325; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true { return 25333; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 25359; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 25353; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South { return 25307; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 25335; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 25328; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 25285; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == false { return 25352; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 25357; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 25354; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 25337; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 25323; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25302 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25341 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25319 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 25332 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25291 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25286 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 25299 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25347 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25330 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25312 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 25294 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25342 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25363 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25364 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25301 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 25326 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25331 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 25351 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25297 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25358 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25346 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25356 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25288 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25362 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25300 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25316 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25318 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25313 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 25314 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 25334 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25293 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25349 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25361 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25317 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25295 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 25309 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25339 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25343 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25290 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25348 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25287 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 25336 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25315 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25320 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25321 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25327 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25311 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25340 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25296 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25308 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25322 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25338 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25345 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25350 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 25360 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25310 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25355 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25344 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25292 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25305 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 25303 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25324 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25329 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25298 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25304 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 25289 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 25306 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25325 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25333 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25359 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25353 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25307 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 25335 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 25328 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25285 {
            return Some(ExposedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25352 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25357 {
            return Some(ExposedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25354 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25337 {
            return Some(ExposedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25323 {
            return Some(ExposedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

