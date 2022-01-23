#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod consts;


pub mod ai;
pub mod ai_camera;
pub mod ai_dangerzone;
pub mod ai_debug;
pub mod ai_deprecated;
pub mod ai_koopag;
pub mod ai_notify_event;
pub mod ai_param;
pub mod ai_random;
pub mod ai_rule;
pub mod ai_stage;
pub mod ai_system;
pub mod ai_utility;
pub mod ai_weapon;

mod events;

pub use consts::*;
pub use events::*;

// Temporary
#[repr(C)]
pub struct BattleObject(u64);

#[repr(C)]
pub struct BattleObjectModuleAccessor(u64);

#[repr(C)]
pub struct FighterAIWeapon(u64);