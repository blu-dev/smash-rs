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
mod module_accessors;
mod singletons;
pub mod smashball;
pub mod smashballheavy;
mod specializers;
pub mod stage;
pub mod sv_information;

pub use ai_extras::*;
pub use bosses::*;
pub use consts::*;
pub use events::*;
pub use items::*;
pub use modules::*;
pub use module_accessors::*;
pub use singletons::*;
pub use specializers::*;

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
pub struct Gimmick {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Enemy {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct FighterAIWeapon(u64);

pub type FighterEntryID = u32;

#[repr(C)]
pub struct FighterEntry {}

#[repr(C)]
pub struct FighterInformation {}