use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrewingStand {
    pub has_bottle_2: bool,
    pub has_bottle_1: bool,
    pub has_bottle_0: bool,
}

impl BlockState for BrewingStand {
    fn to_id(&self) -> i32 {
        if self.r#has_bottle_2 == false
            && self.r#has_bottle_1 == true
            && self.r#has_bottle_0 == true
        {
            return 9252;
        }
        if self.r#has_bottle_0 == true
            && self.r#has_bottle_2 == true
            && self.r#has_bottle_1 == false
        {
            return 9253;
        }
        if self.r#has_bottle_2 == false
            && self.r#has_bottle_0 == true
            && self.r#has_bottle_1 == false
        {
            return 9254;
        }
        if self.r#has_bottle_0 == true && self.r#has_bottle_1 == true && self.r#has_bottle_2 == true
        {
            return 9251;
        }
        if self.r#has_bottle_2 == true
            && self.r#has_bottle_0 == false
            && self.r#has_bottle_1 == false
        {
            return 9257;
        }
        if self.r#has_bottle_1 == true
            && self.r#has_bottle_0 == false
            && self.r#has_bottle_2 == true
        {
            return 9255;
        }
        if self.r#has_bottle_2 == false
            && self.r#has_bottle_1 == false
            && self.r#has_bottle_0 == false
        {
            return 9258;
        }
        if self.r#has_bottle_0 == false
            && self.r#has_bottle_1 == true
            && self.r#has_bottle_2 == false
        {
            return 9256;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9252 {
            return Some(BrewingStand {
                r#has_bottle_2: false,
                r#has_bottle_1: true,
                r#has_bottle_0: true,
            });
        }
        if state_id == 9253 {
            return Some(BrewingStand {
                r#has_bottle_0: true,
                r#has_bottle_2: true,
                r#has_bottle_1: false,
            });
        }
        if state_id == 9254 {
            return Some(BrewingStand {
                r#has_bottle_2: false,
                r#has_bottle_0: true,
                r#has_bottle_1: false,
            });
        }
        if state_id == 9251 {
            return Some(BrewingStand {
                r#has_bottle_0: true,
                r#has_bottle_1: true,
                r#has_bottle_2: true,
            });
        }
        if state_id == 9257 {
            return Some(BrewingStand {
                r#has_bottle_2: true,
                r#has_bottle_0: false,
                r#has_bottle_1: false,
            });
        }
        if state_id == 9255 {
            return Some(BrewingStand {
                r#has_bottle_1: true,
                r#has_bottle_0: false,
                r#has_bottle_2: true,
            });
        }
        if state_id == 9258 {
            return Some(BrewingStand {
                r#has_bottle_2: false,
                r#has_bottle_1: false,
                r#has_bottle_0: false,
            });
        }
        if state_id == 9256 {
            return Some(BrewingStand {
                r#has_bottle_0: false,
                r#has_bottle_1: true,
                r#has_bottle_2: false,
            });
        }
        return None;
    }
}
