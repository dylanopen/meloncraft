use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedNetherBrickStairs {
    pub r#shape: Shape,
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#facing: Facing,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for RedNetherBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 16003; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 16051; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 16049; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 16010; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 16018; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 16037; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 15986; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 16032; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Top { return 15995; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 16009; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 16030; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft { return 15976; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South { return 15999; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 16022; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 16035; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft { return 16047; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 15987; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 16045; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 16002; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 15990; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 16015; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 16000; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false { return 16023; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 15982; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 15997; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 16011; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 16053; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false { return 15981; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South { return 16001; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::South { return 16012; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 15974; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 15975; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 15993; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 16036; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 16024; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 15994; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 16031; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft { return 16040; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 16007; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 16016; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15989; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 15992; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 15980; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 15984; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 16025; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 16005; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false { return 15983; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 15991; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 16027; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true { return 16052; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 16039; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 15988; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == true { return 16004; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 16008; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 15985; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 16013; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West { return 16026; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 16021; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 16046; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == true { return 16006; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 16042; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 16020; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true { return 16048; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft { return 16050; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 16043; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West { return 16019; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 16028; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 16017; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 15978; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 15979; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == true { return 16014; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 15977; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 15996; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 16044; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 15998; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 16033; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 16029; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 16038; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 16041; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 16034; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16003 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16051 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 16049 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 16010 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16018 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16037 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 15986 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16032 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 15995 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 16009 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16030 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15976 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15999 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 16022 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16035 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16047 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15987 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16045 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 16002 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15990 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16015 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 16000 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 16023 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15982 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15997 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16011 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 16053 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 15981 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 16001 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 16012 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15974 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15975 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15993 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16036 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 16024 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15994 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16031 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 16040 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16007 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16016 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 15989 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15992 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15980 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15984 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16025 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16005 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 15983 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15991 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16027 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 16052 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 16039 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15988 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 16004 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 16008 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15985 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16013 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16026 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 16021 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 16046 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16006 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 16042 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16020 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 16048 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16050 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16043 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 16019 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 16028 {
            return Some(RedNetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16017 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15978 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15979 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 16014 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15977 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15996 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 16044 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15998 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16033 {
            return Some(RedNetherBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16029 {
            return Some(RedNetherBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 16038 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16041 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16034 {
            return Some(RedNetherBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

