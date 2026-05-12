use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperGolemStatue {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#copper_golem_pose: CopperGolemPose,
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

impl BlockState for CopperGolemStatue {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North { return 27109; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27096; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27098; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27086; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::South { return 27095; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27105; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27090; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27113; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 27116; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27111; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North { return 27094; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27108; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27115; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27102; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::North { return 27101; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 27091; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27099; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27100; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::North { return 27085; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 27106; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 27114; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27092; }
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true { return 27089; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27097; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27093; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27110; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27104; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27112; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27087; }
        if block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true { return 27103; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27107; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27088; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27109 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
            });
        }
        if state_id == 27096 {
            return Some(CopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27098 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27086 {
            return Some(CopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27095 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
            });
        }
        if state_id == 27105 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27090 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27113 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27116 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27111 {
            return Some(CopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27094 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27108 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27115 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27102 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27101 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27091 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27099 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27100 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27085 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
            });
        }
        if state_id == 27106 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27114 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27092 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27089 {
            return Some(CopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27097 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27093 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27110 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27104 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27112 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27087 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27103 {
            return Some(CopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
            });
        }
        if state_id == 27107 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27088 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

