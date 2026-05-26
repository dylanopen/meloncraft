use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperGolemStatue {
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for WeatheredCopperGolemStatue {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27162;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 27167;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 27177;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 27180;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Running
        {
            return 27171;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27154;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::North
        {
            return 27165;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 27168;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27158;
        }
        if self.r#facing == Facing::East
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#waterlogged == true
        {
            return 27155;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 27156;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::North
        {
            return 27150;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 27176;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27164;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 27174;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27163;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 27175;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::North
        {
            return 27166;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27153;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 27160;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::North
        {
            return 27157;
        }
        if self.r#facing == Facing::West
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#waterlogged == true
        {
            return 27161;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27179;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::East
        {
            return 27172;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27159;
        }
        if self.r#facing == Facing::North
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#waterlogged == true
        {
            return 27149;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27173;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::West
        {
            return 27178;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 27170;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Running
        {
            return 27169;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27151;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 27152;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27162 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27167 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27177 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27180 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27171 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27154 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27165 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27168 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27158 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27155 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27156 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27150 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
            });
        }
        if state_id == 27176 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27164 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27174 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27163 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27175 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27166 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27153 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27160 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27157 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27161 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
            });
        }
        if state_id == 27179 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27172 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
            });
        }
        if state_id == 27159 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27149 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27173 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27178 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
            });
        }
        if state_id == 27170 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27169 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27151 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27152 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
