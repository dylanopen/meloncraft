use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vine {
    pub south: bool,
    pub east: bool,
    pub north: bool,
    pub up: bool,
    pub west: bool,
}

impl BlockState for Vine {
    fn to_id(&self) -> i32 {
        if self.r#west == true
            && self.r#south == true
            && self.r#north == true
            && self.r#up == false
            && self.r#east == true
        {
            return 8159;
        }
        if self.r#north == true
            && self.r#up == false
            && self.r#west == false
            && self.r#east == true
            && self.r#south == false
        {
            return 8164;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#west == true
            && self.r#up == true
            && self.r#south == true
        {
            return 8165;
        }
        if self.r#up == true
            && self.r#west == true
            && self.r#north == true
            && self.r#east == true
            && self.r#south == true
        {
            return 8157;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#north == false
            && self.r#west == false
            && self.r#up == false
        {
            return 8172;
        }
        if self.r#east == true
            && self.r#west == false
            && self.r#up == true
            && self.r#south == true
            && self.r#north == true
        {
            return 8158;
        }
        if self.r#south == true
            && self.r#up == false
            && self.r#west == false
            && self.r#north == true
            && self.r#east == true
        {
            return 8160;
        }
        if self.r#north == false
            && self.r#west == false
            && self.r#up == false
            && self.r#east == true
            && self.r#south == true
        {
            return 8168;
        }
        if self.r#west == true
            && self.r#south == true
            && self.r#east == false
            && self.r#up == false
            && self.r#north == true
        {
            return 8175;
        }
        if self.r#east == false
            && self.r#up == true
            && self.r#north == false
            && self.r#south == false
            && self.r#west == false
        {
            return 8186;
        }
        if self.r#south == false
            && self.r#east == true
            && self.r#north == true
            && self.r#up == false
            && self.r#west == true
        {
            return 8163;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#up == false
            && self.r#north == true
            && self.r#west == false
        {
            return 8176;
        }
        if self.r#up == true
            && self.r#east == false
            && self.r#north == true
            && self.r#west == false
            && self.r#south == true
        {
            return 8174;
        }
        if self.r#south == false
            && self.r#up == false
            && self.r#west == true
            && self.r#east == false
            && self.r#north == false
        {
            return 8187;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#north == true
            && self.r#up == true
            && self.r#west == true
        {
            return 8173;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#up == false
            && self.r#west == false
            && self.r#east == false
        {
            return 8188;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#up == true
            && self.r#north == false
            && self.r#west == false
        {
            return 8170;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#north == false
            && self.r#south == true
            && self.r#up == true
        {
            return 8166;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#north == false
            && self.r#up == false
            && self.r#east == true
        {
            return 8171;
        }
        if self.r#up == true
            && self.r#east == true
            && self.r#south == false
            && self.r#north == true
            && self.r#west == false
        {
            return 8162;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#up == true
            && self.r#south == false
            && self.r#east == true
        {
            return 8169;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#east == false
            && self.r#north == true
            && self.r#up == false
        {
            return 8180;
        }
        if self.r#west == true
            && self.r#south == false
            && self.r#east == true
            && self.r#north == true
            && self.r#up == true
        {
            return 8161;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#west == true
            && self.r#up == false
        {
            return 8179;
        }
        if self.r#up == true
            && self.r#north == true
            && self.r#south == false
            && self.r#east == false
            && self.r#west == false
        {
            return 8178;
        }
        if self.r#up == false
            && self.r#north == false
            && self.r#west == true
            && self.r#east == true
            && self.r#south == true
        {
            return 8167;
        }
        if self.r#south == true
            && self.r#east == false
            && self.r#west == true
            && self.r#north == false
            && self.r#up == true
        {
            return 8181;
        }
        if self.r#east == false
            && self.r#west == false
            && self.r#north == false
            && self.r#south == true
            && self.r#up == false
        {
            return 8184;
        }
        if self.r#up == false
            && self.r#west == true
            && self.r#east == false
            && self.r#north == false
            && self.r#south == true
        {
            return 8183;
        }
        if self.r#north == false
            && self.r#up == true
            && self.r#east == false
            && self.r#south == true
            && self.r#west == false
        {
            return 8182;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#up == true
        {
            return 8177;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#north == false
            && self.r#up == true
            && self.r#south == false
        {
            return 8185;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8159 {
            return Some(Vine {
                r#west: true,
                r#south: true,
                r#north: true,
                r#up: false,
                r#east: true,
            });
        }
        if state_id == 8164 {
            return Some(Vine {
                r#north: true,
                r#up: false,
                r#west: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 8165 {
            return Some(Vine {
                r#east: true,
                r#north: false,
                r#west: true,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 8157 {
            return Some(Vine {
                r#up: true,
                r#west: true,
                r#north: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 8172 {
            return Some(Vine {
                r#east: true,
                r#south: false,
                r#north: false,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 8158 {
            return Some(Vine {
                r#east: true,
                r#west: false,
                r#up: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 8160 {
            return Some(Vine {
                r#south: true,
                r#up: false,
                r#west: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 8168 {
            return Some(Vine {
                r#north: false,
                r#west: false,
                r#up: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 8175 {
            return Some(Vine {
                r#west: true,
                r#south: true,
                r#east: false,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 8186 {
            return Some(Vine {
                r#east: false,
                r#up: true,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 8163 {
            return Some(Vine {
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 8176 {
            return Some(Vine {
                r#east: false,
                r#south: true,
                r#up: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 8174 {
            return Some(Vine {
                r#up: true,
                r#east: false,
                r#north: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 8187 {
            return Some(Vine {
                r#south: false,
                r#up: false,
                r#west: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8173 {
            return Some(Vine {
                r#east: false,
                r#south: true,
                r#north: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 8188 {
            return Some(Vine {
                r#north: false,
                r#south: false,
                r#up: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 8170 {
            return Some(Vine {
                r#east: true,
                r#south: false,
                r#up: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 8166 {
            return Some(Vine {
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 8171 {
            return Some(Vine {
                r#south: false,
                r#west: true,
                r#north: false,
                r#up: false,
                r#east: true,
            });
        }
        if state_id == 8162 {
            return Some(Vine {
                r#up: true,
                r#east: true,
                r#south: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 8169 {
            return Some(Vine {
                r#west: true,
                r#north: false,
                r#up: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 8180 {
            return Some(Vine {
                r#west: false,
                r#south: false,
                r#east: false,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 8161 {
            return Some(Vine {
                r#west: true,
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 8179 {
            return Some(Vine {
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 8178 {
            return Some(Vine {
                r#up: true,
                r#north: true,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 8167 {
            return Some(Vine {
                r#up: false,
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 8181 {
            return Some(Vine {
                r#south: true,
                r#east: false,
                r#west: true,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 8184 {
            return Some(Vine {
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 8183 {
            return Some(Vine {
                r#up: false,
                r#west: true,
                r#east: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 8182 {
            return Some(Vine {
                r#north: false,
                r#up: true,
                r#east: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 8177 {
            return Some(Vine {
                r#west: true,
                r#east: false,
                r#north: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 8185 {
            return Some(Vine {
                r#west: true,
                r#east: false,
                r#north: false,
                r#up: true,
                r#south: false,
            });
        }
        return None;
    }
}
