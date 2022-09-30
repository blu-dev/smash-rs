use crate::*;

#[repr(C)]
#[vtable_impl(BattleObject)]
// #[derive(TypeAssert)]
pub(crate) struct BattleObjectVTable {}

#[repr(C)]
pub struct BattleObject {
    vtable: &'static BattleObjectVTable,
    pub battle_object_id: u32,
    pub kind: i32,
    pub entry_id: i32,
    pub agent_kind: phx::Hash40,
    pub module_accessor: *const app::BattleObjectModuleAccessor,
    _x28: [u8; 0x38],
}

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
pub struct Enemy {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Gimmick {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Item {
    parent: BattleObject,
    // ...
}
