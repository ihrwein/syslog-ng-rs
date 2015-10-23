use ::types::*;

pub enum EVTREC {}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn msg_event_create_from_desc(prio: i32, desc: *const c_char) -> *mut EVTREC;
    pub fn msg_event_suppress_recursions_and_send(e: *mut EVTREC);
    pub static debug_flag: c_int;
    pub static trace_flag: c_int;
}