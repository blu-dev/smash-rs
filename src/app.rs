#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod consts;


pub mod ai;
mod events;

pub use consts::*;
pub use events::*;

// Temporary
#[repr(C)]
pub struct BattleObject(u64);

#[repr(C)]
pub struct BattleObjectModuleAccessor(u64);