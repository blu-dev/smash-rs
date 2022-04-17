use std::ops::{Deref, DerefMut};

use crate::*;

use app::*;

#[repr(C)]
pub struct ModuleAccessor {
    event_manager:                                   lib::EventManager,
    posture_module:                                  *mut Module,
    status_module:                                   *mut Module,
    control_module:                                  *mut Module,
    work_module:                                     *mut WorkModule,
    ground_module:                                   *mut Module,
    camera_module:                                   *mut Module,
    kinetic_module:                                  *mut Module,
    color_blend_module:                              *mut Module,
    model_module:                                    *mut Module,
    physics_module:                                  *mut Module,
    motion_module:                                   *mut Module,
    stop_module:                                     *mut Module,
    article_module:                                  *mut Module,
    attack_module:                                   *mut Module,
    damage_module:                                   *mut Module,
    hit_module:                                      *mut Module,
    combo_module:                                    *mut Module,
    area_module:                                     *mut Module,
    item_module:                                     *mut Module,
    link_module:                                     *mut Module,
    team_module:                                     *mut Module,
    search_module:                                   *mut Module,
    unk_module_1:                                    *mut Module,
    turn_module:                                     *mut Module,
    reflect_module:                                  *mut Module,
    shield_module:                                   *mut Module,
    reflector_module:                                *mut Module,
    absorber_module:                                 *mut Module,
    jostle_module:                                   *mut Module,
    catch_module:                                    *mut Module,
    cancel_module:                                   *mut CancelModule,
    unk_module_2:                                    *mut Module, // appears to have to do with spirits
    capture_module:                                  *mut Module,
    effect_module:                                   *mut Module,
    sound_module:                                    *mut Module,
    visibility_module:                               *mut Module,
    grab_module:                                     *mut Module,
    slope_module:                                    *mut Module,
    shake_module:                                    *mut Module,
    slow_module:                                     *mut Module,
    unk_module_3:                                    *mut Module,
    shadow_module:                                   *mut Module,
    motion_animcmd_module:                           *mut Module,
    lua_module:                                      *mut Module,
    ink_paint_module:                                *mut Module,
}

#[cfg(feature = "type_assert")]
impl ModuleAccessor {
    pub fn assert() {
        assert_eq!(size_of!(ModuleAccessor), 0x190);
        assert_eq!(offset_of!(ModuleAccessor, posture_module),        0x028);
        assert_eq!(offset_of!(ModuleAccessor, status_module),         0x030);
        assert_eq!(offset_of!(ModuleAccessor, control_module),        0x038);
        assert_eq!(offset_of!(ModuleAccessor, work_module),           0x040);
        assert_eq!(offset_of!(ModuleAccessor, ground_module),         0x048);
        assert_eq!(offset_of!(ModuleAccessor, camera_module),         0x050);
        assert_eq!(offset_of!(ModuleAccessor, kinetic_module),        0x058);
        assert_eq!(offset_of!(ModuleAccessor, color_blend_module),    0x060);
        assert_eq!(offset_of!(ModuleAccessor, model_module),          0x068);
        assert_eq!(offset_of!(ModuleAccessor, physics_module),        0x070);
        assert_eq!(offset_of!(ModuleAccessor, motion_module),         0x078);
        assert_eq!(offset_of!(ModuleAccessor, stop_module),           0x080);
        assert_eq!(offset_of!(ModuleAccessor, article_module),        0x088);
        assert_eq!(offset_of!(ModuleAccessor, attack_module),         0x090);
        assert_eq!(offset_of!(ModuleAccessor, damage_module),         0x098);
        assert_eq!(offset_of!(ModuleAccessor, hit_module),            0x0A0);
        assert_eq!(offset_of!(ModuleAccessor, combo_module),          0x0A8);
        assert_eq!(offset_of!(ModuleAccessor, area_module),           0x0B0);
        assert_eq!(offset_of!(ModuleAccessor, item_module),           0x0B8);
        assert_eq!(offset_of!(ModuleAccessor, link_module),           0x0C0);
        assert_eq!(offset_of!(ModuleAccessor, team_module),           0x0C8);
        assert_eq!(offset_of!(ModuleAccessor, search_module),         0x0D0);
        assert_eq!(offset_of!(ModuleAccessor, unk_module_1),          0x0D8);
        assert_eq!(offset_of!(ModuleAccessor, turn_module),           0x0E0);
        assert_eq!(offset_of!(ModuleAccessor, reflect_module),        0x0E8);
        assert_eq!(offset_of!(ModuleAccessor, shield_module),         0x0F0);
        assert_eq!(offset_of!(ModuleAccessor, reflector_module),      0x0F8);
        assert_eq!(offset_of!(ModuleAccessor, absorber_module),       0x100);
        assert_eq!(offset_of!(ModuleAccessor, jostle_module),         0x108);
        assert_eq!(offset_of!(ModuleAccessor, catch_module),          0x110);
        assert_eq!(offset_of!(ModuleAccessor, cancel_module),         0x118);
        assert_eq!(offset_of!(ModuleAccessor, unk_module_2),          0x120);
        assert_eq!(offset_of!(ModuleAccessor, capture_module),        0x128);
        assert_eq!(offset_of!(ModuleAccessor, effect_module),         0x130);
        assert_eq!(offset_of!(ModuleAccessor, sound_module),          0x138);
        assert_eq!(offset_of!(ModuleAccessor, visibility_module),     0x140);
        assert_eq!(offset_of!(ModuleAccessor, grab_module),           0x148);
        assert_eq!(offset_of!(ModuleAccessor, slope_module),          0x150);
        assert_eq!(offset_of!(ModuleAccessor, shake_module),          0x158);
        assert_eq!(offset_of!(ModuleAccessor, slow_module),           0x160);
        assert_eq!(offset_of!(ModuleAccessor, unk_module_3),          0x168);
        assert_eq!(offset_of!(ModuleAccessor, shadow_module),         0x170);
        assert_eq!(offset_of!(ModuleAccessor, motion_animcmd_module), 0x178);
        assert_eq!(offset_of!(ModuleAccessor, lua_module),            0x180);
        assert_eq!(offset_of!(ModuleAccessor, ink_paint_module),      0x188);
    }
}

