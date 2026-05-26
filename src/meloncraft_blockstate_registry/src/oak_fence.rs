use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakFence {
    pub north: bool,
    pub waterlogged: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl BlockState for OakFence {
    fn to_id(&self) -> i32 {
        if self.r#west == false
            && self.r#east == false
            && self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == false
        {
            return 6783;
        }
        if self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == false
            && self.r#north == true
            && self.r#east == true
        {
            return 6771;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#north == false
            && self.r#east == true
            && self.r#waterlogged == false
        {
            return 6775;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == false
            && self.r#north == false
        {
            return 6777;
        }
        if self.r#west == false
            && self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == true
        {
            return 6785;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 6776;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 6794;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#south == true
            && self.r#west == true
        {
            return 6790;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#west == true
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 6772;
        }
        if self.r#west == false
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#north == true
            && self.r#south == true
        {
            return 6765;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#south == false
        {
            return 6784;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 6774;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#west == true
            && self.r#waterlogged == false
        {
            return 6766;
        }
        if self.r#waterlogged == true
            && self.r#south == true
            && self.r#north == true
            && self.r#west == false
            && self.r#east == false
        {
            return 6781;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == true
            && self.r#east == false
            && self.r#south == false
        {
            return 6786;
        }
        if self.r#west == false
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#north == false
            && self.r#east == false
        {
            return 6793;
        }
        if self.r#north == false
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 6789;
        }
        if self.r#west == false
            && self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#south == false
        {
            return 6787;
        }
        if self.r#west == true
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#east == true
            && self.r#south == false
        {
            return 6768;
        }
        if self.r#north == false
            && self.r#east == false
            && self.r#south == false
            && self.r#west == true
            && self.r#waterlogged == true
        {
            return 6792;
        }
        if self.r#waterlogged == true
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
            && self.r#south == false
        {
            return 6769;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == false
        {
            return 6773;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 6767;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#north == false
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 6779;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#east == false
            && self.r#west == false
        {
            return 6795;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == true
        {
            return 6780;
        }
        if self.r#west == true
            && self.r#south == false
            && self.r#north == true
            && self.r#east == true
            && self.r#waterlogged == false
        {
            return 6770;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#west == false
        {
            return 6791;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == true
        {
            return 6782;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 6788;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#north == true
            && self.r#west == true
            && self.r#waterlogged == true
        {
            return 6764;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
        {
            return 6778;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6783 {
            return Some(OakFence {
                r#west: false,
                r#east: false,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6771 {
            return Some(OakFence {
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 6775 {
            return Some(OakFence {
                r#west: false,
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6777 {
            return Some(OakFence {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 6785 {
            return Some(OakFence {
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6776 {
            return Some(OakFence {
                r#south: false,
                r#west: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6794 {
            return Some(OakFence {
                r#east: false,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 6790 {
            return Some(OakFence {
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 6772 {
            return Some(OakFence {
                r#south: true,
                r#east: true,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6765 {
            return Some(OakFence {
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 6784 {
            return Some(OakFence {
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 6774 {
            return Some(OakFence {
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 6766 {
            return Some(OakFence {
                r#east: true,
                r#north: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6781 {
            return Some(OakFence {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 6786 {
            return Some(OakFence {
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 6793 {
            return Some(OakFence {
                r#west: false,
                r#waterlogged: true,
                r#south: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 6789 {
            return Some(OakFence {
                r#north: false,
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 6787 {
            return Some(OakFence {
                r#west: false,
                r#north: true,
                r#east: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 6768 {
            return Some(OakFence {
                r#west: true,
                r#waterlogged: true,
                r#north: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 6792 {
            return Some(OakFence {
                r#north: false,
                r#east: false,
                r#south: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6769 {
            return Some(OakFence {
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 6773 {
            return Some(OakFence {
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 6767 {
            return Some(OakFence {
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 6779 {
            return Some(OakFence {
                r#east: true,
                r#south: false,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6795 {
            return Some(OakFence {
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 6780 {
            return Some(OakFence {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 6770 {
            return Some(OakFence {
                r#west: true,
                r#south: false,
                r#north: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6791 {
            return Some(OakFence {
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 6782 {
            return Some(OakFence {
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 6788 {
            return Some(OakFence {
                r#west: true,
                r#north: false,
                r#east: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6764 {
            return Some(OakFence {
                r#east: true,
                r#south: true,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6778 {
            return Some(OakFence {
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        return None;
    }
}
