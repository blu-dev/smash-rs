use crate::*;

#[repr(C)]
struct SituationTransitionSet {
    situation_kind: app::SituationKind,
    transitions_groups: *const app::WorkId,
    transition_group_count: u32,
}

#[repr(C)]
struct SituationTransitionSetInfo {
    situation_transition_sets: *const SituationTransitionSet,
    set_count: u32
}

#[repr(C)]
#[vtable_impl(CancelModule)]
struct CancelModuleVTable {
    destructor: extern "C" fn(this: &mut CancelModule),
    deleter: extern "C" fn(this: &mut CancelModule),
    is_implemented: extern "C" fn(this: &mut CancelModule) -> bool,
    handle_int_msc_command: extern "C" fn(this: &mut CancelModule, command: &lib::MscCommand) -> lib::TValue,
    handle_float_msc_command: extern "C" fn(this: &mut CancelModule, command: &lib::MscCommand) -> lib::TValue,

    /// This method is blank and does not do anything
    initialize: extern "C" fn(this: &mut CancelModule),

    /// Despite the initialization not doing anything, this method will
    /// attempt to remove some event listeners
    finalize: extern "C" fn(this: &mut CancelModule),

    /// This method is blank and does not do anything
    start_module: extern "C" fn(this: &mut CancelModule),

    /// This method is blank and does not do anything
    end_module: extern "C" fn(this: &mut CancelModule),

    /// Checks if the fighter's cancel groups have been enabled
    /// 
    /// ### Behavior
    /// At least in the fighter's implementation of [`CancelModule`], canceling is determined based on 
    /// whether or not there is a list of transition groups for the fighter's situation kind. If there
    /// is a list of them, then when either [`CancelModule::enable_cancel`] or [`CancelModule::re_renable_cancel`]
    /// is called it will enable those transitions groups via [`app::WorkModule::enable_transition_term_group`].
    /// 
    /// ### Returns
    /// * `true` - The fighter has transition groups for their current situation kind and they have been
    /// enabled by [`CancelModule`]
    /// * `false` - Either there are no transition groups fo the fighter's current situation kind or 
    /// they haven't been enabled yet
    /// 
    /// ### Notes
    /// This function can be `true` while also not having canceling be enabled if something else disables
    /// the transition term groups manually
    pub is_enable_cancel: extern "C" fn(this: &CancelModule) -> bool,

    /// Conditionally enables the fighter to cancel their current action
    /// 
    /// ### Behavior (Fighter Implementation)
    /// If the fighter is in any of the following states (checked via [`app::WorkModule`] flags), this function exits without enabling
    /// the cancel:
    /// * Using a hammer
    /// * Holding up the special flag (the one that gives you an extra life) or the bomber (the one that explodes)
    /// * Using the daybreak
    /// * When the flag for [`app::work_ids::fighter::instance::KNOCKOUT`] is set
    /// 
    /// It also checks to see if the flag for `unable_cancel_status` has been set, and if it is it will not enable the transitions.
    /// 
    /// ### Notes
    /// * If you are performing an aerial, then [`app::transition_terms::LANDING`] and [`app::transition_terms::LANDING_LIGHT`]
    /// are disabled.
    /// * If you are performing down tilt, then [`app::transition_terms::CONT_SQUAT`] is disabled.
    pub enable_cancel: extern "C" fn(this: &mut CancelModule),

    /// Re-enables cancelling if your situation kind has changed.
    /// 
    /// ### Behavior (Fighter Implementation)
    /// After checking to make sure that you were able to cancel before ([`CancelModule::is_enable_cancel`] is true)
    /// and your situation kind has changed, it will disable the previous situation's transition term groups
    /// and enable the new situation's transition term groups.
    /// 
    /// It also checks to see if the flag for `unable_cancel_status` has been set, and if it is it will not enable the transitions.
    /// 
    /// ### Notes
    /// Unlike [`CancelModule::enable_cancel`], this method will not disable [`app::transition_terms::LANDING`], [`app::transition_terms::LANDING_LIGHT`]
    /// or [`app::transition_terms::CONT_SQUAT] after switching the transition term groups, likely because the status kind has not changed yet.
    pub re_renable_cancel: extern "C" fn(this: &mut CancelModule),

    /// Disables all transition term groups for all situation kinds and blocks cancelling for the rest of the status
    /// 
    /// ### Behavior (Fighter Implementation)
    /// Disables the transition term groups for **all** situation kinds (does not check the current one) and sets a flag which
    /// will only be unset upon a status change.
    pub unable_cancel_status: extern "C" fn(this: &mut CancelModule),

    /// Disables all transition term groups for all situation kinds 
    /// 
    /// ### Behavior (Fighter Implementation)
    /// The behavior for this method is identical to [`CancelModule::unable_cancel_status`], except it does not set the flag
    /// which blocks cancelling until the next status
    pub unable_cancel: extern "C" fn(this: &mut CancelModule)
}

#[repr(C)]
pub struct CancelModule {
    vtable: &'static CancelModuleVTable
}