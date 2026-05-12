use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateBrickStairs {
    pub waterlogged: bool,
    pub r#shape: Shape,
    pub r#facing: Facing,
    pub r#half: Half,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for DeepslateBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 29035; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == true { return 28992; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 28996; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 28967; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 28978; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 28963; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West { return 28999; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 29012; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 29034; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 29009; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 28972; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 28971; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 29005; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 29011; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 28989; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 29017; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 29019; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 28960; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 29014; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 29022; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 29024; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 29025; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 29030; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 29000; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 28969; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South { return 28984; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 28958; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 28966; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::South { return 28980; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 29013; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 28976; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 28994; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 29015; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 28982; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 28997; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South { return 28981; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North { return 28974; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::West { return 29004; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 28977; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 28995; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 28990; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 29002; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 29001; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 28962; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 29026; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South { return 28983; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 29016; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 29031; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 29003; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 29008; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 29036; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Top { return 28959; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 28965; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 29010; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 28991; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 29028; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false { return 28979; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 29020; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 28964; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::West { return 29006; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 29021; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false { return 29027; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 28985; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South { return 28986; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::South { return 28987; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 28970; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 29007; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 28961; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East { return 29032; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top { return 29018; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 29023; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false { return 29029; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 29037; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 28993; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 28968; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 28973; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 28975; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East { return 29033; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South { return 28988; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West { return 28998; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29035 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 28992 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 28996 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 28967 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 28978 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 28963 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28999 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 29012 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 29034 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 29009 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 28972 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28971 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 29005 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 29011 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 28989 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 29017 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 29019 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 28960 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 29014 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 29022 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 29024 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 29025 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 29030 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 29000 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 28969 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 28984 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 28958 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 28966 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 28980 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 29013 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 28976 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28994 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 29015 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 28982 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28997 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28981 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 28974 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 29004 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 28977 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 28995 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28990 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 29002 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 29001 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 28962 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 29026 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28983 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 29016 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 29031 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 29003 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 29008 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 29036 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 28959 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 28965 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 29010 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28991 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 29028 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 28979 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 29020 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 28964 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 29006 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 29021 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 29027 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 28985 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28986 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 28987 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 28970 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 29007 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 28961 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 29032 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 29018 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 29023 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 29029 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 29037 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28993 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 28968 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 28973 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 28975 {
            return Some(DeepslateBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 29033 {
            return Some(DeepslateBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 28988 {
            return Some(DeepslateBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 28998 {
            return Some(DeepslateBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