impl ModuleAccessor {
    pub fn work<'a>(&'a self) -> &'a mut WorkModule {
        unsafe {
            &mut *self.work_module
        }
    }

    pub fn cancel<'a>(&'a self) -> &'a mut CancelModule {
        unsafe {
            &mut *self.cancel_module
        }
    }
}

#[repr(C)]
#[vtable_impl(BattleObjectModuleAccessor)]
pub(crate) struct BattleObjectModuleAccessorVTable {

    /// Checks if this module accessor is implemented
    /// 
    /// ### Returns
    /// * `true` - The module accessor is **not** implemented
    /// * `false` - The module accessor **is** implemented
    /// 
    /// ### Notes
    /// If the module accessor is not implemented, there should be no attempt to
    /// use any of its modules as if they were implemented, and there should be
    /// not attempt to cast it to an implementor's type.
    pub is_virtual: extern "C" fn(this: &BattleObjectModuleAccessor) -> bool,


    destructor: extern "C" fn(this: &mut BattleObjectModuleAccessor),
    deleter: extern "C" fn(this: &mut BattleObjectModuleAccessor),

    initialize_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor, init_args: *const u64),
    finalize_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor),
    start_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor),
    end_modules: extern "C" fn(this: &mut BattleObjectModuleAccessor),
}

#[cfg(feature = "type_assert")]
impl BattleObjectModuleAccessorVTable {
    pub fn assert() {
        assert_eq!(size_of!(BattleObjectModuleAccessorVTable), 0x38);
        assert_eq!(offset_of!(BattleObjectModuleAccessorVTable, is_virtual), 0x0);
        assert_eq!(offset_of!(BattleObjectModuleAccessorVTable, destructor), 0x8);
        assert_eq!(offset_of!(BattleObjectModuleAccessorVTable, deleter), 0x10);
        assert_eq!(offset_of!(BattleObjectModuleAccessorVTable, initialize_modules), 0x18);
        assert_eq!(offset_of!(BattleObjectModuleAccessorVTable, finalize_modules), 0x20);
        assert_eq!(offset_of!(BattleObjectModuleAccessorVTable, start_modules), 0x28);
        assert_eq!(offset_of!(BattleObjectModuleAccessorVTable, end_modules), 0x30);
    }
}

#[repr(C)]
pub struct BattleObjectModuleAccessor {
    vtable: &'static BattleObjectModuleAccessorVTable,
    battle_object_id: u32,
    padding: u32,
    module_accessor: ModuleAccessor,
}

impl Deref for BattleObjectModuleAccessor {
    type Target = ModuleAccessor;

    fn deref(&self) -> &Self::Target {
        &self.module_accessor
    }
}

impl DerefMut for BattleObjectModuleAccessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.module_accessor
    }
}

// Temporary...

/** Module allocation sizes for Fighters
 * PostureModule => 0xD0
 * StatusModule => 0x188
 * ControlModule => 0xAF0
 * WorkModule => 0x98
 * GroundModule => 0x170
 * CameraModule => 0x58
 * KineticModule => 0xD70
 * ColorBlendModule => 0x620
 * ModelModule => 0x1D0
 * PhysicsModule => 0xD0
 * MotionModule => 0x310
 * StopModule => 0x50
 * ArticleModule => 0x260
 * AttackModule => 0x260
 * DamageModule => 0x3D0
 * HitModule => 0xC8
 * ComboModule => 0x78
 * AreaModule => 0x310
 * ItemModule => 0x70
 * LinkModule => 0x150
 * TeamModule => 0x50
 * SearchModule => 0x3A0
 * UnknownModule1 => DefaultImpl
 * TurnModule => 0x68
 * ReflectModule => 0x38
 * ShieldModule => 0xB8
 * ReflectorModule => 0xB8
 * AbsorberModule => 0xB8
 * JostleModule => 0x80
 * CatchModule => 0xA0
 * CancelModule => 0x20
 * UnknownModule2 => DefaultImpl
 * CaptureModule => 0x90
 * EffectModule => 0x108
 * SoundModule => 0x860
 * VisibilityModule => 0x88
 * GrabModule => 0x380
 * SlopeModule => 0xC8
 * ShakeModule => 0x80
 * SlowModule => 0x48
 * UnknownModule3 => 0x2b0
 * ShadowModule => 0x40
 * MotionAnimcmdModule => 0x10
 * LuaModule => 0x228
 * InkPaintModule => 0xE0
 */

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