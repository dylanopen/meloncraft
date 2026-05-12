use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonTrapdoor {
    pub r#half: Half,
    pub r#facing: Facing,
    pub open: bool,
    pub powered: bool,
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

impl BlockState for CrimsonTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Bottom && self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == false { return 20922; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#open == false && self.r#waterlogged == true && self.r#powered == true { return 20964; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == false && self.r#open == true && self.r#facing == Facing::West { return 20946; }
        if self.r#half == Half::Top && self.r#open == true && self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::East { return 20962; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false && self.r#powered == true { return 20961; }
        if self.r#waterlogged == false && self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == false { return 20963; }
        if self.r#waterlogged == true && self.r#open == true && self.r#powered == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 20944; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#open == true && self.r#powered == false { return 20947; }
        if self.r#open == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true { return 20934; }
        if self.r#half == Half::Top && self.r#open == false && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West { return 20949; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::South { return 20940; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#open == true && self.r#waterlogged == false && self.r#facing == Facing::West { return 20955; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == true { return 20921; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::North { return 20920; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == true { return 20929; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#open == false && self.r#powered == true { return 20933; }
        if self.r#powered == true && self.r#open == true && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East { return 20960; }
        if self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == true { return 20915; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == false && self.r#powered == true { return 20941; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false && self.r#waterlogged == false { return 20939; }
        if self.r#waterlogged == true && self.r#open == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#powered == false { return 20926; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 20973; }
        if self.r#powered == false && self.r#open == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false { return 20935; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#powered == true && self.r#open == true { return 20928; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Top && self.r#powered == true { return 20932; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false && self.r#waterlogged == false { return 20971; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#open == false && self.r#waterlogged == true && self.r#powered == false { return 20966; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#powered == true && self.r#half == Half::Bottom && self.r#open == false { return 20972; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#powered == false && self.r#open == false && self.r#waterlogged == true { return 20950; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == false { return 20918; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true { return 20968; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Top { return 20931; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#powered == false && self.r#open == false { return 20959; }
        if self.r#open == false && self.r#powered == false && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top { return 20967; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true { return 20954; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == true { return 20938; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#open == false && self.r#waterlogged == true && self.r#powered == true { return 20916; }
        if self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#powered == true && self.r#waterlogged == false { return 20925; }
        if self.r#powered == true && self.r#open == true && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true { return 20912; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == false { return 20948; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false && self.r#waterlogged == false { return 20943; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#powered == true && self.r#open == true && self.r#facing == Facing::North { return 20913; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#open == true { return 20945; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#powered == true && self.r#open == false { return 20957; }
        if self.r#facing == Facing::North && self.r#open == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false { return 20927; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#open == false { return 20917; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 20923; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#open == false && self.r#powered == true { return 20965; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == true && self.r#powered == false { return 20942; }
        if self.r#powered == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#open == false { return 20974; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#open == true && self.r#powered == true { return 20969; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Bottom { return 20937; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == false && self.r#open == false && self.r#half == Half::Top { return 20919; }
        if self.r#open == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#powered == false { return 20951; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false { return 20975; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == false && self.r#open == true && self.r#half == Half::Top { return 20914; }
        if self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#powered == true { return 20924; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Top && self.r#waterlogged == true { return 20930; }
        if self.r#open == false && self.r#facing == Facing::West && self.r#powered == false && self.r#half == Half::Bottom && self.r#waterlogged == true { return 20958; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#open == true && self.r#waterlogged == false && self.r#half == Half::Bottom { return 20953; }
        if self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == true && self.r#half == Half::Bottom { return 20956; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == true { return 20952; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#powered == true && self.r#open == true && self.r#waterlogged == true { return 20936; }
        if self.r#powered == false && self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 20970; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20922 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 20964 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 20946 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 20962 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20961 {
            return Some(CrimsonTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 20963 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 20944 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 20947 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 20934 {
            return Some(CrimsonTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20949 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20940 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 20955 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20921 {
            return Some(CrimsonTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 20920 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 20929 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 20933 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 20960 {
            return Some(CrimsonTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 20915 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 20941 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 20939 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20926 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 20973 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20935 {
            return Some(CrimsonTrapdoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 20928 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 20932 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 20971 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20966 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 20972 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 20950 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20918 {
            return Some(CrimsonTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 20968 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 20931 {
            return Some(CrimsonTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 20959 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 20967 {
            return Some(CrimsonTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 20954 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 20938 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 20916 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 20925 {
            return Some(CrimsonTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20912 {
            return Some(CrimsonTrapdoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 20948 {
            return Some(CrimsonTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 20943 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20913 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 20945 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 20957 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 20927 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20917 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 20923 {
            return Some(CrimsonTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 20965 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 20942 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 20974 {
            return Some(CrimsonTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 20969 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 20937 {
            return Some(CrimsonTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 20919 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 20951 {
            return Some(CrimsonTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 20975 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 20914 {
            return Some(CrimsonTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 20924 {
            return Some(CrimsonTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 20930 {
            return Some(CrimsonTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 20958 {
            return Some(CrimsonTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 20953 {
            return Some(CrimsonTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 20956 {
            return Some(CrimsonTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 20952 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20936 {
            return Some(CrimsonTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20970 {
            return Some(CrimsonTrapdoor {
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

