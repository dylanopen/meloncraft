use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakTrapdoor {
    pub powered: bool,
    pub open: bool,
    pub r#half: Half,
    pub r#facing: Facing,
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

impl BlockState for OakTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 6913; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 6923; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 6939; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 6934; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6916; }
        if block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::West { return 6957; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::East { return 6963; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true { return 6953; }
        if block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 6919; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true { return 6947; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 6936; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == true { return 6938; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false { return 6951; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 6966; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false { return 6968; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 6971; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true { return 6973; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 6975; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 6921; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 6932; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 6931; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 6935; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == false { return 6946; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false { return 6928; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 6933; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 6949; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true { return 6954; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true { return 6925; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == false { return 6950; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true { return 6959; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true { return 6961; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == false { return 6976; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 6924; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 6915; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 6926; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 6972; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 6974; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#open == true { return 6929; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false { return 6930; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 6955; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 6965; }
        if block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == false { return 6952; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::East { return 6962; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false { return 6943; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false { return 6940; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6956; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == true { return 6969; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 6944; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::South { return 6937; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#waterlogged == true { return 6917; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 6958; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 6964; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#open == true { return 6914; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Top { return 6945; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == false { return 6960; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 6967; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false { return 6918; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true { return 6927; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true { return 6970; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == true { return 6941; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#half == Half::Top { return 6948; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == false { return 6920; }
        if block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 6942; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == true { return 6922; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6913 {
            return Some(OakTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 6923 {
            return Some(OakTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 6939 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6934 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6916 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6957 {
            return Some(OakTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6963 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6953 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 6919 {
            return Some(OakTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 6947 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6936 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 6938 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 6951 {
            return Some(OakTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 6966 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6968 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6971 {
            return Some(OakTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 6973 {
            return Some(OakTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 6975 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6921 {
            return Some(OakTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 6932 {
            return Some(OakTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 6931 {
            return Some(OakTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6935 {
            return Some(OakTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 6946 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6928 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6933 {
            return Some(OakTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 6949 {
            return Some(OakTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6954 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 6925 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 6950 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 6959 {
            return Some(OakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6961 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 6976 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 6924 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 6915 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 6926 {
            return Some(OakTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 6972 {
            return Some(OakTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6974 {
            return Some(OakTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6929 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 6930 {
            return Some(OakTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6955 {
            return Some(OakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6965 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6952 {
            return Some(OakTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 6962 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6943 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6940 {
            return Some(OakTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6956 {
            return Some(OakTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6969 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 6944 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 6937 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6917 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6958 {
            return Some(OakTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 6964 {
            return Some(OakTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 6914 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 6945 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 6960 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6967 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 6918 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 6927 {
            return Some(OakTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6970 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 6941 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 6948 {
            return Some(OakTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 6920 {
            return Some(OakTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6942 {
            return Some(OakTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6922 {
            return Some(OakTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        return None;
    }
}

