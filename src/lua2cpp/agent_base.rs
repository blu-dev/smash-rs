use crate::*;

pub type StatusFn = extern "C" fn(*mut L2CAgentBase) -> lib::L2CValueHack;

#[repr(C)]
struct StatusData {
    pub pre: StatusFn,
    pub main: StatusFn,
    pub end: StatusFn,
    pub init: StatusFn,
    pub exec: StatusFn,
    pub exec_stop: StatusFn,
    pub exit: StatusFn,
    pub map_correction: StatusFn,
    pub fix_camera: StatusFn,
    pub fix_pos_slow: StatusFn,
    pub check_damage: StatusFn,
    pub check_attack: StatusFn,
    pub on_change_lr: StatusFn,
    pub leave_stop: StatusFn,
    pub notify_event_gimmick: StatusFn,
    pub calc_param: StatusFn,
    pub reserve1: StatusFn,
    pub reserve2: StatusFn,
    pub reserve3: StatusFn,
    pub reserve4: StatusFn,
    pub reserve5: StatusFn
}

#[repr(C)]
struct Coroutine {
    fiber: *mut phx::Fiber,
    parent_fiber: *mut phx::Fiber,
    status: u32,
    is_running: bool,
    padding: [u8; 3]
}

#[repr(C)]
pub struct L2CAgentBase {
    agent: lib::L2CAgent,
    statuses: cpp::Vector<StatusData>,
    coroutines: [Coroutine; 4],
    unused: u8,
    coroutine_release_control: bool,
    padding: [u8; 6]
}