use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperGolemStatue {
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

impl BlockState for ExposedCopperGolemStatue {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == false { return 27120; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27121; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27144; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::West { return 27130; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27140; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::North { return 27134; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27147; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27129; }
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == false { return 27122; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::West { return 27138; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true { return 27133; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::East { return 27131; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27136; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27146; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27126; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::South { return 27143; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27118; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27127; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27139; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27124; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27132; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27142; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27148; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North { return 27141; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 27123; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27117; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North { return 27125; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 27128; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27145; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27137; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::South { return 27135; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::South { return 27119; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27120 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27121 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27144 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27130 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
            });
        }
        if state_id == 27140 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27134 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27147 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27129 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27122 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27138 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
            });
        }
        if state_id == 27133 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
            });
        }
        if state_id == 27131 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
            });
        }
        if state_id == 27136 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27146 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27126 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27143 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
            });
        }
        if state_id == 27118 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27127 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27139 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27124 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27132 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27142 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27148 {
            return Some(ExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27141 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
            });
        }
        if state_id == 27123 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27117 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27125 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27128 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27145 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27137 {
            return Some(ExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27135 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
            });
        }
        if state_id == 27119 {
            return Some(ExposedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

