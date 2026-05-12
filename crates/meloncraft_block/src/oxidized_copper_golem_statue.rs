use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperGolemStatue {
    pub r#facing: Facing,
    pub r#copper_golem_pose: CopperGolemPose,
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
pub enum CopperGolemPose {
    Standing,
    Sitting,
    Running,
    Star,
}

impl BlockState for OxidizedCopperGolemStatue {
    fn to_id(self) -> i32 {
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27185; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::West { return 27201; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::South { return 27199; }
        if block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true { return 27183; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27209; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::West { return 27210; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27190; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27195; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27184; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == false { return 27182; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::North { return 27198; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27200; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27208; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == false { return 27188; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27205; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true { return 27189; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27187; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == false { return 27196; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27197; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::West { return 27202; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27203; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27204; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27193; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::East { return 27212; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::West { return 27194; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27181; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27191; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27186; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::South { return 27207; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 27211; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North { return 27206; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::South { return 27192; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27185 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27201 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
            });
        }
        if state_id == 27199 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
            });
        }
        if state_id == 27183 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27209 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27210 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
            });
        }
        if state_id == 27190 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27195 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27184 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27182 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27198 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27200 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27208 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27188 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27205 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27189 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
            });
        }
        if state_id == 27187 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27196 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
            });
        }
        if state_id == 27197 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27202 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
            });
        }
        if state_id == 27203 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27204 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27193 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27212 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
            });
        }
        if state_id == 27194 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
            });
        }
        if state_id == 27181 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27191 {
            return Some(OxidizedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27186 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27207 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
            });
        }
        if state_id == 27211 {
            return Some(OxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27206 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
            });
        }
        if state_id == 27192 {
            return Some(OxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

