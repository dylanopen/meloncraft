use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaStairs {
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

impl BlockState for AcaciaStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false { return 11823; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 11837; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 11798; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 11780; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == true { return 11788; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false { return 11791; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 11789; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true { return 11796; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 11776; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 11809; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 11825; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North { return 11785; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 11787; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South { return 11794; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 11799; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 11813; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 11817; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South { return 11790; }
        if self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 11820; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::West { return 11811; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 11810; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 11828; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 11842; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 11843; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 11845; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft { return 11773; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 11822; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 11772; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 11840; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 11841; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 11847; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false { return 11793; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 11806; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 11835; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 11833; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 11775; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom { return 11803; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 11808; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 11838; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 11844; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 11774; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 11848; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 11792; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 11771; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 11824; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 11829; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 11839; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::West { return 11816; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 11815; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 11832; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 11827; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 11781; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 11778; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 11800; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 11804; }
        if self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 11821; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 11831; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 11834; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 11770; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 11807; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true { return 11818; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 11849; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 11836; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom { return 11784; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 11779; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 11783; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 11830; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 11805; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 11786; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 11777; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 11797; }
        if self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 11801; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == true { return 11802; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West { return 11819; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 11826; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 11812; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 11814; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == false { return 11795; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 11846; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 11782; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11823 {
            return Some(AcaciaStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 11837 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 11798 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11780 {
            return Some(AcaciaStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11788 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 11791 {
            return Some(AcaciaStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 11789 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 11796 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 11776 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11809 {
            return Some(AcaciaStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 11825 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11785 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 11787 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11794 {
            return Some(AcaciaStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 11799 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 11813 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11817 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11790 {
            return Some(AcaciaStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 11820 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 11811 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 11810 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11828 {
            return Some(AcaciaStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11842 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 11843 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11845 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11773 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11822 {
            return Some(AcaciaStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11772 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 11840 {
            return Some(AcaciaStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11841 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11847 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 11793 {
            return Some(AcaciaStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 11806 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11835 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11833 {
            return Some(AcaciaStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 11775 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11803 {
            return Some(AcaciaStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11808 {
            return Some(AcaciaStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11838 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11844 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11774 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11848 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11792 {
            return Some(AcaciaStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 11771 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11824 {
            return Some(AcaciaStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 11829 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 11839 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11816 {
            return Some(AcaciaStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 11815 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11832 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 11827 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11781 {
            return Some(AcaciaStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 11778 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 11800 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11804 {
            return Some(AcaciaStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 11821 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11831 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11834 {
            return Some(AcaciaStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 11770 {
            return Some(AcaciaStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11807 {
            return Some(AcaciaStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11818 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 11849 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11836 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 11784 {
            return Some(AcaciaStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11779 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 11783 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11830 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 11805 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 11786 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 11777 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11797 {
            return Some(AcaciaStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 11801 {
            return Some(AcaciaStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11802 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 11819 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 11826 {
            return Some(AcaciaStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11812 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11814 {
            return Some(AcaciaStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 11795 {
            return Some(AcaciaStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 11846 {
            return Some(AcaciaStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11782 {
            return Some(AcaciaStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

