use crate::*;
use skyline::libc::c_void;

#[repr(C)]
pub struct L2CTable {
    ref_count: u32,
    padding: u32,
    array: cpp::Vector<[u8; 16]>, // to become cpp::Vector<L2CValue>
    map: cpp::Tree<phx::Hash40, [u8; 16]>, // to become cpp::Tree<phx::Hash40, L2CValue>
    agent: *mut c_void, // to become L2CAgent
    metatable: *mut L2CTable
}