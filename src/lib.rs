pub mod cpp;
mod lib_impl;
pub mod phx;

pub use lib_impl::lib;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct lua_State(u64);