use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCutCopperStairs {
    pub r#facing: Facing,
    pub r#shape: Shape,
    pub waterlogged: bool,
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
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
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for WaxedWeatheredCutCopperStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 25559; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom { return 25614; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == true { return 25577; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Top { return 25581; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 25631; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West { return 25602; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25629; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 25590; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == true { return 25597; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25613; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 25596; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 25616; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 25628; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::East { return 25620; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 25564; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25635; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25567; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top { return 25566; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 25610; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 25563; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South { return 25585; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 25624; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#half == Half::Top { return 25561; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 25621; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 25575; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 25626; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 25569; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 25579; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 25599; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25589; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 25586; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 25588; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South { return 25593; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East { return 25625; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 25632; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 25592; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 25571; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South { return 25591; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West { return 25611; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 25619; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 25558; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 25587; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 25562; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 25608; }
        if self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 25568; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North { return 25565; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25573; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top { return 25582; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 25598; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 25560; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true { return 25601; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 25578; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 25595; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 25570; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::West { return 25600; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#waterlogged == true { return 25609; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25615; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == true { return 25617; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 25623; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 25572; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 25630; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East { return 25636; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 25634; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top { return 25604; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 25557; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 25612; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25627; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 25607; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 25583; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 25574; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 25580; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 25605; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 25576; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 25622; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 25594; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 25603; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 25633; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 25584; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 25618; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight { return 25606; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25559 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25614 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25577 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 25581 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25631 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25602 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25629 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25590 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25597 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25613 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25596 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25616 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 25628 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25620 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 25564 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25635 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25567 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25566 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25610 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 25563 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25585 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 25624 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25561 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 25621 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25575 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25626 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25569 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25579 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25599 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25589 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25586 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25588 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 25593 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 25625 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25632 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25592 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25571 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25591 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25611 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 25619 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 25558 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25587 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25562 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25608 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25568 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25565 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 25573 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25582 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25598 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 25560 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 25601 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25578 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25595 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 25570 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 25600 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25609 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25615 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25617 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25623 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25572 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25630 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25636 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25634 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25604 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25557 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25612 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25627 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25607 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25583 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25574 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25580 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25605 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25576 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25622 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25594 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25603 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25633 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25584 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25618 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 25606 {
            return Some(WaxedWeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        return None;
    }
}

