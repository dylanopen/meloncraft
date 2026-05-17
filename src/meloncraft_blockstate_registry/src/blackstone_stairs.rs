use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackstoneStairs {
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#shape: Shape,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
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

impl BlockState for BlackstoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 21636; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 21656; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 21643; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == true { return 21694; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 21661; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 21701; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 21644; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::South { return 21654; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 21682; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 21680; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 21698; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 21693; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 21665; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true { return 21686; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 21670; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft { return 21633; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 21678; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 21666; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South { return 21664; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 21709; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false { return 21677; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 21672; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 21642; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 21646; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 21691; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 21648; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 21662; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 21683; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::North { return 21631; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 21676; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false { return 21679; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 21647; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::East { return 21692; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::East { return 21695; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Top { return 21699; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 21702; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 21703; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false { return 21635; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North { return 21638; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom { return 21684; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 21651; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East { return 21707; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false { return 21655; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 21690; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 21645; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 21696; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 21675; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 21697; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 21706; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::East { return 21708; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top { return 21653; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 21669; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 21685; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true { return 21668; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 21705; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 21659; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 21671; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::South { return 21657; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 21700; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 21634; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 21667; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 21687; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true { return 21660; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 21641; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West { return 21673; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 21637; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true { return 21658; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Top { return 21674; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 21704; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 21650; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 21630; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 21681; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 21688; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 21640; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 21632; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top { return 21639; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 21652; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 21649; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 21689; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 21663; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21636 {
            return Some(BlackstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21656 {
            return Some(BlackstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21643 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 21694 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 21661 {
            return Some(BlackstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21701 {
            return Some(BlackstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21644 {
            return Some(BlackstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 21654 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 21682 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21680 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21698 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21693 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 21665 {
            return Some(BlackstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21686 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21670 {
            return Some(BlackstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 21633 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21678 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21666 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 21664 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 21709 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21677 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21672 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21642 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 21646 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21691 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 21648 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21662 {
            return Some(BlackstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21683 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21631 {
            return Some(BlackstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21676 {
            return Some(BlackstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 21679 {
            return Some(BlackstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 21647 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21692 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21695 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21699 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 21702 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21703 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21635 {
            return Some(BlackstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 21638 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 21684 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21651 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 21707 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 21655 {
            return Some(BlackstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 21690 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21645 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21696 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 21675 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21697 {
            return Some(BlackstoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21706 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21708 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21653 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21669 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21685 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21668 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21705 {
            return Some(BlackstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21659 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 21671 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21657 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 21700 {
            return Some(BlackstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21634 {
            return Some(BlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21667 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21687 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21660 {
            return Some(BlackstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21641 {
            return Some(BlackstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 21673 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21637 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21658 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 21674 {
            return Some(BlackstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21704 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21650 {
            return Some(BlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 21630 {
            return Some(BlackstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 21681 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21688 {
            return Some(BlackstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21640 {
            return Some(BlackstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 21632 {
            return Some(BlackstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21639 {
            return Some(BlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21652 {
            return Some(BlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 21649 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 21689 {
            return Some(BlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21663 {
            return Some(BlackstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

