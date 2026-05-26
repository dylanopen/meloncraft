use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledDeepslate {}

impl BlockState for ChiseledDeepslate {
    fn to_id(&self) -> i32 {
        return 29368;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29368 {
            return Some(ChiseledDeepslate {});
        }
        return None;
    }
}
