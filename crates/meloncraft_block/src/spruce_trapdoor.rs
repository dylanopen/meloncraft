use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceTrapdoor {
    pub r#half: Half,
    pub r#facing: Facing,
    pub powered: bool,
    pub open: bool,
    pub waterlogged: bool,
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

impl BlockState for SpruceTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true { return 7033; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 6991; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 7028; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#open == false { return 7006; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false { return 7014; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 7019; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 7026; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true { return 6993; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 7032; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false { return 7039; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 7025; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true { return 6997; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Bottom { return 7003; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 6996; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 7011; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true { return 7004; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 6985; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 7015; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::South { return 6995; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false { return 7040; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false { return 7030; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == true { return 7009; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Top { return 6999; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false { return 6983; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true { return 7037; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 7029; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == false { return 7016; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true { return 6982; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false { return 7031; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 6979; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false { return 7000; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false { return 7034; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::North { return 6990; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true { return 6986; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 7022; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 7035; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 7036; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7024; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::South { return 7007; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#open == false { return 7038; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == true { return 6987; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#open == true { return 6977; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#powered == false { return 6984; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == true { return 6994; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#open == false { return 6992; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 7008; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false { return 7018; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true { return 6998; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 7001; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::South { return 7002; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 6988; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == true { return 7020; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == true { return 6980; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Bottom { return 7021; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false { return 7012; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 6978; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#open == false { return 7023; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 7005; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == true { return 6981; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7017; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 7027; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false { return 7013; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true { return 7010; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false { return 6989; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7033 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 6991 {
            return Some(SpruceTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7028 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 7006 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7014 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7019 {
            return Some(SpruceTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7026 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 6993 {
            return Some(SpruceTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7032 {
            return Some(SpruceTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7039 {
            return Some(SpruceTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7025 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 6997 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7003 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6996 {
            return Some(SpruceTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 7011 {
            return Some(SpruceTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7004 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 6985 {
            return Some(SpruceTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7015 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6995 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7040 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7030 {
            return Some(SpruceTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7009 {
            return Some(SpruceTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 6999 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 6983 {
            return Some(SpruceTrapdoor {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7037 {
            return Some(SpruceTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 7029 {
            return Some(SpruceTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 7016 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6982 {
            return Some(SpruceTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7031 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 6979 {
            return Some(SpruceTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7000 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7034 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6990 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6986 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7022 {
            return Some(SpruceTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7035 {
            return Some(SpruceTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7036 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7024 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7007 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7038 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 6987 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 6977 {
            return Some(SpruceTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 6984 {
            return Some(SpruceTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 6994 {
            return Some(SpruceTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 6992 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 7008 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7018 {
            return Some(SpruceTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6998 {
            return Some(SpruceTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7001 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 7002 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6988 {
            return Some(SpruceTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7020 {
            return Some(SpruceTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 6980 {
            return Some(SpruceTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7021 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7012 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 6978 {
            return Some(SpruceTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 7023 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 7005 {
            return Some(SpruceTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 6981 {
            return Some(SpruceTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7017 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7027 {
            return Some(SpruceTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7013 {
            return Some(SpruceTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7010 {
            return Some(SpruceTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 6989 {
            return Some(SpruceTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        return None;
    }
}

