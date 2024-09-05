use super::{Core, Handle};

use std::boxed::Box;

impl Handle {
    pub(super) fn trace_core(&self, core: Box<Core>) -> Box<Core> {
        core
    }
}
