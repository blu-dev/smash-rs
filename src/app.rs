#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod consts;


mod ai_extras;
pub mod ai;
mod bosses;
pub mod camera;
pub mod debug;
mod events;
pub mod fighter;
mod items;
mod modules;
mod singletons;


pub use ai_extras::*;
pub use bosses::*;
pub use consts::*;
pub use events::*;
pub use items::*;
pub use modules::*;
pub use singletons::*;

// Temporary
#[repr(C)]
pub struct BattleObject(u64);

#[repr(C)]
pub struct Fighter {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Weapon {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Item {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct BattleObjectModuleAccessor(u64);

#[repr(C)]
pub struct FighterModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct WeaponModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct ItemModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct FighterAIWeapon(u64);