use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangrovePropagule {
    pub stage: i32,
    pub age: i32,
    pub hanging: bool,
    pub waterlogged: bool,
}

impl BlockState for MangrovePropagule {
    fn to_id(&self) -> i32 {
        if self.r#age == 0
            && self.r#waterlogged == true
            && self.r#stage == 1
            && self.r#hanging == false
        {
            return 51;
        }
        if self.r#age == 3
            && self.r#stage == 0
            && self.r#waterlogged == true
            && self.r#hanging == true
        {
            return 69;
        }
        if self.r#age == 0
            && self.r#stage == 0
            && self.r#hanging == false
            && self.r#waterlogged == true
        {
            return 49;
        }
        if self.r#age == 4
            && self.r#stage == 1
            && self.r#hanging == true
            && self.r#waterlogged == false
        {
            return 80;
        }
        if self.r#waterlogged == true
            && self.r#hanging == false
            && self.r#age == 4
            && self.r#stage == 0
        {
            return 81;
        }
        if self.r#hanging == false
            && self.r#age == 4
            && self.r#stage == 0
            && self.r#waterlogged == false
        {
            return 82;
        }
        if self.r#hanging == true
            && self.r#waterlogged == false
            && self.r#age == 2
            && self.r#stage == 0
        {
            return 62;
        }
        if self.r#stage == 0
            && self.r#age == 3
            && self.r#hanging == true
            && self.r#waterlogged == false
        {
            return 70;
        }
        if self.r#stage == 1
            && self.r#waterlogged == false
            && self.r#age == 2
            && self.r#hanging == true
        {
            return 64;
        }
        if self.r#hanging == false
            && self.r#age == 3
            && self.r#stage == 1
            && self.r#waterlogged == true
        {
            return 75;
        }
        if self.r#age == 1
            && self.r#stage == 1
            && self.r#waterlogged == true
            && self.r#hanging == false
        {
            return 59;
        }
        if self.r#stage == 1
            && self.r#waterlogged == false
            && self.r#hanging == false
            && self.r#age == 3
        {
            return 76;
        }
        if self.r#hanging == true
            && self.r#stage == 0
            && self.r#age == 4
            && self.r#waterlogged == false
        {
            return 78;
        }
        if self.r#age == 3
            && self.r#waterlogged == false
            && self.r#hanging == true
            && self.r#stage == 1
        {
            return 72;
        }
        if self.r#waterlogged == false
            && self.r#age == 0
            && self.r#hanging == false
            && self.r#stage == 0
        {
            return 50;
        }
        if self.r#waterlogged == true
            && self.r#hanging == false
            && self.r#stage == 1
            && self.r#age == 2
        {
            return 67;
        }
        if self.r#waterlogged == false
            && self.r#stage == 1
            && self.r#age == 2
            && self.r#hanging == false
        {
            return 68;
        }
        if self.r#hanging == false
            && self.r#age == 3
            && self.r#waterlogged == false
            && self.r#stage == 0
        {
            return 74;
        }
        if self.r#age == 4
            && self.r#hanging == false
            && self.r#stage == 1
            && self.r#waterlogged == true
        {
            return 83;
        }
        if self.r#waterlogged == false
            && self.r#stage == 0
            && self.r#hanging == true
            && self.r#age == 0
        {
            return 46;
        }
        if self.r#waterlogged == true
            && self.r#age == 4
            && self.r#hanging == true
            && self.r#stage == 1
        {
            return 79;
        }
        if self.r#waterlogged == true
            && self.r#stage == 1
            && self.r#age == 1
            && self.r#hanging == true
        {
            return 55;
        }
        if self.r#waterlogged == true
            && self.r#hanging == true
            && self.r#age == 1
            && self.r#stage == 0
        {
            return 53;
        }
        if self.r#hanging == true
            && self.r#waterlogged == true
            && self.r#stage == 1
            && self.r#age == 3
        {
            return 71;
        }
        if self.r#age == 2
            && self.r#waterlogged == true
            && self.r#stage == 1
            && self.r#hanging == true
        {
            return 63;
        }
        if self.r#age == 1
            && self.r#stage == 1
            && self.r#hanging == false
            && self.r#waterlogged == false
        {
            return 60;
        }
        if self.r#waterlogged == true
            && self.r#stage == 0
            && self.r#age == 0
            && self.r#hanging == true
        {
            return 45;
        }
        if self.r#stage == 0
            && self.r#waterlogged == true
            && self.r#age == 3
            && self.r#hanging == false
        {
            return 73;
        }
        if self.r#stage == 1
            && self.r#age == 4
            && self.r#waterlogged == false
            && self.r#hanging == false
        {
            return 84;
        }
        if self.r#age == 2
            && self.r#hanging == false
            && self.r#waterlogged == false
            && self.r#stage == 0
        {
            return 66;
        }
        if self.r#age == 0
            && self.r#hanging == true
            && self.r#stage == 1
            && self.r#waterlogged == false
        {
            return 48;
        }
        if self.r#hanging == false
            && self.r#waterlogged == false
            && self.r#age == 1
            && self.r#stage == 0
        {
            return 58;
        }
        if self.r#hanging == true
            && self.r#age == 2
            && self.r#stage == 0
            && self.r#waterlogged == true
        {
            return 61;
        }
        if self.r#age == 4
            && self.r#waterlogged == true
            && self.r#stage == 0
            && self.r#hanging == true
        {
            return 77;
        }
        if self.r#age == 0
            && self.r#waterlogged == false
            && self.r#hanging == false
            && self.r#stage == 1
        {
            return 52;
        }
        if self.r#hanging == false
            && self.r#age == 2
            && self.r#stage == 0
            && self.r#waterlogged == true
        {
            return 65;
        }
        if self.r#hanging == false
            && self.r#age == 1
            && self.r#waterlogged == true
            && self.r#stage == 0
        {
            return 57;
        }
        if self.r#hanging == true
            && self.r#stage == 1
            && self.r#waterlogged == true
            && self.r#age == 0
        {
            return 47;
        }
        if self.r#stage == 0
            && self.r#waterlogged == false
            && self.r#age == 1
            && self.r#hanging == true
        {
            return 54;
        }
        if self.r#age == 1
            && self.r#hanging == true
            && self.r#stage == 1
            && self.r#waterlogged == false
        {
            return 56;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 51 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#waterlogged: true,
                r#stage: 1,
                r#hanging: false,
            });
        }
        if state_id == 69 {
            return Some(MangrovePropagule {
                r#age: 3,
                r#stage: 0,
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        if state_id == 49 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#stage: 0,
                r#hanging: false,
                r#waterlogged: true,
            });
        }
        if state_id == 80 {
            return Some(MangrovePropagule {
                r#age: 4,
                r#stage: 1,
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 81 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#hanging: false,
                r#age: 4,
                r#stage: 0,
            });
        }
        if state_id == 82 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#age: 4,
                r#stage: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 62 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#waterlogged: false,
                r#age: 2,
                r#stage: 0,
            });
        }
        if state_id == 70 {
            return Some(MangrovePropagule {
                r#stage: 0,
                r#age: 3,
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 64 {
            return Some(MangrovePropagule {
                r#stage: 1,
                r#waterlogged: false,
                r#age: 2,
                r#hanging: true,
            });
        }
        if state_id == 75 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#age: 3,
                r#stage: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 59 {
            return Some(MangrovePropagule {
                r#age: 1,
                r#stage: 1,
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 76 {
            return Some(MangrovePropagule {
                r#stage: 1,
                r#waterlogged: false,
                r#hanging: false,
                r#age: 3,
            });
        }
        if state_id == 78 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#stage: 0,
                r#age: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 72 {
            return Some(MangrovePropagule {
                r#age: 3,
                r#waterlogged: false,
                r#hanging: true,
                r#stage: 1,
            });
        }
        if state_id == 50 {
            return Some(MangrovePropagule {
                r#waterlogged: false,
                r#age: 0,
                r#hanging: false,
                r#stage: 0,
            });
        }
        if state_id == 67 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#hanging: false,
                r#stage: 1,
                r#age: 2,
            });
        }
        if state_id == 68 {
            return Some(MangrovePropagule {
                r#waterlogged: false,
                r#stage: 1,
                r#age: 2,
                r#hanging: false,
            });
        }
        if state_id == 74 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#age: 3,
                r#waterlogged: false,
                r#stage: 0,
            });
        }
        if state_id == 83 {
            return Some(MangrovePropagule {
                r#age: 4,
                r#hanging: false,
                r#stage: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 46 {
            return Some(MangrovePropagule {
                r#waterlogged: false,
                r#stage: 0,
                r#hanging: true,
                r#age: 0,
            });
        }
        if state_id == 79 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#age: 4,
                r#hanging: true,
                r#stage: 1,
            });
        }
        if state_id == 55 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#stage: 1,
                r#age: 1,
                r#hanging: true,
            });
        }
        if state_id == 53 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#hanging: true,
                r#age: 1,
                r#stage: 0,
            });
        }
        if state_id == 71 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#waterlogged: true,
                r#stage: 1,
                r#age: 3,
            });
        }
        if state_id == 63 {
            return Some(MangrovePropagule {
                r#age: 2,
                r#waterlogged: true,
                r#stage: 1,
                r#hanging: true,
            });
        }
        if state_id == 60 {
            return Some(MangrovePropagule {
                r#age: 1,
                r#stage: 1,
                r#hanging: false,
                r#waterlogged: false,
            });
        }
        if state_id == 45 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#stage: 0,
                r#age: 0,
                r#hanging: true,
            });
        }
        if state_id == 73 {
            return Some(MangrovePropagule {
                r#stage: 0,
                r#waterlogged: true,
                r#age: 3,
                r#hanging: false,
            });
        }
        if state_id == 84 {
            return Some(MangrovePropagule {
                r#stage: 1,
                r#age: 4,
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        if state_id == 66 {
            return Some(MangrovePropagule {
                r#age: 2,
                r#hanging: false,
                r#waterlogged: false,
                r#stage: 0,
            });
        }
        if state_id == 48 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#hanging: true,
                r#stage: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 58 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#waterlogged: false,
                r#age: 1,
                r#stage: 0,
            });
        }
        if state_id == 61 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#age: 2,
                r#stage: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 77 {
            return Some(MangrovePropagule {
                r#age: 4,
                r#waterlogged: true,
                r#stage: 0,
                r#hanging: true,
            });
        }
        if state_id == 52 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#waterlogged: false,
                r#hanging: false,
                r#stage: 1,
            });
        }
        if state_id == 65 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#age: 2,
                r#stage: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 57 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#age: 1,
                r#waterlogged: true,
                r#stage: 0,
            });
        }
        if state_id == 47 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#stage: 1,
                r#waterlogged: true,
                r#age: 0,
            });
        }
        if state_id == 54 {
            return Some(MangrovePropagule {
                r#stage: 0,
                r#waterlogged: false,
                r#age: 1,
                r#hanging: true,
            });
        }
        if state_id == 56 {
            return Some(MangrovePropagule {
                r#age: 1,
                r#hanging: true,
                r#stage: 1,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
