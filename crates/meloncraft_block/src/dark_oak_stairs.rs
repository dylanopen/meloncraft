use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakStairs {
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

impl BlockState for DarkOakStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top { return 11993; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == false { return 11969; }
        if self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#half == Half::Top { return 11971; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false { return 11991; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 11953; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 11970; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South { return 11955; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West { return 11985; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 12000; }
        if self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 11980; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 12001; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 11992; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 11986; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top { return 11954; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 11983; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 11957; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 11938; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 12002; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West { return 11975; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 11932; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 11981; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 11976; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 11990; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 11984; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 11951; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == true { return 11956; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 11963; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 11996; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#waterlogged == false { return 11933; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North { return 11945; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East { return 12006; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 11978; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 11977; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top { return 11973; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 11952; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 11961; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 11974; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == true { return 11958; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 11930; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Bottom { return 11966; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 11946; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 11944; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 12003; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::North { return 11939; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 12008; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 11936; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 11994; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 11987; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 11950; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 11988; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false { return 11989; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 12005; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 11979; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 11941; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 11948; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 11934; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 11960; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 11942; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 11982; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == true { return 11998; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 12004; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == false { return 11997; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 11999; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 11940; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 11962; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 11964; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 11965; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 11947; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top { return 11937; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#half == Half::Top { return 11935; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 12009; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 11995; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 11968; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South { return 11959; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false { return 11967; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#waterlogged == true { return 11972; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft { return 11943; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 11949; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 11931; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 12007; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11993 {
            return Some(DarkOakStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 11969 {
            return Some(DarkOakStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 11971 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 11991 {
            return Some(DarkOakStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11953 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 11970 {
            return Some(DarkOakStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 11955 {
            return Some(DarkOakStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 11985 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 12000 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 11980 {
            return Some(DarkOakStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12001 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11992 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 11986 {
            return Some(DarkOakStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11954 {
            return Some(DarkOakStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 11983 {
            return Some(DarkOakStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 11957 {
            return Some(DarkOakStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11938 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12002 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11975 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 11932 {
            return Some(DarkOakStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 11981 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11976 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11990 {
            return Some(DarkOakStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 11984 {
            return Some(DarkOakStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 11951 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11956 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 11963 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11996 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 11933 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 11945 {
            return Some(DarkOakStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 12006 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 11978 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 11977 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11973 {
            return Some(DarkOakStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 11952 {
            return Some(DarkOakStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 11961 {
            return Some(DarkOakStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11974 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 11958 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 11930 {
            return Some(DarkOakStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11966 {
            return Some(DarkOakStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11946 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 11944 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12003 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11939 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 12008 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 11936 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 11994 {
            return Some(DarkOakStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 11987 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 11950 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 11988 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11989 {
            return Some(DarkOakStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 12005 {
            return Some(DarkOakStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11979 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11941 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11948 {
            return Some(DarkOakStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11934 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11960 {
            return Some(DarkOakStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 11942 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 11982 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11998 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 12004 {
            return Some(DarkOakStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 11997 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11999 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11940 {
            return Some(DarkOakStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 11962 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 11964 {
            return Some(DarkOakStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 11965 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 11947 {
            return Some(DarkOakStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 11937 {
            return Some(DarkOakStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 11935 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 12009 {
            return Some(DarkOakStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11995 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 11968 {
            return Some(DarkOakStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 11959 {
            return Some(DarkOakStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 11967 {
            return Some(DarkOakStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 11972 {
            return Some(DarkOakStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 11943 {
            return Some(DarkOakStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 11949 {
            return Some(DarkOakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 11931 {
            return Some(DarkOakStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12007 {
            return Some(DarkOakStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        return None;
    }
}

