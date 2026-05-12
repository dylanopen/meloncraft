use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleHangingSign {
    pub waterlogged: bool,
    pub attached: bool,
    pub rotation: i32,
}


impl BlockState for JungleHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6036; }
        if block_state.r#rotation == 14 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6086; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 && block_state.r#attached == true { return 6056; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6026; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 6033; }
        if block_state.r#attached == true && block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 6047; }
        if block_state.r#rotation == 0 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6058; }
        if block_state.r#attached == false && block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 6078; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6065; }
        if block_state.r#rotation == 2 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6062; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 4 { return 6034; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6046; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 15 { return 6057; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 9 { return 6076; }
        if block_state.r#attached == true && block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 6027; }
        if block_state.r#rotation == 10 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6079; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6042; }
        if block_state.r#attached == true && block_state.r#rotation == 11 && block_state.r#waterlogged == false { return 6049; }
        if block_state.r#attached == false && block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 6069; }
        if block_state.r#attached == true && block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 6029; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6072; }
        if block_state.r#rotation == 9 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6077; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 13 { return 6084; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 && block_state.r#attached == true { return 6031; }
        if block_state.r#attached == true && block_state.r#rotation == 6 && block_state.r#waterlogged == true { return 6038; }
        if block_state.r#waterlogged == true && block_state.r#attached == false && block_state.r#rotation == 5 { return 6068; }
        if block_state.r#attached == false && block_state.r#rotation == 15 && block_state.r#waterlogged == false { return 6089; }
        if block_state.r#rotation == 11 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6081; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 14 && block_state.r#attached == false { return 6087; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6041; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 4 { return 6066; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true && block_state.r#attached == true { return 6044; }
        if block_state.r#waterlogged == false && block_state.r#attached == true && block_state.r#rotation == 12 { return 6051; }
        if block_state.r#attached == true && block_state.r#rotation == 2 && block_state.r#waterlogged == true { return 6030; }
        if block_state.r#attached == false && block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 6074; }
        if block_state.r#rotation == 7 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6073; }
        if block_state.r#rotation == 1 && block_state.r#attached == false && block_state.r#waterlogged == true { return 6060; }
        if block_state.r#rotation == 5 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6037; }
        if block_state.r#waterlogged == true && block_state.r#attached == true && block_state.r#rotation == 1 { return 6028; }
        if block_state.r#attached == false && block_state.r#rotation == 6 && block_state.r#waterlogged == true { return 6070; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6088; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 6032; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 6053; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 0 && block_state.r#attached == false { return 6059; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 && block_state.r#attached == false { return 6063; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false && block_state.r#attached == false { return 6083; }
        if block_state.r#attached == false && block_state.r#waterlogged == false && block_state.r#rotation == 6 { return 6071; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 12 { return 6050; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true && block_state.r#attached == false { return 6082; }
        if block_state.r#rotation == 8 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6043; }
        if block_state.r#rotation == 8 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6075; }
        if block_state.r#waterlogged == false && block_state.r#attached == false && block_state.r#rotation == 4 { return 6067; }
        if block_state.r#attached == false && block_state.r#rotation == 13 && block_state.r#waterlogged == false { return 6085; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 3 && block_state.r#attached == false { return 6064; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 7 { return 6040; }
        if block_state.r#rotation == 1 && block_state.r#attached == false && block_state.r#waterlogged == false { return 6061; }
        if block_state.r#attached == true && block_state.r#waterlogged == true && block_state.r#rotation == 14 { return 6054; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 && block_state.r#attached == true { return 6048; }
        if block_state.r#rotation == 13 && block_state.r#attached == true && block_state.r#waterlogged == true { return 6052; }
        if block_state.r#rotation == 14 && block_state.r#attached == true && block_state.r#waterlogged == false { return 6055; }
        if block_state.r#attached == true && block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 6035; }
        if block_state.r#attached == true && block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 6045; }
        if block_state.r#attached == false && block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 6080; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false && block_state.r#attached == true { return 6039; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6036 {
            return Some(JungleHangingSign {
                r#rotation: 5,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6086 {
            return Some(JungleHangingSign {
                r#rotation: 14,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6056 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#rotation: 15,
                r#attached: true,
            });
        }
        if state_id == 6026 {
            return Some(JungleHangingSign {
                r#rotation: 0,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6033 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 6047 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 6058 {
            return Some(JungleHangingSign {
                r#rotation: 0,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6078 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 6065 {
            return Some(JungleHangingSign {
                r#rotation: 3,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6062 {
            return Some(JungleHangingSign {
                r#rotation: 2,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6034 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 4,
            });
        }
        if state_id == 6046 {
            return Some(JungleHangingSign {
                r#rotation: 10,
                r#waterlogged: true,
                r#attached: true,
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
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 9,
            });
        }
        if state_id == 6027 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 6079 {
            return Some(JungleHangingSign {
                r#rotation: 10,
                r#attached: false,
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
        if state_id == 6049 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 6069 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 6029 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 6072 {
            return Some(JungleHangingSign {
                r#rotation: 7,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6077 {
            return Some(JungleHangingSign {
                r#rotation: 9,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6084 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 13,
            });
        }
        if state_id == 6031 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 2,
                r#attached: true,
            });
        }
        if state_id == 6038 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 6068 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: false,
                r#rotation: 5,
            });
        }
        if state_id == 6089 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 6081 {
            return Some(JungleHangingSign {
                r#rotation: 11,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6087 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 14,
                r#attached: false,
            });
        }
        if state_id == 6041 {
            return Some(JungleHangingSign {
                r#rotation: 7,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        if state_id == 6066 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 6044 {
            return Some(JungleHangingSign {
                r#rotation: 9,
                r#waterlogged: true,
                r#attached: true,
            });
        }
        if state_id == 6051 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: true,
                r#rotation: 12,
            });
        }
        if state_id == 6030 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 6074 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 6073 {
            return Some(JungleHangingSign {
                r#rotation: 7,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6060 {
            return Some(JungleHangingSign {
                r#rotation: 1,
                r#attached: false,
                r#waterlogged: true,
            });
        }
        if state_id == 6037 {
            return Some(JungleHangingSign {
                r#rotation: 5,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6028 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#attached: true,
                r#rotation: 1,
            });
        }
        if state_id == 6070 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 6,
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
        if state_id == 6032 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 6053 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 6059 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 0,
                r#attached: false,
            });
        }
        if state_id == 6063 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#rotation: 2,
                r#attached: false,
            });
        }
        if state_id == 6083 {
            return Some(JungleHangingSign {
                r#rotation: 12,
                r#waterlogged: false,
                r#attached: false,
            });
        }
        if state_id == 6071 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 6050 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 6082 {
            return Some(JungleHangingSign {
                r#rotation: 12,
                r#waterlogged: true,
                r#attached: false,
            });
        }
        if state_id == 6043 {
            return Some(JungleHangingSign {
                r#rotation: 8,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6075 {
            return Some(JungleHangingSign {
                r#rotation: 8,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6067 {
            return Some(JungleHangingSign {
                r#waterlogged: false,
                r#attached: false,
                r#rotation: 4,
            });
        }
        if state_id == 6085 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 6064 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#rotation: 3,
                r#attached: false,
            });
        }
        if state_id == 6040 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 6061 {
            return Some(JungleHangingSign {
                r#rotation: 1,
                r#attached: false,
                r#waterlogged: false,
            });
        }
        if state_id == 6054 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: true,
                r#rotation: 14,
            });
        }
        if state_id == 6048 {
            return Some(JungleHangingSign {
                r#waterlogged: true,
                r#rotation: 11,
                r#attached: true,
            });
        }
        if state_id == 6052 {
            return Some(JungleHangingSign {
                r#rotation: 13,
                r#attached: true,
                r#waterlogged: true,
            });
        }
        if state_id == 6055 {
            return Some(JungleHangingSign {
                r#rotation: 14,
                r#attached: true,
                r#waterlogged: false,
            });
        }
        if state_id == 6035 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 6045 {
            return Some(JungleHangingSign {
                r#attached: true,
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 6080 {
            return Some(JungleHangingSign {
                r#attached: false,
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 6039 {
            return Some(JungleHangingSign {
                r#rotation: 6,
                r#waterlogged: false,
                r#attached: true,
            });
        }
        return None;
    }
}

