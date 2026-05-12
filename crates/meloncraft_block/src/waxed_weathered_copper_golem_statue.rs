use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperGolemStatue {
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

impl BlockState for WaxedWeatheredCopperGolemStatue {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true { return 27281; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27296; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27300; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27278; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::South { return 27280; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false { return 27308; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27283; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::West { return 27297; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27302; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27289; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27303; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27306; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27277; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27292; }
        if block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == false { return 27288; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27285; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27301; }
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == false { return 27282; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27305; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27299; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27286; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == false { return 27294; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27279; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27284; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27291; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::South { return 27304; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27295; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 27307; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 27298; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::North { return 27293; }
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == false { return 27290; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27287; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27281 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27296 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27300 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27278 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27280 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
            });
        }
        if state_id == 27308 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
            });
        }
        if state_id == 27283 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27297 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
            });
        }
        if state_id == 27302 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27289 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27303 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27306 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27277 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27292 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27288 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
            });
        }
        if state_id == 27285 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27301 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27282 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27305 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27299 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27286 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27294 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: false,
            });
        }
        if state_id == 27279 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27284 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27291 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27304 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
            });
        }
        if state_id == 27295 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27307 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27298 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27293 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27290 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
            });
        }
        if state_id == 27287 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

