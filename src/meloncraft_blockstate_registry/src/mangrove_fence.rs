use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveFence {
    pub east: bool,
    pub south: bool,
    pub west: bool,
    pub waterlogged: bool,
    pub north: bool,
}

impl BlockState for MangroveFence {
    fn to_id(&self) -> i32 {
        if self.r#west == false
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#north == false
        {
            return 13809;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == false
        {
            return 13825;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 13805;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == true
        {
            return 13804;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == false
            && self.r#north == false
        {
            return 13807;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 13813;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#west == true
            && self.r#east == true
            && self.r#waterlogged == true
        {
            return 13802;
        }
        if self.r#south == false
            && self.r#east == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 13822;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == false
            && self.r#east == true
            && self.r#north == false
        {
            return 13806;
        }
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#west == false
            && self.r#east == true
        {
            return 13799;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 13794;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#north == false
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 13824;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#south == false
            && self.r#west == false
        {
            return 13801;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == true
            && self.r#east == true
        {
            return 13808;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 13818;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == false
            && self.r#east == false
        {
            return 13817;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == true
        {
            return 13815;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == true
        {
            return 13811;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 13820;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == true
        {
            return 13800;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == true
            && self.r#north == true
        {
            return 13797;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == true
            && self.r#west == true
        {
            return 13796;
        }
        if self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == true
            && self.r#north == true
            && self.r#east == false
        {
            return 13816;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == false
        {
            return 13821;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 13798;
        }
        if self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == true
            && self.r#east == true
            && self.r#north == true
        {
            return 13795;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#west == true
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 13812;
        }
        if self.r#east == true
            && self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 13803;
        }
        if self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == true
            && self.r#east == false
        {
            return 13810;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 13819;
        }
        if self.r#south == false
            && self.r#north == false
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 13823;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#east == false
            && self.r#south == false
            && self.r#north == true
        {
            return 13814;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13809 {
            return Some(MangroveFence {
                r#west: false,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 13825 {
            return Some(MangroveFence {
                r#west: false,
                r#north: false,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13805 {
            return Some(MangroveFence {
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13804 {
            return Some(MangroveFence {
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13807 {
            return Some(MangroveFence {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 13813 {
            return Some(MangroveFence {
                r#north: true,
                r#south: true,
                r#east: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13802 {
            return Some(MangroveFence {
                r#north: false,
                r#south: true,
                r#west: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13822 {
            return Some(MangroveFence {
                r#south: false,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13806 {
            return Some(MangroveFence {
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 13799 {
            return Some(MangroveFence {
                r#north: true,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 13794 {
            return Some(MangroveFence {
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13824 {
            return Some(MangroveFence {
                r#west: true,
                r#east: false,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13801 {
            return Some(MangroveFence {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 13808 {
            return Some(MangroveFence {
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13818 {
            return Some(MangroveFence {
                r#west: true,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 13817 {
            return Some(MangroveFence {
                r#north: true,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 13815 {
            return Some(MangroveFence {
                r#west: false,
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13811 {
            return Some(MangroveFence {
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13820 {
            return Some(MangroveFence {
                r#west: true,
                r#north: false,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13800 {
            return Some(MangroveFence {
                r#east: true,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 13797 {
            return Some(MangroveFence {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 13796 {
            return Some(MangroveFence {
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13816 {
            return Some(MangroveFence {
                r#waterlogged: false,
                r#south: false,
                r#west: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 13821 {
            return Some(MangroveFence {
                r#south: true,
                r#north: false,
                r#west: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13798 {
            return Some(MangroveFence {
                r#east: true,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13795 {
            return Some(MangroveFence {
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 13812 {
            return Some(MangroveFence {
                r#east: false,
                r#north: true,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13803 {
            return Some(MangroveFence {
                r#east: true,
                r#west: false,
                r#north: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 13810 {
            return Some(MangroveFence {
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 13819 {
            return Some(MangroveFence {
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13823 {
            return Some(MangroveFence {
                r#south: false,
                r#north: false,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13814 {
            return Some(MangroveFence {
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: false,
                r#north: true,
            });
        }
        return None;
    }
}
