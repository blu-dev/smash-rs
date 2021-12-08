use crate::*;
use skyline::libc::c_void;

#[repr(C)]
pub struct L2CTable {
    ref_count: u32,
    padding: u32,
    array: cpp::Vector<lib::L2CValue>,
    pub map: cpp::Tree<phx::Hash40, lib::L2CValue>,
    agent: *mut c_void, // to become L2CAgent
    metatable: *mut L2CTable
}