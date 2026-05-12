use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleHangingSign {
    pub rotation: i32,
    pub waterlogged: bool,
    pub attached: bool,
}


impl BlockState for JungleHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#attached == false && self.r#rotation == 2 && self.r#waterlogged == false { return 6063; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 12 { return 6050; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 7 { return 6073; }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 0 { return 6058; }
        if self.r#rotation == 8 && self.r#waterlogged == true && self.r#attached == false { return 6074; }
        if self.r#waterlogged == false && self.r#rotation == 2 && self.r#attached == true { return 6031; }
        if self.r#waterlogged == false && self.r#rotation == 9 && self.r#attached == true { return 6045; }
        if self.r#attached == true && self.r#rotation == 10 && self.r#waterlogged == false { return 6047; }
        if self.r#rotation == 11 && self.r#waterlogged == true && self.r#attached == true { return 6048; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 10 { return 6046; }
        if self.r#attached == false && self.r#waterlogged == true && self.r#rotation == 5 { return 6068; }
        if self.r#rotation == 3 && self.r#waterlogged == true && self.r#attached == false { return 6064; }
        if self.r#rotation == 0 && self.r#attached == true && self.r#waterlogged == true { return 6026; }
        if self.r#rotation == 1 && self.r#waterlogged == false && self.r#attached == false { return 6061; }
        if self.r#attached == false && self.r#rotation == 4 && self.r#waterlogged == false { return 6067; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 12 { return 6083; }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 3 { return 6032; }
        if self.r#attached == true && self.r#rotation == 1 && self.r#waterlogged == true { return 6028; }
        if self.r#rotation == 4 && self.r#attached == true && self.r#waterlogged == true { return 6034; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 11 { return 6081; }
        if self.r#rotation == 14 && self.r#waterlogged == false && self.r#attached == false { return 6087; }
        if self.r#attached == true && self.r#rotation == 3 && self.r#waterlogged == false { return 6033; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 7 { return 6041; }
        if self.r#waterlogged == false && self.r#rotation == 6 && self.r#attached == false { return 6071; }
        if self.r#rotation == 9 && self.r#waterlogged == true && self.r#attached == true { return 6044; }
        if self.r#attached == false && self.r#rotation == 7 && self.r#waterlogged == true { return 6072; }
        if self.r#attached == false && self.r#waterlogged == false && self.r#rotation == 3 { return 6065; }
        if self.r#rotation == 11 && self.r#attached == true && self.r#waterlogged == false { return 6049; }
        if self.r#rotation == 8 && self.r#waterlogged == true && self.r#attached == true { return 6042; }
        if self.r#rotation == 1 && self.r#attached == false && self.r#waterlogged == true { return 6060; }
        if self.r#attached == true && self.r#waterlogged == false && self.r#rotation == 1 { return 6029; }
        if self.r#rotation == 5 && self.r#waterlogged == true && self.r#attached == true { return 6036; }
        if self.r#waterlogged == true && self.r#attached == true && self.r#rotation == 13 { return 6052; }
        if self.r#rotation == 13 && self.r#waterlogged == false && self.r#attached == true { return 6053; }
        if self.r#attached == true && self.r#rotation == 7 && self.r#waterlogged == true { return 6040; }
        if self.r#rotation == 14 && self.r#attached == true && self.r#waterlogged == true { return 6054; }
        if self.r#attached == false && self.r#rotation == 0 && self.r#waterlogged == false { return 6059; }
        if self.r#rotation == 0 && self.r#attached == true && self.r#waterlogged == false { return 6027; }
        if self.r#attached == false && self.r#waterlogged == true && self.r#rotation == 2 { return 6062; }
        if self.r#attached == false && self.r#rotation == 4 && self.r#waterlogged == true { return 6066; }
        if self.r#attached == true && self.r#rotation == 5 && self.r#waterlogged == false { return 6037; }
        if self.r#attached == true && self.r#waterlogged == true && self.r#rotation == 15 { return 6056; }
        if self.r#waterlogged == false && self.r#rotation == 14 && self.r#attached == true { return 6055; }
        if self.r#attached == true && self.r#rotation == 8 && self.r#waterlogged == false { return 6043; }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 12 { return 6082; }
        if self.r#attached == false && self.r#rotation == 10 && self.r#waterlogged == true { return 6078; }
        if self.r#rotation == 15 && self.r#waterlogged == true && self.r#attached == false { return 6088; }
        if self.r#rotation == 6 && self.r#attached == true && self.r#waterlogged == true { return 6038; }
        if self.r#rotation == 5 && self.r#waterlogged == false && self.r#attached == false { return 6069; }
        if self.r#attached == false && self.r#waterlogged == true && self.r#rotation == 13 { return 6084; }
        if self.r#attached == true && self.r#waterlogged == false && self.r#rotation == 6 { return 6039; }
        if self.r#rotation == 2 && self.r#waterlogged == true && self.r#attached == true { return 6030; }
        if self.r#rotation == 13 && self.r#waterlogged == false && self.r#attached == false { return 6085; }
        if self.r#attached == false && self.r#rotation == 9 && self.r#waterlogged == false { return 6077; }
        if self.r#waterlogged == true && self.r#attached == false && self.r#rotation == 6 { return 6070; }
        if self.r#attached == false && self.r#rotation == 14 && self.r#waterlogged == true { return 6086; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 15 { return 6089; }
        if self.r#waterlogged == false && self.r#attached == true && self.r#rotation == 15 { return 6057; }
        if self.r#attached == false && self.r#rotation == 9 && self.r#waterlogged == true { return 6076; }
        if self.r#waterlogged == false && self.r#rotation == 12 && self.r#attached == true { return 6051; }
        if self.r#rotation == 11 && self.r#waterlogged == true && self.r#attached == false { return 6080; }
        if self.r#waterlogged == false && self.r#attached == false && self.r#rotation == 10 { return 6079; }
        if self.r#attached == true && self.r#rotation == 4 && self.r#waterlogged == false { return 6035; }
        if self.r#attached == false && self.r#rotation == 8 && self.r#waterlogged == false { return 6075; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6063 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 6050 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 6073 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 7,
            });
        }
        if state_id == 6058 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 0,
            });
        }
        if state_id == 6074 {
            return Some(JungleHangingSign {
                r#rotation: 8,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6031 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 2,
                r#attached: true,
            });
        }
        if state_id == 6045 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 9,
                r#attached: true,
            });
        }
        if state_id == 6047 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 6048 {
            return Some(JungleHangingSign {
                r#rotation: 11,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6046 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 10,
            });
        }
        if state_id == 6068 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 5,
            });
        }
        if state_id == 6064 {
            return Some(JungleHangingSign {
                r#rotation: 3,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6026 {
            return Some(JungleHangingSign {
                r#rotation: 0,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6061 {
            return Some(JungleHangingSign {
                r#rotation: 1,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6067 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 6083 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 12,
            });
        }
        if state_id == 6032 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 3,
            });
        }
        if state_id == 6028 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 6034 {
            return Some(JungleHangingSign {
                r#rotation: 4,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6081 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 11,
            });
        }
        if state_id == 6087 {
            return Some(JungleHangingSign {
                r#rotation: 14,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6033 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 6041 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 7,
            });
        }
        if state_id == 6071 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 6,
                r#attached: false,
            });
        }
        if state_id == 6044 {
            return Some(JungleHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6072 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 6065 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 6049 {
            return Some(JungleHangingSign {
                r#rotation: 11,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6042 {
            return Some(JungleHangingSign {
                r#rotation: 8,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6060 {
            return Some(JungleHangingSign {
                r#rotation: 1,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6029 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 6036 {
            return Some(JungleHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6052 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 13,
            });
        }
        if state_id == 6053 {
            return Some(JungleHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6040 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 6054 {
            return Some(JungleHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6059 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 6027 {
            return Some(JungleHangingSign {
                r#rotation: 0,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6062 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 6066 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 6037 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 6056 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 6055 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 14,
                r#attached: true,
            });
        }
        if state_id == 6043 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 6082 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 12,
            });
        }
        if state_id == 6078 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 6088 {
            return Some(JungleHangingSign {
                r#rotation: 15,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6038 {
            return Some(JungleHangingSign {
                r#rotation: 6,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6069 {
            return Some(JungleHangingSign {
                r#rotation: 5,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6084 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 6039 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 6030 {
            return Some(JungleHangingSign {
                r#rotation: 2,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6085 {
            return Some(JungleHangingSign {
                r#rotation: 13,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6077 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 6070 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 6,
            });
        }
        if state_id == 6086 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 6089 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 15,
            });
        }
        if state_id == 6057 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 15,
            });
        }
        if state_id == 6076 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 6051 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 12,
                r#attached: true,
            });
        }
        if state_id == 6080 {
            return Some(JungleHangingSign {
                r#rotation: 11,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6079 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 10,
            });
        }
        if state_id == 6035 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 6075 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

