use std::sync::Mutex;

use crate::log::Log;

pub static LOG_BUFFER: Mutex<Vec<Log>> = Mutex::new(Vec::new());
