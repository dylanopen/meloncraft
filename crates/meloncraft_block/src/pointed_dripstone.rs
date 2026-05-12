use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointedDripstone {
    pub waterlogged: bool,
    pub r#vertical_direction: VerticalDirection,
    pub r#thickness: Thickness,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Thickness {
    TipMerge,
    Tip,
    Frustum,
    Middle,
    Base,
}

impl BlockState for PointedDripstone {
    fn to_id(self) -> i32 {
        if block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#waterlogged == true && block_state.r#thickness == Thickness::Base { return 27551; }
        if block_state.r#waterlogged == true && block_state.r#thickness == Thickness::Frustum && block_state.r#vertical_direction == VerticalDirection::Up { return 27541; }
        if block_state.r#waterlogged == true && block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#thickness == Thickness::TipMerge { return 27535; }
        if block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#thickness == Thickness::Frustum && block_state.r#waterlogged == false { return 27544; }
        if block_state.r#waterlogged == false && block_state.r#thickness == Thickness::TipMerge && block_state.r#vertical_direction == VerticalDirection::Up { return 27534; }
        if block_state.r#thickness == Thickness::Tip && block_state.r#waterlogged == true && block_state.r#vertical_direction == VerticalDirection::Down { return 27539; }
        if block_state.r#waterlogged == false && block_state.r#thickness == Thickness::Tip && block_state.r#vertical_direction == VerticalDirection::Up { return 27538; }
        if block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#waterlogged == true && block_state.r#thickness == Thickness::Frustum { return 27543; }
        if block_state.r#waterlogged == false && block_state.r#thickness == Thickness::TipMerge && block_state.r#vertical_direction == VerticalDirection::Down { return 27536; }
        if block_state.r#vertical_direction == VerticalDirection::Up && block_state.r#thickness == Thickness::Middle && block_state.r#waterlogged == true { return 27545; }
        if block_state.r#vertical_direction == VerticalDirection::Up && block_state.r#waterlogged == true && block_state.r#thickness == Thickness::Tip { return 27537; }
        if block_state.r#waterlogged == false && block_state.r#vertical_direction == VerticalDirection::Up && block_state.r#thickness == Thickness::Base { return 27550; }
        if block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#thickness == Thickness::Tip && block_state.r#waterlogged == false { return 27540; }
        if block_state.r#vertical_direction == VerticalDirection::Up && block_state.r#waterlogged == false && block_state.r#thickness == Thickness::Middle { return 27546; }
        if block_state.r#thickness == Thickness::Base && block_state.r#waterlogged == true && block_state.r#vertical_direction == VerticalDirection::Up { return 27549; }
        if block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#thickness == Thickness::Base && block_state.r#waterlogged == false { return 27552; }
        if block_state.r#thickness == Thickness::TipMerge && block_state.r#vertical_direction == VerticalDirection::Up && block_state.r#waterlogged == true { return 27533; }
        if block_state.r#waterlogged == false && block_state.r#thickness == Thickness::Frustum && block_state.r#vertical_direction == VerticalDirection::Up { return 27542; }
        if block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#waterlogged == false && block_state.r#thickness == Thickness::Middle { return 27548; }
        if block_state.r#thickness == Thickness::Middle && block_state.r#vertical_direction == VerticalDirection::Down && block_state.r#waterlogged == true { return 27547; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27551 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Down,
                r#waterlogged: true,
                r#thickness: Thickness::Base,
            });
        }
        if state_id == 27541 {
            return Some(PointedDripstone {
                r#waterlogged: true,
                r#thickness: Thickness::Frustum,
                r#vertical_direction: VerticalDirection::Up,
            });
        }
        if state_id == 27535 {
            return Some(PointedDripstone {
                r#waterlogged: true,
                r#vertical_direction: VerticalDirection::Down,
                r#thickness: Thickness::TipMerge,
            });
        }
        if state_id == 27544 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Down,
                r#thickness: Thickness::Frustum,
                r#waterlogged: false,
            });
        }
        if state_id == 27534 {
            return Some(PointedDripstone {
                r#waterlogged: false,
                r#thickness: Thickness::TipMerge,
                r#vertical_direction: VerticalDirection::Up,
            });
        }
        if state_id == 27539 {
            return Some(PointedDripstone {
                r#thickness: Thickness::Tip,
                r#waterlogged: true,
                r#vertical_direction: VerticalDirection::Down,
            });
        }
        if state_id == 27538 {
            return Some(PointedDripstone {
                r#waterlogged: false,
                r#thickness: Thickness::Tip,
                r#vertical_direction: VerticalDirection::Up,
            });
        }
        if state_id == 27543 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Down,
                r#waterlogged: true,
                r#thickness: Thickness::Frustum,
            });
        }
        if state_id == 27536 {
            return Some(PointedDripstone {
                r#waterlogged: false,
                r#thickness: Thickness::TipMerge,
                r#vertical_direction: VerticalDirection::Down,
            });
        }
        if state_id == 27545 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Up,
                r#thickness: Thickness::Middle,
                r#waterlogged: true,
            });
        }
        if state_id == 27537 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Up,
                r#waterlogged: true,
                r#thickness: Thickness::Tip,
            });
        }
        if state_id == 27550 {
            return Some(PointedDripstone {
                r#waterlogged: false,
                r#vertical_direction: VerticalDirection::Up,
                r#thickness: Thickness::Base,
            });
        }
        if state_id == 27540 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Down,
                r#thickness: Thickness::Tip,
                r#waterlogged: false,
            });
        }
        if state_id == 27546 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Up,
                r#waterlogged: false,
                r#thickness: Thickness::Middle,
            });
        }
        if state_id == 27549 {
            return Some(PointedDripstone {
                r#thickness: Thickness::Base,
                r#waterlogged: true,
                r#vertical_direction: VerticalDirection::Up,
            });
        }
        if state_id == 27552 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Down,
                r#thickness: Thickness::Base,
                r#waterlogged: false,
            });
        }
        if state_id == 27533 {
            return Some(PointedDripstone {
                r#thickness: Thickness::TipMerge,
                r#vertical_direction: VerticalDirection::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 27542 {
            return Some(PointedDripstone {
                r#waterlogged: false,
                r#thickness: Thickness::Frustum,
                r#vertical_direction: VerticalDirection::Up,
            });
        }
        if state_id == 27548 {
            return Some(PointedDripstone {
                r#vertical_direction: VerticalDirection::Down,
                r#waterlogged: false,
                r#thickness: Thickness::Middle,
            });
        }
        if state_id == 27547 {
            return Some(PointedDripstone {
                r#thickness: Thickness::Middle,
                r#vertical_direction: VerticalDirection::Down,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

