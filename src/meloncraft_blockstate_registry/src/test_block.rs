use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestBlock {
    pub r#mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Start,
    Log,
    Fail,
    Accept,
}

impl BlockState for TestBlock {
    fn to_id(&self) -> i32 {
        if self.r#mode == Mode::Log {
            return 21537;
        }
        if self.r#mode == Mode::Accept {
            return 21539;
        }
        if self.r#mode == Mode::Start {
            return 21536;
        }
        if self.r#mode == Mode::Fail {
            return 21538;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21537 {
            return Some(TestBlock { r#mode: Mode::Log });
        }
        if state_id == 21539 {
            return Some(TestBlock {
                r#mode: Mode::Accept,
            });
        }
        if state_id == 21536 {
            return Some(TestBlock {
                r#mode: Mode::Start,
            });
        }
        if state_id == 21538 {
            return Some(TestBlock { r#mode: Mode::Fail });
        }
        return None;
    }
}
