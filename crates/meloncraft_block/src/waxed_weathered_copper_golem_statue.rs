use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperGolemStatue {
    pub waterlogged: bool,
    pub r#copper_golem_pose: CopperGolemPose,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopperGolemPose {
    Standing,
    Sitting,
    Running,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WaxedWeatheredCopperGolemStatue {
    fn to_id(&self) -> i32 {
        if self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#waterlogged == false && self.r#facing == Facing::East { return 27284; }
        if self.r#facing == Facing::West && self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#waterlogged == true { return 27281; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Sitting { return 27292; }
        if self.r#facing == Facing::North && self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#waterlogged == true { return 27285; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#copper_golem_pose == CopperGolemPose::Star { return 27304; }
        if self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#facing == Facing::North { return 27286; }
        if self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::South { return 27303; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Star { return 27301; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Running { return 27300; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Sitting { return 27287; }
        if self.r#facing == Facing::North && self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#waterlogged == false { return 27278; }
        if self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::North && self.r#waterlogged == false { return 27302; }
        if self.r#facing == Facing::South && self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#waterlogged == false { return 27280; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Standing { return 27282; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Sitting { return 27288; }
        if self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#facing == Facing::North { return 27277; }
        if self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::West && self.r#waterlogged == false { return 27306; }
        if self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::East && self.r#waterlogged == false { return 27308; }
        if self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#facing == Facing::West { return 27289; }
        if self.r#facing == Facing::West && self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#waterlogged == false { return 27290; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#copper_golem_pose == CopperGolemPose::Star { return 27305; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#copper_golem_pose == CopperGolemPose::Standing { return 27283; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Running { return 27293; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#copper_golem_pose == CopperGolemPose::Running { return 27299; }
        if self.r#copper_golem_pose == CopperGolemPose::Running && self.r#facing == Facing::West && self.r#waterlogged == false { return 27298; }
        if self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Running && self.r#facing == Facing::West { return 27297; }
        if self.r#copper_golem_pose == CopperGolemPose::Running && self.r#facing == Facing::South && self.r#waterlogged == false { return 27296; }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#waterlogged == true && self.r#facing == Facing::East { return 27291; }
        if self.r#facing == Facing::South && self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#waterlogged == true { return 27279; }
        if self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Running && self.r#facing == Facing::North { return 27294; }
        if self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::East && self.r#waterlogged == true { return 27307; }
        if self.r#copper_golem_pose == CopperGolemPose::Running && self.r#waterlogged == true && self.r#facing == Facing::South { return 27295; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27284 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27281 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27292 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27285 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
            });
        }
        if state_id == 27304 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27286 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27303 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
            });
        }
        if state_id == 27301 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27300 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27287 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27278 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27302 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27280 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27282 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27288 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27277 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
            });
        }
        if state_id == 27306 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27308 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27289 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
            });
        }
        if state_id == 27290 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
            });
        }
        if state_id == 27305 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27283 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27293 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27299 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27298 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27297 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
            });
        }
        if state_id == 27296 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27291 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27279 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27294 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27307 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27295 {
            return Some(WaxedWeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

