use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakTrapdoor {
    pub open: bool,
    pub waterlogged: bool,
    pub powered: bool,
    pub r#half: Half,
    pub r#facing: Facing,
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
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#open == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#powered == true { return 6930; }
        if self.r#open == false && self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Top { return 6934; }
        if self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#powered == false { return 6935; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == false { return 6936; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#powered == true && self.r#open == true && self.r#facing == Facing::South { return 6938; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#open == false { return 6950; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#powered == false && self.r#open == false && self.r#waterlogged == true { return 6975; }
        if self.r#powered == false && self.r#half == Half::Bottom && self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::West { return 6955; }
        if self.r#powered == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#open == false && self.r#waterlogged == false { return 6974; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == true { return 6969; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#facing == Facing::North && self.r#open == true && self.r#waterlogged == true { return 6923; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#open == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 6968; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true { return 6939; }
        if self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Top && self.r#powered == false && self.r#facing == Facing::South { return 6932; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false { return 6943; }
        if self.r#powered == true && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#open == true && self.r#waterlogged == false { return 6946; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false && self.r#powered == true && self.r#waterlogged == true { return 6917; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#open == true && self.r#powered == false { return 6971; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#open == true && self.r#powered == true { return 6961; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#open == false && self.r#half == Half::Bottom { return 6927; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true && self.r#waterlogged == true { return 6953; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == false && self.r#powered == false && self.r#facing == Facing::South { return 6944; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == true && self.r#powered == false && self.r#facing == Facing::East { return 6963; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false && self.r#facing == Facing::East && self.r#waterlogged == false { return 6972; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == true { return 6924; }
        if self.r#open == false && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#powered == true { return 6966; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#waterlogged == true && self.r#powered == true && self.r#half == Half::Bottom { return 6973; }
        if self.r#waterlogged == false && self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == true { return 6962; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#powered == false && self.r#open == true { return 6916; }
        if self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false && self.r#facing == Facing::West && self.r#waterlogged == false { return 6960; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#powered == false && self.r#open == false { return 6976; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == false && self.r#powered == false { return 6951; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#powered == false && self.r#open == false { return 6959; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true && self.r#powered == true && self.r#open == false { return 6933; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 6958; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#open == true && self.r#waterlogged == false && self.r#half == Half::Top { return 6948; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == true { return 6915; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#open == false && self.r#half == Half::Top { return 6949; }
        if self.r#waterlogged == true && self.r#open == true && self.r#powered == false && self.r#half == Half::Top && self.r#facing == Facing::West { return 6947; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == true { return 6970; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == true && self.r#facing == Facing::West && self.r#powered == true { return 6954; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::North && self.r#powered == false { return 6928; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#powered == true && self.r#open == false && self.r#waterlogged == true { return 6925; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false { return 6956; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false { return 6919; }
        if self.r#powered == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#open == true && self.r#waterlogged == true { return 6937; }
        if self.r#powered == true && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 6921; }
        if self.r#open == false && self.r#powered == true && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 6918; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#open == true { return 6945; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false { return 6940; }
        if self.r#powered == false && self.r#open == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 6920; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#open == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 6926; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == true && self.r#powered == false { return 6931; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == false && self.r#powered == true { return 6942; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Top && self.r#facing == Facing::North { return 6914; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == false && self.r#powered == true { return 6941; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#open == false { return 6965; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#open == true { return 6922; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == true && self.r#open == false && self.r#half == Half::Bottom { return 6957; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == true { return 6929; }
        if self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#powered == true { return 6913; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#open == true && self.r#waterlogged == false && self.r#half == Half::Top { return 6964; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Top { return 6967; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#open == false && self.r#powered == false { return 6952; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6930 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 6934 {
            return Some(OakTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 6935 {
            return Some(OakTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 6936 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 6938 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6950 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 6975 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6955 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6974 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6969 {
            return Some(OakTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6923 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6968 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 6939 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 6932 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6943 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
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
        if state_id == 6917 {
            return Some(OakTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6971 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 6961 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 6927 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6953 {
            return Some(OakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6944 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6963 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6972 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6924 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 6966 {
            return Some(OakTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 6973 {
            return Some(OakTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6962 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 6916 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 6960 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6976 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 6951 {
            return Some(OakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6959 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 6933 {
            return Some(OakTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 6958 {
            return Some(OakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6948 {
            return Some(OakTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 6915 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 6949 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 6947 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 6970 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 6954 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 6928 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 6925 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6956 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 6919 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 6937 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6921 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 6918 {
            return Some(OakTrapdoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6945 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 6940 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 6920 {
            return Some(OakTrapdoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6926 {
            return Some(OakTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6931 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 6942 {
            return Some(OakTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 6914 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 6941 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 6965 {
            return Some(OakTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 6922 {
            return Some(OakTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 6957 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 6929 {
            return Some(OakTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 6913 {
            return Some(OakTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 6964 {
            return Some(OakTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 6967 {
            return Some(OakTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 6952 {
            return Some(OakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
            });
        }
        return None;
    }
}

