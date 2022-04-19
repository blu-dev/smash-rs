use crate::*;

use app::*;

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x1C0]
pub struct LuaStateAccessor {
    #[offset = 0x000] _x0: [u64; 3],
    #[offset = 0x018] posture_module: *mut Module,
    #[offset = 0x020] _x20: u64,
    #[offset = 0x028] status_module: *mut StatusModule,
    #[offset = 0x030] control_module: *mut Module,
    #[offset = 0x038] work_module: *mut WorkModule,
    #[offset = 0x040] ground_module: *mut Module,
    #[offset = 0x048] camera_module: *mut Module,
    #[offset = 0x050] kinetic_module: *mut Module,
    #[offset = 0x058] color_blend_module: *mut Module,
    #[offset = 0x060] model_module: *mut Module,
    #[offset = 0x068] physics_module: *mut Module,
    #[offset = 0x070] motion_module: *mut Module,
    #[offset = 0x078] stop_module: *mut Module,
    #[offset = 0x080] article_module: *mut Module,
    #[offset = 0x088] attack_module: *mut Module,
    #[offset = 0x090] damage_module: *mut Module,
    #[offset = 0x098] hit_module: *mut Module,
    #[offset = 0x0A0] combo_module: *mut Module,
    #[offset = 0x0A8] area_module: *mut Module,
    #[offset = 0x0B0] item_module: *mut Module,
    #[offset = 0x0B8] link_module: *mut Module,
    #[offset = 0x0C0] team_module: *mut Module,
    #[offset = 0x0C8] search_module: *mut Module,
    #[offset = 0x0D0] unk_module_1: *mut Module,
    #[offset = 0x0D8] turn_module: *mut Module,
    #[offset = 0x0E0] reflect_module: *mut Module,
    #[offset = 0x0E8] shield_module: *mut Module,
    #[offset = 0x0F0] absorber_module: *mut Module,
    #[offset = 0x0F8] reflector_module: *mut Module,
    #[offset = 0x100] jostle_module: *mut Module,
    #[offset = 0x108] catch_module: *mut Module,
    #[offset = 0x110] cancel_module: *mut CancelModule,
    #[offset = 0x118] unk_module_2: *mut Module,
    #[offset = 0x120] capture_module: *mut Module,
    #[offset = 0x128] effect_module: *mut Module,
    #[offset = 0x130] sound_module: *mut Module,
    #[offset = 0x138] visibility_module: *mut Module,
    #[offset = 0x140] grab_module: *mut Module,
    #[offset = 0x148] slope_module: *mut Module,
    #[offset = 0x150] shake_module: *mut Module,
    #[offset = 0x158] slow_module: *mut Module,
    #[offset = 0x160] unk_module_3: *mut Module,
    #[offset = 0x168] shadow_module: *mut Module,
    #[offset = 0x170] motion_animcmd_module: *mut MotionAnimcmdModule,
    #[offset = 0x178] lua_module: *mut LuaModule,
    #[offset = 0x180] ink_paint_module: *mut Module,
    #[offset = 0x188] _x188: u64,
    #[offset = 0x190] battle_object_id: u32,
    #[offset = 0x194] _x194: u32,
    #[offset = 0x198] agent_kind: i32,
    #[offset = 0x19C] entry_id: app::FighterEntryID,
    #[offset = 0x1A0] module_accessor: *mut BattleObjectModuleAccessor,
    #[offset = 0x1A8] battle_object: *mut BattleObject,
    #[offset = 0x1B0] animcmd_state: *mut lib::LuaAnimcmdState,
    #[offset = 0x1B8] animcmd_exec_state: *mut lib::LuaAnimcmdExecutionState
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x30]
pub struct LuaThread {
    #[offset = 0x00] vtable: *const *const skyline::libc::c_void,
    #[offset = 0x08] state: *mut lua_State,
    #[offset = 0x10] _xC: u32,
    #[offset = 0x18] thread_name: phx::Hash40,
    #[offset = 0x20] parent_thread_name: phx::Hash40,
    #[offset = 0x28] _x28: u32,
    #[offset = 0x2C] _x2C: u32,
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x50]
pub struct LuaThreadInfo {
    #[offset = 0x00] thread: *mut LuaThread,
    #[offset = 0x08] coroutines: [lib::TValue; 4],
    #[offset = 0x48] is_release_control: bool
}