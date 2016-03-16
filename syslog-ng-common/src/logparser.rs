// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use syslog_ng_sys;
use syslog_ng_sys::LogPipe;

pub struct LogParser(*mut syslog_ng_sys::LogParser);

impl LogParser {
    pub fn wrap_raw(raw: *mut syslog_ng_sys::LogParser) -> LogParser {
        LogParser(raw)
    }
    pub fn as_pipe(&mut self) -> *mut LogPipe {
        self.0 as *mut LogPipe
    }
}
