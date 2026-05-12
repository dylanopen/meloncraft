use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureBlock {
    pub r#mode: Mode,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Save,
    Load,
    Corner,
    Data,
}

impl BlockState for StructureBlock {
    fn to_id(self) -> i32 {
        if block_state.r#mode == Mode::Corner { return 21522; }
        if block_state.r#mode == Mode::Save { return 21520; }
        if block_state.r#mode == Mode::Load { return 21521; }
        if block_state.r#mode == Mode::Data { return 21523; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21522 {
            return Some(StructureBlock {
                r#mode: Mode::Corner,
            });
        }
        if state_id == 21520 {
            return Some(StructureBlock {
                r#mode: Mode::Save,
            });
        }
        if state_id == 21521 {
            return Some(StructureBlock {
                r#mode: Mode::Load,
            });
        }
        if state_id == 21523 {
            return Some(StructureBlock {
                r#mode: Mode::Data,
            });
        }
        return None;
    }
}

