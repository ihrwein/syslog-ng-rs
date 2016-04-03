use std::os::raw::{c_int, c_char, c_void};
use GlobalConfig;

#[link(name = "syslog-ng")]
extern "C" {
    pub fn plugin_register(cfg: *mut GlobalConfig, plugin: *const Plugin, number: c_int) -> c_void;
}

pub enum TokenType {
    ContextParser = 4
}

pub enum CfgArgs {}
pub enum CfgLexerKeyword {}

#[repr(C)]
pub struct CfgParser {
    // pub debug_flag: *mut c_int,
    pub debug_flag: *const c_int,
    pub context: c_int,
    pub name: *const c_char,
    pub keywords: *const CfgLexerKeyword,
    // fn ptr
    pub parse: *const c_void,
    // fn ptr
    pub cleanup: *const c_void,
}

#[repr(C)]
pub struct Plugin {
    // this is called `type` in the original C struct but it's a reserved
    // keyword in Rust
    pub _type: c_int,
    pub name: *const c_char,
    pub parser: *const CfgParser,
    // fn ptr
    pub setup_context: *const c_void,
    // fn ptr
    pub construct: *const c_void,
    // fn ptr
    pub free_fn: *const c_void,
}

unsafe impl Sync for ModuleInfo {}
unsafe impl Sync for CfgParser {}
unsafe impl Sync for Plugin {}

#[repr(C)]
pub struct ModuleInfo {
    pub canonical_name: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
    pub core_revision: *const c_char,
    pub plugins: *const Plugin,
    pub plugins_len: c_int,
    pub preference: c_int
}
