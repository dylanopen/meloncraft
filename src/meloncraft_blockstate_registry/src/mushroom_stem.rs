use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MushroomStem {
    pub down: bool,
    pub up: bool,
    pub west: bool,
    pub east: bool,
    pub south: bool,
    pub north: bool,
}

impl BlockState for MushroomStem {
    fn to_id(&self) -> i32 {
        if self.r#east == false
            && self.r#west == true
            && self.r#north == true
            && self.r#down == true
            && self.r#south == false
            && self.r#up == false
        {
            return 7715;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == true
            && self.r#down == true
            && self.r#up == false
            && self.r#west == false
        {
            return 7696;
        }
        if self.r#east == true
            && self.r#up == false
            && self.r#west == true
            && self.r#north == true
            && self.r#south == true
            && self.r#down == true
        {
            return 7695;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#up == false
            && self.r#down == true
            && self.r#north == false
            && self.r#west == true
        {
            return 7723;
        }
        if self.r#south == true
            && self.r#up == true
            && self.r#north == false
            && self.r#east == false
            && self.r#down == false
            && self.r#west == false
        {
            return 7750;
        }
        if self.r#down == false
            && self.r#south == false
            && self.r#west == true
            && self.r#up == true
            && self.r#east == true
            && self.r#north == false
        {
            return 7737;
        }
        if self.r#east == false
            && self.r#west == true
            && self.r#down == true
            && self.r#north == false
            && self.r#south == false
            && self.r#up == true
        {
            return 7721;
        }
        if self.r#west == true
            && self.r#down == true
            && self.r#south == false
            && self.r#north == false
            && self.r#east == true
            && self.r#up == true
        {
            return 7705;
        }
        if self.r#south == true
            && self.r#up == false
            && self.r#west == false
            && self.r#down == false
            && self.r#north == true
            && self.r#east == true
        {
            return 7728;
        }
        if self.r#north == true
            && self.r#down == false
            && self.r#up == false
            && self.r#east == false
            && self.r#south == false
            && self.r#west == false
        {
            return 7748;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#south == true
            && self.r#west == true
            && self.r#up == true
            && self.r#down == true
        {
            return 7709;
        }
        if self.r#east == false
            && self.r#down == false
            && self.r#up == false
            && self.r#north == false
            && self.r#south == true
            && self.r#west == true
        {
            return 7751;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#east == true
            && self.r#south == true
            && self.r#up == false
            && self.r#down == true
        {
            return 7704;
        }
        if self.r#up == true
            && self.r#west == true
            && self.r#south == false
            && self.r#east == false
            && self.r#north == true
            && self.r#down == false
        {
            return 7745;
        }
        if self.r#up == true
            && self.r#down == true
            && self.r#west == false
            && self.r#south == false
            && self.r#east == false
            && self.r#north == true
        {
            return 7714;
        }
        if self.r#down == false
            && self.r#up == false
            && self.r#west == true
            && self.r#north == false
            && self.r#south == true
            && self.r#east == true
        {
            return 7735;
        }
        if self.r#up == true
            && self.r#east == false
            && self.r#north == false
            && self.r#west == false
            && self.r#down == true
            && self.r#south == true
        {
            return 7718;
        }
        if self.r#east == false
            && self.r#down == true
            && self.r#west == false
            && self.r#south == true
            && self.r#up == false
            && self.r#north == true
        {
            return 7712;
        }
        if self.r#east == true
            && self.r#west == false
            && self.r#down == true
            && self.r#up == true
            && self.r#north == false
            && self.r#south == false
        {
            return 7706;
        }
        if self.r#west == true
            && self.r#down == false
            && self.r#up == true
            && self.r#east == false
            && self.r#north == true
            && self.r#south == true
        {
            return 7741;
        }
        if self.r#east == true
            && self.r#west == true
            && self.r#down == false
            && self.r#north == false
            && self.r#south == true
            && self.r#up == true
        {
            return 7733;
        }
        if self.r#down == false
            && self.r#north == true
            && self.r#east == true
            && self.r#west == true
            && self.r#up == true
            && self.r#south == true
        {
            return 7725;
        }
        if self.r#down == false
            && self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#west == true
            && self.r#up == false
        {
            return 7747;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#down == false
            && self.r#south == true
            && self.r#east == true
            && self.r#up == false
        {
            return 7736;
        }
        if self.r#down == false
            && self.r#west == false
            && self.r#north == false
            && self.r#south == false
            && self.r#east == true
            && self.r#up == false
        {
            return 7740;
        }
        if self.r#down == false
            && self.r#north == true
            && self.r#up == true
            && self.r#east == true
            && self.r#west == true
            && self.r#south == false
        {
            return 7729;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#east == false
            && self.r#down == false
            && self.r#up == true
            && self.r#south == false
        {
            return 7754;
        }
        if self.r#down == true
            && self.r#up == true
            && self.r#south == true
            && self.r#east == true
            && self.r#west == false
            && self.r#north == false
        {
            return 7702;
        }
        if self.r#up == true
            && self.r#east == false
            && self.r#west == false
            && self.r#down == false
            && self.r#north == true
            && self.r#south == false
        {
            return 7746;
        }
        if self.r#west == true
            && self.r#down == true
            && self.r#south == true
            && self.r#east == true
            && self.r#north == false
            && self.r#up == false
        {
            return 7703;
        }
        if self.r#south == true
            && self.r#up == true
            && self.r#down == true
            && self.r#north == true
            && self.r#west == false
            && self.r#east == false
        {
            return 7710;
        }
        if self.r#down == true
            && self.r#east == false
            && self.r#north == false
            && self.r#south == false
            && self.r#up == true
            && self.r#west == false
        {
            return 7722;
        }
        if self.r#east == false
            && self.r#down == true
            && self.r#south == false
            && self.r#up == false
            && self.r#west == false
            && self.r#north == false
        {
            return 7724;
        }
        if self.r#down == false
            && self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#up == false
            && self.r#west == true
        {
            return 7727;
        }
        if self.r#down == false
            && self.r#east == false
            && self.r#south == false
            && self.r#west == true
            && self.r#up == true
            && self.r#north == false
        {
            return 7753;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#up == true
            && self.r#west == false
            && self.r#down == false
            && self.r#north == false
        {
            return 7734;
        }
        if self.r#down == false
            && self.r#north == false
            && self.r#east == false
            && self.r#south == true
            && self.r#west == false
            && self.r#up == false
        {
            return 7752;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#up == false
            && self.r#down == true
            && self.r#north == false
            && self.r#west == true
        {
            return 7707;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#down == false
            && self.r#south == false
            && self.r#west == false
            && self.r#up == false
        {
            return 7756;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#down == false
            && self.r#up == true
            && self.r#west == false
        {
            return 7742;
        }
        if self.r#east == false
            && self.r#up == true
            && self.r#north == false
            && self.r#down == true
            && self.r#south == true
            && self.r#west == true
        {
            return 7717;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#south == false
            && self.r#up == true
            && self.r#north == true
            && self.r#down == true
        {
            return 7698;
        }
        if self.r#down == false
            && self.r#north == false
            && self.r#south == false
            && self.r#west == true
            && self.r#east == false
            && self.r#up == false
        {
            return 7755;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#down == true
            && self.r#west == false
            && self.r#north == true
            && self.r#up == false
        {
            return 7700;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#up == true
            && self.r#east == true
            && self.r#west == false
            && self.r#down == true
        {
            return 7694;
        }
        if self.r#west == true
            && self.r#north == true
            && self.r#east == true
            && self.r#up == false
            && self.r#down == true
            && self.r#south == false
        {
            return 7699;
        }
        if self.r#down == false
            && self.r#south == false
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
            && self.r#up == false
        {
            return 7732;
        }
        if self.r#south == false
            && self.r#up == true
            && self.r#west == false
            && self.r#east == true
            && self.r#north == false
            && self.r#down == false
        {
            return 7738;
        }
        if self.r#south == true
            && self.r#down == true
            && self.r#up == false
            && self.r#east == false
            && self.r#west == true
            && self.r#north == false
        {
            return 7719;
        }
        if self.r#down == false
            && self.r#up == false
            && self.r#east == true
            && self.r#south == false
            && self.r#west == true
            && self.r#north == true
        {
            return 7731;
        }
        if self.r#down == false
            && self.r#up == false
            && self.r#north == false
            && self.r#south == false
            && self.r#west == true
            && self.r#east == true
        {
            return 7739;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#east == false
            && self.r#west == false
            && self.r#down == true
            && self.r#up == false
        {
            return 7716;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#up == false
            && self.r#down == false
            && self.r#west == true
            && self.r#south == true
        {
            return 7743;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#down == true
            && self.r#south == true
            && self.r#north == true
            && self.r#up == false
        {
            return 7711;
        }
        if self.r#east == true
            && self.r#down == true
            && self.r#north == true
            && self.r#south == true
            && self.r#up == true
            && self.r#west == true
        {
            return 7693;
        }
        if self.r#north == false
            && self.r#up == true
            && self.r#west == true
            && self.r#east == false
            && self.r#south == true
            && self.r#down == false
        {
            return 7749;
        }
        if self.r#north == false
            && self.r#down == true
            && self.r#west == false
            && self.r#south == false
            && self.r#east == true
            && self.r#up == false
        {
            return 7708;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#down == false
            && self.r#north == true
            && self.r#south == true
            && self.r#up == true
        {
            return 7726;
        }
        if self.r#east == true
            && self.r#down == true
            && self.r#south == true
            && self.r#north == false
            && self.r#up == true
            && self.r#west == true
        {
            return 7701;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#down == true
            && self.r#up == true
            && self.r#west == true
            && self.r#east == true
        {
            return 7697;
        }
        if self.r#east == true
            && self.r#down == false
            && self.r#south == false
            && self.r#up == true
            && self.r#west == false
            && self.r#north == true
        {
            return 7730;
        }
        if self.r#down == false
            && self.r#east == false
            && self.r#up == false
            && self.r#north == true
            && self.r#south == true
            && self.r#west == false
        {
            return 7744;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#up == true
            && self.r#down == true
            && self.r#west == true
            && self.r#east == false
        {
            return 7713;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#up == false
            && self.r#west == false
            && self.r#down == true
            && self.r#south == true
        {
            return 7720;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7715 {
            return Some(MushroomStem {
                r#east: false,
                r#west: true,
                r#north: true,
                r#down: true,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 7696 {
            return Some(MushroomStem {
                r#north: true,
                r#south: true,
                r#east: true,
                r#down: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 7695 {
            return Some(MushroomStem {
                r#east: true,
                r#up: false,
                r#west: true,
                r#north: true,
                r#south: true,
                r#down: true,
            });
        }
        if state_id == 7723 {
            return Some(MushroomStem {
                r#east: false,
                r#south: false,
                r#up: false,
                r#down: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 7750 {
            return Some(MushroomStem {
                r#south: true,
                r#up: true,
                r#north: false,
                r#east: false,
                r#down: false,
                r#west: false,
            });
        }
        if state_id == 7737 {
            return Some(MushroomStem {
                r#down: false,
                r#south: false,
                r#west: true,
                r#up: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 7721 {
            return Some(MushroomStem {
                r#east: false,
                r#west: true,
                r#down: true,
                r#north: false,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 7705 {
            return Some(MushroomStem {
                r#west: true,
                r#down: true,
                r#south: false,
                r#north: false,
                r#east: true,
                r#up: true,
            });
        }
        if state_id == 7728 {
            return Some(MushroomStem {
                r#south: true,
                r#up: false,
                r#west: false,
                r#down: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 7748 {
            return Some(MushroomStem {
                r#north: true,
                r#down: false,
                r#up: false,
                r#east: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 7709 {
            return Some(MushroomStem {
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: true,
                r#up: true,
                r#down: true,
            });
        }
        if state_id == 7751 {
            return Some(MushroomStem {
                r#east: false,
                r#down: false,
                r#up: false,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 7704 {
            return Some(MushroomStem {
                r#west: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#up: false,
                r#down: true,
            });
        }
        if state_id == 7745 {
            return Some(MushroomStem {
                r#up: true,
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: true,
                r#down: false,
            });
        }
        if state_id == 7714 {
            return Some(MushroomStem {
                r#up: true,
                r#down: true,
                r#west: false,
                r#south: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 7735 {
            return Some(MushroomStem {
                r#down: false,
                r#up: false,
                r#west: true,
                r#north: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 7718 {
            return Some(MushroomStem {
                r#up: true,
                r#east: false,
                r#north: false,
                r#west: false,
                r#down: true,
                r#south: true,
            });
        }
        if state_id == 7712 {
            return Some(MushroomStem {
                r#east: false,
                r#down: true,
                r#west: false,
                r#south: true,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 7706 {
            return Some(MushroomStem {
                r#east: true,
                r#west: false,
                r#down: true,
                r#up: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 7741 {
            return Some(MushroomStem {
                r#west: true,
                r#down: false,
                r#up: true,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 7733 {
            return Some(MushroomStem {
                r#east: true,
                r#west: true,
                r#down: false,
                r#north: false,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 7725 {
            return Some(MushroomStem {
                r#down: false,
                r#north: true,
                r#east: true,
                r#west: true,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 7747 {
            return Some(MushroomStem {
                r#down: false,
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 7736 {
            return Some(MushroomStem {
                r#west: false,
                r#north: false,
                r#down: false,
                r#south: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 7740 {
            return Some(MushroomStem {
                r#down: false,
                r#west: false,
                r#north: false,
                r#south: false,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 7729 {
            return Some(MushroomStem {
                r#down: false,
                r#north: true,
                r#up: true,
                r#east: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 7754 {
            return Some(MushroomStem {
                r#west: false,
                r#north: false,
                r#east: false,
                r#down: false,
                r#up: true,
                r#south: false,
            });
        }
        if state_id == 7702 {
            return Some(MushroomStem {
                r#down: true,
                r#up: true,
                r#south: true,
                r#east: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 7746 {
            return Some(MushroomStem {
                r#up: true,
                r#east: false,
                r#west: false,
                r#down: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 7703 {
            return Some(MushroomStem {
                r#west: true,
                r#down: true,
                r#south: true,
                r#east: true,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 7710 {
            return Some(MushroomStem {
                r#south: true,
                r#up: true,
                r#down: true,
                r#north: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 7722 {
            return Some(MushroomStem {
                r#down: true,
                r#east: false,
                r#north: false,
                r#south: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 7724 {
            return Some(MushroomStem {
                r#east: false,
                r#down: true,
                r#south: false,
                r#up: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 7727 {
            return Some(MushroomStem {
                r#down: false,
                r#east: true,
                r#north: true,
                r#south: true,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 7753 {
            return Some(MushroomStem {
                r#down: false,
                r#east: false,
                r#south: false,
                r#west: true,
                r#up: true,
                r#north: false,
            });
        }
        if state_id == 7734 {
            return Some(MushroomStem {
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#down: false,
                r#north: false,
            });
        }
        if state_id == 7752 {
            return Some(MushroomStem {
                r#down: false,
                r#north: false,
                r#east: false,
                r#south: true,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 7707 {
            return Some(MushroomStem {
                r#east: true,
                r#south: false,
                r#up: false,
                r#down: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 7756 {
            return Some(MushroomStem {
                r#east: false,
                r#north: false,
                r#down: false,
                r#south: false,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 7742 {
            return Some(MushroomStem {
                r#north: true,
                r#south: true,
                r#east: false,
                r#down: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 7717 {
            return Some(MushroomStem {
                r#east: false,
                r#up: true,
                r#north: false,
                r#down: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 7698 {
            return Some(MushroomStem {
                r#west: false,
                r#east: true,
                r#south: false,
                r#up: true,
                r#north: true,
                r#down: true,
            });
        }
        if state_id == 7755 {
            return Some(MushroomStem {
                r#down: false,
                r#north: false,
                r#south: false,
                r#west: true,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 7700 {
            return Some(MushroomStem {
                r#east: true,
                r#south: false,
                r#down: true,
                r#west: false,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 7694 {
            return Some(MushroomStem {
                r#north: true,
                r#south: true,
                r#up: true,
                r#east: true,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 7699 {
            return Some(MushroomStem {
                r#west: true,
                r#north: true,
                r#east: true,
                r#up: false,
                r#down: true,
                r#south: false,
            });
        }
        if state_id == 7732 {
            return Some(MushroomStem {
                r#down: false,
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 7738 {
            return Some(MushroomStem {
                r#south: false,
                r#up: true,
                r#west: false,
                r#east: true,
                r#north: false,
                r#down: false,
            });
        }
        if state_id == 7719 {
            return Some(MushroomStem {
                r#south: true,
                r#down: true,
                r#up: false,
                r#east: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 7731 {
            return Some(MushroomStem {
                r#down: false,
                r#up: false,
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7739 {
            return Some(MushroomStem {
                r#down: false,
                r#up: false,
                r#north: false,
                r#south: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 7716 {
            return Some(MushroomStem {
                r#south: false,
                r#north: true,
                r#east: false,
                r#west: false,
                r#down: true,
                r#up: false,
            });
        }
        if state_id == 7743 {
            return Some(MushroomStem {
                r#east: false,
                r#north: true,
                r#up: false,
                r#down: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 7711 {
            return Some(MushroomStem {
                r#west: true,
                r#east: false,
                r#down: true,
                r#south: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 7693 {
            return Some(MushroomStem {
                r#east: true,
                r#down: true,
                r#north: true,
                r#south: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 7749 {
            return Some(MushroomStem {
                r#north: false,
                r#up: true,
                r#west: true,
                r#east: false,
                r#south: true,
                r#down: false,
            });
        }
        if state_id == 7708 {
            return Some(MushroomStem {
                r#north: false,
                r#down: true,
                r#west: false,
                r#south: false,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 7726 {
            return Some(MushroomStem {
                r#west: false,
                r#east: true,
                r#down: false,
                r#north: true,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 7701 {
            return Some(MushroomStem {
                r#east: true,
                r#down: true,
                r#south: true,
                r#north: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 7697 {
            return Some(MushroomStem {
                r#north: true,
                r#south: false,
                r#down: true,
                r#up: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 7730 {
            return Some(MushroomStem {
                r#east: true,
                r#down: false,
                r#south: false,
                r#up: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 7744 {
            return Some(MushroomStem {
                r#down: false,
                r#east: false,
                r#up: false,
                r#north: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7713 {
            return Some(MushroomStem {
                r#north: true,
                r#south: false,
                r#up: true,
                r#down: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 7720 {
            return Some(MushroomStem {
                r#east: false,
                r#north: false,
                r#up: false,
                r#west: false,
                r#down: true,
                r#south: true,
            });
        }
        return None;
    }
}
