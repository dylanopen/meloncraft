use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedAcaciaSapling {}

impl BlockState for PottedAcaciaSapling {
    fn to_id(&self) -> i32 {
        return 10434;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10434 {
            return Some(PottedAcaciaSapling {});
        }
        return None;
    }
}
