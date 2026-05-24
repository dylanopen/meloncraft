use std::sync::{LazyLock, Mutex};

use crate::log::Log;

pub static LOG_BUFFER: LazyLock<Mutex<Vec<Log>>> = LazyLock::new(|| return Mutex::new(Vec::new()));

