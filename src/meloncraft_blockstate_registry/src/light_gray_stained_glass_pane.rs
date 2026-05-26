use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub west: bool,
    pub south: bool,
    pub waterlogged: bool,
}

impl BlockState for LightGrayStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#south == true
            && self.r#north == false
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11539;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == false
        {
            return 11535;
        }
        if self.r#north == false
            && self.r#west == true
            && self.r#south == false
            && self.r#east == true
            && self.r#waterlogged == false
        {
            return 11528;
        }
        if self.r#west == true
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#south == true
            && self.r#north == true
        {
            return 11532;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == true
        {
            return 11531;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#south == false
            && self.r#west == false
            && self.r#waterlogged == true
        {
            return 11527;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#north == true
            && self.r#east == true
            && self.r#waterlogged == true
        {
            return 11518;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 11516;
        }
        if self.r#south == false
            && self.r#north == false
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 11529;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == true
        {
            return 11538;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 11542;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#west == true
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 11522;
        }
        if self.r#west == false
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#north == false
            && self.r#east == false
        {
            return 11545;
        }
        if self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#north == true
            && self.r#west == true
        {
            return 11534;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#south == false
            && self.r#east == true
            && self.r#waterlogged == true
        {
            return 11526;
        }
        if self.r#waterlogged == false
            && self.r#east == false
            && self.r#west == false
            && self.r#south == true
            && self.r#north == true
        {
            return 11533;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
        {
            return 11544;
        }
        if self.r#waterlogged == true
            && self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#west == true
        {
            return 11514;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#west == true
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 11524;
        }
        if self.r#north == false
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#east == false
            && self.r#west == false
        {
            return 11543;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#east == true
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 11521;
        }
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == true
            && self.r#south == true
        {
            return 11530;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#south == true
            && self.r#west == false
        {
            return 11517;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#west == false
        {
            return 11515;
        }
        if self.r#south == true
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#west == false
            && self.r#east == true
        {
            return 11525;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#west == true
            && self.r#waterlogged == false
        {
            return 11536;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#east == false
        {
            return 11541;
        }
        if self.r#north == true
            && self.r#west == false
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#south == false
        {
            return 11519;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#east == true
        {
            return 11523;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == true
            && self.r#east == true
            && self.r#south == false
        {
            return 11520;
        }
        if self.r#north == false
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 11540;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == false
            && self.r#north == true
        {
            return 11537;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11539 {
            return Some(LightGrayStainedGlassPane {
                r#south: true,
                r#north: false,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11535 {
            return Some(LightGrayStainedGlassPane {
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11528 {
            return Some(LightGrayStainedGlassPane {
                r#north: false,
                r#west: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11532 {
            return Some(LightGrayStainedGlassPane {
                r#west: true,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 11531 {
            return Some(LightGrayStainedGlassPane {
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11527 {
            return Some(LightGrayStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11518 {
            return Some(LightGrayStainedGlassPane {
                r#south: false,
                r#west: true,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11516 {
            return Some(LightGrayStainedGlassPane {
                r#north: true,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11529 {
            return Some(LightGrayStainedGlassPane {
                r#south: false,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11538 {
            return Some(LightGrayStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11542 {
            return Some(LightGrayStainedGlassPane {
                r#east: false,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11522 {
            return Some(LightGrayStainedGlassPane {
                r#north: false,
                r#east: true,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11545 {
            return Some(LightGrayStainedGlassPane {
                r#west: false,
                r#waterlogged: false,
                r#south: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 11534 {
            return Some(LightGrayStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 11526 {
            return Some(LightGrayStainedGlassPane {
                r#west: true,
                r#north: false,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11533 {
            return Some(LightGrayStainedGlassPane {
                r#waterlogged: false,
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 11544 {
            return Some(LightGrayStainedGlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11514 {
            return Some(LightGrayStainedGlassPane {
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11524 {
            return Some(LightGrayStainedGlassPane {
                r#north: false,
                r#east: true,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11543 {
            return Some(LightGrayStainedGlassPane {
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11521 {
            return Some(LightGrayStainedGlassPane {
                r#south: false,
                r#north: true,
                r#east: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11530 {
            return Some(LightGrayStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11517 {
            return Some(LightGrayStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11515 {
            return Some(LightGrayStainedGlassPane {
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 11525 {
            return Some(LightGrayStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11536 {
            return Some(LightGrayStainedGlassPane {
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11541 {
            return Some(LightGrayStainedGlassPane {
                r#south: true,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11519 {
            return Some(LightGrayStainedGlassPane {
                r#north: true,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 11523 {
            return Some(LightGrayStainedGlassPane {
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11520 {
            return Some(LightGrayStainedGlassPane {
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11540 {
            return Some(LightGrayStainedGlassPane {
                r#north: false,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11537 {
            return Some(LightGrayStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#north: true,
            });
        }
        return None;
    }
}
