use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherBrickFence {
    pub south: bool,
    pub west: bool,
    pub north: bool,
    pub east: bool,
    pub waterlogged: bool,
}

impl BlockState for NetherBrickFence {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#north == true
            && self.r#east == true
            && self.r#south == true
        {
            return 9137;
        }
        if self.r#south == false
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#west == false
        {
            return 9139;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#east == true
            && self.r#west == false
        {
            return 9141;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#west == true
            && self.r#waterlogged == false
            && self.r#south == true
        {
            return 9152;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#south == true
        {
            return 9158;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 9155;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 9161;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 9134;
        }
        if self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == false
            && self.r#south == true
        {
            return 9159;
        }
        if self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == false
            && self.r#east == false
            && self.r#south == false
        {
            return 9163;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 9149;
        }
        if self.r#west == false
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#south == true
            && self.r#east == true
        {
            return 9143;
        }
        if self.r#north == true
            && self.r#west == false
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 9151;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#east == false
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 9164;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#north == false
            && self.r#south == false
            && self.r#east == false
        {
            return 9165;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#west == true
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 9142;
        }
        if self.r#south == true
            && self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 9150;
        }
        if self.r#north == true
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#east == false
        {
            return 9157;
        }
        if self.r#west == true
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#north == true
        {
            return 9138;
        }
        if self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == true
            && self.r#north == true
            && self.r#south == false
        {
            return 9140;
        }
        if self.r#waterlogged == true
            && self.r#south == false
            && self.r#east == true
            && self.r#west == false
            && self.r#north == false
        {
            return 9147;
        }
        if self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == true
            && self.r#north == false
            && self.r#south == false
        {
            return 9148;
        }
        if self.r#waterlogged == false
            && self.r#north == false
            && self.r#east == false
            && self.r#west == true
            && self.r#south == true
        {
            return 9160;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == true
        {
            return 9135;
        }
        if self.r#waterlogged == false
            && self.r#east == false
            && self.r#south == false
            && self.r#west == true
            && self.r#north == true
        {
            return 9156;
        }
        if self.r#west == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#north == true
        {
            return 9154;
        }
        if self.r#west == true
            && self.r#east == true
            && self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 9146;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == true
            && self.r#east == true
        {
            return 9144;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#south == true
        {
            return 9136;
        }
        if self.r#north == false
            && self.r#west == false
            && self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == false
        {
            return 9145;
        }
        if self.r#north == false
            && self.r#west == true
            && self.r#east == false
            && self.r#south == false
            && self.r#waterlogged == true
        {
            return 9162;
        }
        if self.r#west == false
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#north == true
            && self.r#south == true
        {
            return 9153;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9137 {
            return Some(NetherBrickFence {
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 9139 {
            return Some(NetherBrickFence {
                r#south: false,
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 9141 {
            return Some(NetherBrickFence {
                r#south: false,
                r#waterlogged: false,
                r#north: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 9152 {
            return Some(NetherBrickFence {
                r#north: true,
                r#east: false,
                r#west: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 9158 {
            return Some(NetherBrickFence {
                r#west: true,
                r#east: false,
                r#waterlogged: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 9155 {
            return Some(NetherBrickFence {
                r#east: false,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 9161 {
            return Some(NetherBrickFence {
                r#east: false,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 9134 {
            return Some(NetherBrickFence {
                r#east: true,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 9159 {
            return Some(NetherBrickFence {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 9163 {
            return Some(NetherBrickFence {
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 9149 {
            return Some(NetherBrickFence {
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 9143 {
            return Some(NetherBrickFence {
                r#west: false,
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 9151 {
            return Some(NetherBrickFence {
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 9164 {
            return Some(NetherBrickFence {
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 9165 {
            return Some(NetherBrickFence {
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 9142 {
            return Some(NetherBrickFence {
                r#east: true,
                r#north: false,
                r#west: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 9150 {
            return Some(NetherBrickFence {
                r#south: true,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 9157 {
            return Some(NetherBrickFence {
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 9138 {
            return Some(NetherBrickFence {
                r#west: true,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 9140 {
            return Some(NetherBrickFence {
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 9147 {
            return Some(NetherBrickFence {
                r#waterlogged: true,
                r#south: false,
                r#east: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 9148 {
            return Some(NetherBrickFence {
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 9160 {
            return Some(NetherBrickFence {
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 9135 {
            return Some(NetherBrickFence {
                r#west: false,
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 9156 {
            return Some(NetherBrickFence {
                r#waterlogged: false,
                r#east: false,
                r#south: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 9154 {
            return Some(NetherBrickFence {
                r#west: true,
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 9146 {
            return Some(NetherBrickFence {
                r#west: true,
                r#east: true,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9144 {
            return Some(NetherBrickFence {
                r#north: false,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 9136 {
            return Some(NetherBrickFence {
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 9145 {
            return Some(NetherBrickFence {
                r#north: false,
                r#west: false,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 9162 {
            return Some(NetherBrickFence {
                r#north: false,
                r#west: true,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9153 {
            return Some(NetherBrickFence {
                r#west: false,
                r#waterlogged: false,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        return None;
    }
}
