use crate::*;

#[repr(C)]
pub struct SituationTransitionSet {
    situation_kind: app::SituationKind,
    transition_group_ids: *const app::WorkId,
    transition_group_count: u32,
}

impl SituationTransitionSet {
    /// Gets the list of transition group ids as a slice
    pub fn group_ids<'a>(&'a self) -> &'a [app::WorkId] {
        unsafe {
            std::slice::from_raw_parts(self.transition_group_ids, self.transition_group_count as usize)
        }
    }

    /// Gets the situation kind of the set
    pub fn situation(&self) -> app::SituationKind {
        self.situation_kind
    }
}

#[repr(C)]
pub struct SituationTransitionSetInfo {
    situation_transition_sets: *const SituationTransitionSet,
    set_count: u32
}

impl SituationTransitionSetInfo {
    /// Gets the number of transition sets in this group
    pub fn count(&self) -> usize {
        self.set_count as usize
    }

    /// Gets the transition sets as a slice
    pub fn as_slice<'a>(&'a self) -> &'a [SituationTransitionSet] {
        unsafe {
            std::slice::from_raw_parts(self.situation_transition_sets, self.set_count as usize)
        }
    }

    /// Attempts to get the transition set for the provided situation kind, if it exists
    /// in this group
    /// 
    /// ### Arguments
    /// * `situation` - The situation kind to get the transition set for
    /// 
    /// ### Returns
    /// * `Some(&SituationTransitionSet)` - If there was a transition set for the provided situation
    /// * `None` - If there was not a transition set for the provided situation
    pub fn get_set_for_situation<'a>(&'a self, situation: app::SituationKind) -> Option<&'a SituationTransitionSet> {
        self.as_slice()
            .iter()
            .find(|set| set.situation_kind == situation)
    }
}

#[repr(C)]
#[vtable_impl(CancelModule)]
pub(crate) struct CancelModuleVTable {
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
    pub re_enable_cancel: extern "C" fn(this: &mut CancelModule),

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

#[cfg(feature = "type_assert")]
impl CancelModuleVTable {
    pub fn assert() {
        assert_eq!(size_of!(CancelModuleVTable), 0x70);
        assert_eq!(offset_of!(CancelModuleVTable, destructor),               0x0);
        assert_eq!(offset_of!(CancelModuleVTable, deleter),                  0x8);
        assert_eq!(offset_of!(CancelModuleVTable, is_implemented),           0x10);
        assert_eq!(offset_of!(CancelModuleVTable, handle_int_msc_command),   0x18);
        assert_eq!(offset_of!(CancelModuleVTable, handle_float_msc_command), 0x20);
        assert_eq!(offset_of!(CancelModuleVTable, initialize),               0x28);
        assert_eq!(offset_of!(CancelModuleVTable, finalize),                 0x30);
        assert_eq!(offset_of!(CancelModuleVTable, start_module),             0x38);
        assert_eq!(offset_of!(CancelModuleVTable, end_module),               0x40);
        assert_eq!(offset_of!(CancelModuleVTable, is_enable_cancel),         0x48);
        assert_eq!(offset_of!(CancelModuleVTable, enable_cancel),            0x50);
        assert_eq!(offset_of!(CancelModuleVTable, re_enable_cancel),         0x58);
        assert_eq!(offset_of!(CancelModuleVTable, unable_cancel_status),     0x60);
        assert_eq!(offset_of!(CancelModuleVTable, unable_cancel),           0x68);
    }   
}

#[repr(C)]
pub struct CancelModule {
    vtable: &'static CancelModuleVTable,
    owner: *mut app::BattleObjectModuleAccessor,
}

#[repr(C)]
#[virtual_implementor(CancelModule)]
pub struct FighterCancelModuleImpl {
    parent: CancelModule,
    transition_groups: *const SituationTransitionSetInfo,
    current_transition_set: app::SituationKind,
    has_canceled: bool,
    is_cancel_blocked: bool
}

#[cfg(feature = "type_assert")]
impl FighterCancelModuleImpl {
    pub fn assert() {
        assert_eq!(offset_of!(FighterCancelModuleImpl, parent), 0x0);
        assert_eq!(offset_of!(FighterCancelModuleImpl, transition_groups), 0x10);
        assert_eq!(offset_of!(FighterCancelModuleImpl, current_transition_set), 0x18);
        assert_eq!(offset_of!(FighterCancelModuleImpl, has_canceled), 0x1C);
        assert_eq!(offset_of!(FighterCancelModuleImpl, is_cancel_blocked), 0x1D);
    }
}

impl FighterCancelModuleImpl {
    /// Gets this module's transition sets
    pub fn transition_groups<'a>(&'a self) -> &'a SituationTransitionSetInfo {
        unsafe {
            &*self.transition_groups
        }
    }

    /// Checks if canceling is blocked until the end of the status
    pub fn is_cancel_blocked(&self) -> bool {
        self.is_cancel_blocked
    }

    /// Checks if canceling is currently enabled
    pub fn has_already_canceled(&self) -> bool {
        self.has_canceled
    }

    /// Tries to get the situation kind of the current enabled cancel set,
    /// returning [`None`] if it has not canceled.
    pub fn current_enabled_transition_set(&self) -> Option<app::SituationKind> {
        if self.has_canceled && !self.is_cancel_blocked {
            Some(self.current_transition_set)
        } else {
            None
        }
    }
}