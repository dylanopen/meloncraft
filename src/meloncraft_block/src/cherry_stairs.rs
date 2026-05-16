use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryStairs {
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

impl BlockState for CherryStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 11879; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 11906; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 11850; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 11855; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == true { return 11876; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true { return 11874; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == false { return 11889; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West { return 11909; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 11871; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 11878; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 11884; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 11898; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 11901; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == false { return 11917; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 11888; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 11852; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight { return 11899; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 11907; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 11864; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top { return 11853; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 11894; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 11902; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 11913; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 11914; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 11925; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::East { return 11929; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East { return 11916; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top { return 11875; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 11883; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 11895; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 11851; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North { return 11868; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 11905; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false { return 11919; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 11922; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 11886; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 11927; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 11918; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 11921; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 11882; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North { return 11862; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 11858; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 11863; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 11866; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 11924; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 11893; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false { return 11861; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 11896; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 11865; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South { return 11877; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == false { return 11881; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 11915; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 11870; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 11860; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top { return 11910; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == false { return 11911; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 11912; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North { return 11856; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 11892; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 11923; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 11859; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 11926; }
        if self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 11890; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 11869; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::North { return 11854; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 11904; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 11867; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top { return 11872; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 11887; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 11873; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false { return 11891; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 11900; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 11880; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West { return 11897; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::South { return 11885; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 11903; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 11908; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 11857; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 11920; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 11928; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11879 {
            return Some(CherryStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11906 {
            return Some(CherryStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11850 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11855 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 11876 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 11874 {
            return Some(CherryStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 11889 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 11909 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 11871 {
            return Some(CherryStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11878 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11884 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11898 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 11901 {
            return Some(CherryStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11917 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11888 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11852 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 11899 {
            return Some(CherryStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11907 {
            return Some(CherryStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 11864 {
            return Some(CherryStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 11853 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 11894 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11902 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 11913 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11914 {
            return Some(CherryStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11925 {
            return Some(CherryStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11929 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 11916 {
            return Some(CherryStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 11875 {
            return Some(CherryStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 11883 {
            return Some(CherryStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11895 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 11851 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 11868 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 11905 {
            return Some(CherryStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11919 {
            return Some(CherryStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11922 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 11886 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11927 {
            return Some(CherryStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11918 {
            return Some(CherryStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11921 {
            return Some(CherryStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11882 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11862 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 11858 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 11863 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11866 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 11924 {
            return Some(CherryStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 11893 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11861 {
            return Some(CherryStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 11896 {
            return Some(CherryStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 11865 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 11877 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 11881 {
            return Some(CherryStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 11915 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11870 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 11860 {
            return Some(CherryStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 11910 {
            return Some(CherryStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 11911 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11912 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 11856 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 11892 {
            return Some(CherryStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 11923 {
            return Some(CherryStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11859 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11926 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 11890 {
            return Some(CherryStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 11869 {
            return Some(CherryStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11854 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 11904 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11867 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11872 {
            return Some(CherryStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 11887 {
            return Some(CherryStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11873 {
            return Some(CherryStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11891 {
            return Some(CherryStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 11900 {
            return Some(CherryStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 11880 {
            return Some(CherryStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 11897 {
            return Some(CherryStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 11885 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 11903 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 11908 {
            return Some(CherryStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 11857 {
            return Some(CherryStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 11920 {
            return Some(CherryStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11928 {
            return Some(CherryStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        return None;
    }
}

