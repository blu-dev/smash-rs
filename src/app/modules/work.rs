use std::{convert::TryFrom, ops::{Index, IndexMut}};

use crate::*;

use thiserror::Error;

macro_rules! const_partial_eq {
    ($id:ident) => {
        impl const std::cmp::PartialEq for $id {
            fn eq(&self, other: &Self) -> bool {
                (*self as u8) == (*other as u8)
            }
        }
    }
}

/// Enum representing the category fo data that a [`WorkId`] refers to. This field
/// determines where data is stored. [`WorkKind::Transition`] and [`WorkKind::TransitionGroup`]
/// are not checked by any calls to the [`WorkModule`] accessors, however [`WorkKind::Instance`]
/// and [`WorkKind::Status`] are used.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq)]
pub enum WorkKind {
    Instance = 0x0,
    Status = 0x1,
    Transition = 0xE,
    TransitionGroup = 0xF,

    None = 0xFF
}

const_partial_eq!(WorkKind);

impl From<u8> for WorkKind {
    fn from(arg: u8) -> Self {
        match arg {
            0x0 => WorkKind::Instance,
            0x1 => WorkKind::Status,
            0xE => WorkKind::Transition,
            0xF => WorkKind::TransitionGroup,
            _ => WorkKind::None
        }
    }
}

/// Enum representing the type of data that a [`WorkId`] refers to. This field
/// has no impact on where the data is stored, and is not check by any calls to the
/// [`WorkModule`] accessors.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq)]
pub enum WorkType {
    Float = 0x0,
    Int = 0x1,
    Flag = 0x2,

    None = 0xFF
}

const_partial_eq!(WorkType);

impl From<u8> for WorkType {
    fn from(arg: u8) -> Self {
        match arg {
            0x0 => WorkType::Float,
            0x1 => WorkType::Int,
            0x2 => WorkType::Flag,
            _ => WorkType::None
        }
    }
}

/// The symbolic constant used to read and write data to the [`WorkModule`]'s internal storage.
/// 
/// They are made up of 3 components:
/// * The [`WorkType`], which designates what kind of data the constant is referring to
/// * The [`WorkKind`], which designates what category the data belongs to
/// * The index, which is used to find the location of the data in the internal storage.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct WorkId(u32);

impl WorkId {
    pub(crate) const fn from_parts(ty: WorkType, kind: WorkKind, index: u32) -> Self {
        if ty == WorkType::None {
            panic!("WorkType cannot be None");
        }
        if kind == WorkKind::None {
            panic!("WorkKind cannot be None");
        }

        let ty = (ty as u8 as u32) << 0x1C;
        let kind = (kind as u8 as u32) << 0x18;
        let index = index & 0x00FF_FFFF;
        Self(ty | kind | index)
    }

    pub fn get_type(self) -> WorkType {
        WorkType::from((self.0 >> 0x1C) as u8)
    }

    pub fn get_kind(self) -> WorkKind {
        WorkKind::from(((self.0 >> 0x18) & 0xF) as u8)
    }

    pub fn get_index(self) -> u32 {
        self.0 & 0x00FF_FFFF
    }

    pub unsafe fn from_u32_unchecked(value: u32) -> Self {
        Self(value)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Error, Debug)]
pub enum WorkIdTryFromError {
    #[error("The type field of the ID was invalid! {0} is not a valid WorkType")]
    InvalidType(u8),

    #[error("The kind field of the ID was invalid! {0} is not a valid WorkKind")]
    InvalidKind(u8)
}

impl TryFrom<u32> for WorkId {
    type Error = WorkIdTryFromError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let ty = ((value >> 0x1C) & 0xF) as u8;
        let kind = ((value >> 0x18) & 0xF) as u8;
        if WorkType::from(ty) == WorkType::None {
            Err(WorkIdTryFromError::InvalidType(ty))
        } else if WorkKind::from(kind) == WorkKind::None {
            Err(WorkIdTryFromError::InvalidKind(kind))
        } else {
            Ok(Self(value))
        }
    }
}

bitflags! {
    pub struct TransitionTerm: u8 {
        /// Controls whether or not the transition term is enabled when it is not a part of a group
        const ENABLED                  = 0b0000_0001;

        /// Declares whether or not the transition term is enabled as part of a group requires the `ENABLED_EX` flag to really be enabled
        const ENABLED_GROUP            = 0b0000_0010;

        /// Declares whether or not the transition term is enabled when it is part of a group
        const ENABLED_EX               = 0b0000_0100;

        /// Forbids the transition from occurring
        const ENABLE_FORBID            = 0b0000_1000;

        /// Another flag to forbid the transition from occurring (potentially the equivalent to `ENABLED_GROUP`)
        const ENABLE_FORBID_OTHER      = 0b0001_0000; // not exported by any symbols but checked when seeing if transition is forbidden

        /// Another flag to forbid the transition from occurring
        const ENABLE_FORBID_INDIVIDUAL = 0b0010_0000;

        /// A helper mask for all forbid flags
        const FORBIDDEN_MASK = 0b0011_1000;
    }
}

impl TransitionTerm {
    /// Checks to see if the transition term is enabled.
    /// 
    /// A transition term is enabled under the following criteria:
    /// 1. None of the `forbid` flags are set
    /// 2. If it is enabled as part of a group, the `ENABLED_EX` flag must be set
    /// 3. If it is not enabled as part of a group, the `ENABLED` flag must be set
    pub fn is_enabled(self) -> bool {
        if self.intersects(TransitionTerm::FORBIDDEN_MASK) {
            return false;
        }

        if self.contains(TransitionTerm::ENABLED_GROUP) {
            self.contains(TransitionTerm::ENABLED_EX)
        } else {
            self.contains(TransitionTerm::ENABLED)
        }
    }

    /// Checks to see if the transition term is forbidden
    pub fn is_forbidden(self) -> bool {
        self.intersects(TransitionTerm::FORBIDDEN_MASK)
    }
}

#[repr(C)]
struct WorkArrayVTable {
    pub destructor: extern "C" fn(&mut WorkArray),
    pub deleter: extern "C" fn(*mut WorkArray),
    pub get_int: extern "C" fn(&WorkArray, index: u32) -> i32
}

/// A structure to hold on to the three required arrays of internal storage for [`WorkModule`].
/// 
/// The structure holds sizes and raw pointers to data. It should be noted that a `WorkArray` *does*
/// inherit from a base, abstract class because the implementation for `GetInt` can vary per implementation.
/// For fighters, the implementation is to allocate space for 64-bit integers and just set the lower 32-bits if need be
/// 
/// This implementation also appears to be the same for weapons and items.
#[repr(C)]
pub struct WorkArray {
    vtable: &'static WorkArrayVTable,
    float_count: usize,
    floats: *mut f32,
    int_count: usize,
    ints: *mut i64,
    flag_count: usize,
    flags: *mut u32
}

impl WorkArray {
    /// Gets the underlying array of float variables
    pub fn get_floats<'a>(&'a self) -> &'a [f32] {
        unsafe {
            std::slice::from_raw_parts(self.floats, self.float_count)
        }
    }

    /// Gets the underlying mutable array of float variables
    pub fn get_floats_mut<'a>(&'a mut self) -> &'a mut [f32] {
        unsafe {
            std::slice::from_raw_parts_mut(self.floats, self.float_count)
        }
    }

    /// Gets the underlying array of integer variables
    pub fn get_ints<'a>(&'a self) -> &'a [i64] {
        unsafe {
            std::slice::from_raw_parts(self.ints, self.int_count)
        }
    }

    /// Gets the underlying mutable array of integer variables
    pub fn get_ints_mut<'a>(&'a mut self) -> &'a mut [i64] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ints, self.int_count)
        }
    }

    /// Gets the underyling array of flag variables
    pub fn get_flags<'a>(&'a self) -> &'a [u32] {
        unsafe {
            std::slice::from_raw_parts(self.flags, self.flag_count)
        }
    }

    /// Gets the underlying mutable array of flag variables
    pub fn get_flags_mut<'a>(&'a mut self) -> &'a mut [u32] {
        unsafe {
            std::slice::from_raw_parts_mut(self.flags, self.flag_count)
        }
    }

    /// Gets the float variable located at `index`
    pub fn get_float(&self, index: usize) -> f32 {
        self.get_floats()[index]
    }

    /// Gets the 32-bit integer variable located at `index`
    pub fn get_int(&self, index: usize) -> i32 {
        (self.get_ints()[index] & 0xFFFF_FFFF) as i32
    }

    /// Gets the 64-bit integer variable located at `index`
    pub fn get_int64(&self, index: usize) -> i64 {
        self.get_ints()[index]
    }

    /// Gets the flag variable located at `index`
    pub fn get_flag(&self, index: usize) -> bool {
        (self.get_flags()[index / 0x20] >> (index % 0x20)) & 0x1 != 0x0
    }

    /// Sets the float variable located at `index`
    pub fn set_float(&mut self, value: f32, index: usize) {
        self.get_floats_mut()[index] = value;
    }

    /// Sets the 32-bit integer variable located at `index`
    pub fn set_int(&mut self, value: i32, index: usize) {
        self.get_ints_mut()[index] = value as i64;
    }

    /// Sets the 64-bit integer variable located at `index`
    pub fn set_int64(&mut self, value: i64, index: usize) {
        self.get_ints_mut()[index] = value;
    }

    /// Sets the flag variable located at `index`
    pub fn set_flag(&mut self, value: bool, index: usize) {
        if value {
            self.get_flags_mut()[index / 0x20] |= 1 << (index % 0x20);
        } else {
            self.get_flags_mut()[index / 0x20] &= !(1 << (index % 0x20));
        }
    }
}

/// A structure to represent the amount of underlying [`WorkArray`] structures that a [`WorkModule`] uses
/// for it's variable storage.
/// 
/// For fighters, weapons, and likely items, this is always 2 [`WorkArray`]s, one for instance variables
/// and one for status variables
#[repr(C)]
pub struct WorkArrayInfo {
    array_count: usize,
    arrays: *mut WorkArray
}

impl WorkArrayInfo {
    /// Gets the underlying [`WorkArray`]s as a slice
    pub fn as_slice<'a>(&'a self) -> &'a [WorkArray] {
        unsafe {
            std::slice::from_raw_parts(self.arrays, self.array_count)
        }
    }

    /// Gets the underlying [`WorkArray`]s as a mutable slice
    pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [WorkArray] {
        unsafe {
            std::slice::from_raw_parts_mut(self.arrays, self.array_count)
        }
    }

    /// Gets the [`WorkArray`] for the instance variables
    pub fn instance<'a>(&'a self) -> &'a WorkArray {
        &self.as_slice()[0]
    }

    /// Gets the [`WorkArray`] for the status variables
    pub fn status<'a>(&'a self) -> &'a WorkArray {
        &self.as_slice()[1]
    }
}

/// A structure to represent the transition terms that a [`WorkModule`] supports and manages.
/// 
/// According to the lua consts, there don't appear to be any transition terms for objects other
/// than fighters, so it's unlikely that this would be useful for weapons or items
#[repr(C)]
pub struct TransitionTermInfo {
    count: usize,
    transitions: *mut TransitionTerm
}

impl TransitionTermInfo {
    /// Gets the underlying [`TransitionTerm`]s as a slice
    pub fn as_slice<'a>(&'a self) -> &'a [TransitionTerm] {
        unsafe {
            std::slice::from_raw_parts(self.transitions, self.count)
        }
    }

    /// Gets the underlying [`TransitionTerm`]s as a mutable slice
    pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [TransitionTerm] {
        unsafe {
            std::slice::from_raw_parts_mut(self.transitions, self.count)
        }
    }

    /// Gets the [`TransitionTerm`] at the location `index`
    pub fn get_transition(&self, index: usize) -> TransitionTerm {
        self.as_slice()[index]
    }

    /// Sets the [`TransitionTerm`] at the location `index`
    pub fn set_transition(&mut self, transition: TransitionTerm, index: usize) {
        self.as_slice_mut()[index] = transition;
    }
}

impl Index<usize> for TransitionTermInfo {
    type Output = TransitionTerm;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for TransitionTermInfo {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<WorkId> for TransitionTermInfo {
    type Output = TransitionTerm;

    fn index(&self, index: WorkId) -> &Self::Output {
        if index.get_kind() != WorkKind::Transition {
            panic!("WorkKind {:?} is an invalid index for TransitionTermInfo", index.get_kind());
        }

        self.index(index.get_index() as usize)
    }
}

impl IndexMut<WorkId> for TransitionTermInfo {
    fn index_mut(&mut self, index: WorkId) -> &mut Self::Output {
        if index.get_kind() != WorkKind::Transition {
            panic!("WorkKind {:?} is an invalid index for TransitionTermInfo", index.get_kind());
        }

        self.index_mut(index.get_index() as usize)
    }
}

/// A structure to represent a group of [`TransitionTerm`]s.
/// 
/// The underlying representation is an array of [`WorkId`]s which have a type of [`WorkType::Int`] and
/// a kind of [`WorkKind::Transition`].
#[repr(C)]
pub struct TransitionGroup {
    transition_term_ids: *const WorkId,
    count: usize
}

impl TransitionGroup {
    /// Gets the underlying [`WorkId`] array
    pub fn as_slice<'a>(&'a self) -> &'a [WorkId] {
        unsafe {
            std::slice::from_raw_parts(self.transition_term_ids, self.count)
        }
    }

    /// Gets the [`WorkId`] at `index`
    pub fn get_transition_term(&self, index: usize) -> WorkId {
        self.as_slice()[index]
    }
}

impl Index<usize> for TransitionGroup {
    type Output = WorkId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

#[repr(C)]
#[vtable_impl(WorkModule)]
pub(crate) struct WorkModuleVTable {
    destructor: extern "C" fn(this: &mut WorkModule),
    deleter: extern "C" fn(this: &mut WorkModule),
    is_implemented: extern "C" fn(this: &WorkModule) -> bool,
    handle_int_msc_cmd: extern "C" fn(this: &mut WorkModule, command: &lib::MscCommand) -> lib::TValue,
    handle_float_msc_cmd: extern "C" fn(this: &mut WorkModule, command: &lib::MscCommand) -> lib::TValue,

    /// Initializes the module by copying the provided arrays/transition information and initializing the memory
    /// ### Arguments
    /// * `array_info` - The array information the module should use. This must be valid for the lifetime of the module.
    /// * `transition_term_info` - The transition term information the module should use. This must be valid for the lifetime of the module.
    /// * `transition_groups` - The transition groups the module should use. This must be valid for the lifetime of the module.
    /// * `param_object` - The structure holding on to the parameter information this object should reference. This field can be `null`
    /// * `agent_kind` - What kind of agent owns this module
    initialize: extern "C" fn(this: &mut WorkModule, array_info: &WorkArrayInfo, transition_term_info: &TransitionTermInfo, transition_groups: *const TransitionGroup, param_object: u64, agent_kind: u32),

    /// Finalizes the module by removing references to everything passed in during initialization
    finalize: extern "C" fn(this: &mut WorkModule),

    /// Starts the module by clearing all of the work storage and attaching required event listeners
    start_module: extern "C" fn(this: &mut WorkModule),

    /// Ends the module by clearing all of the work storage and removing its event listeners
    end_module: extern "C" fn(this: &mut WorkModule),

    /// Calculates all params set to be calculated at runtime, including, but not limited to, params like `jump_initial_speed_y`, `jump_initial_accel_y`, etc.
    /// 
    /// After this call, these params are accessible through the param accessor methods and are cached in the module
    pub calc_params: extern "C" fn(this: &mut WorkModule),

    /// Sets the transition term information for this module to use
    /// 
    /// This enables the user to swap out the transition term info on the fly if need be, and is called during initialization
    pub set_transition_term_info: extern "C" fn(this: &mut WorkModule, info: &TransitionTermInfo),

    /// Gets the specified float from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    pub get_float: extern "C" fn(this: &WorkModule, what: WorkId) -> f32,

    /// Sets the specified float in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variable
    /// * `what` - The work ID specify what to set
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    pub set_float: extern "C" fn(this: &mut WorkModule, value: f32, what: WorkId),

    /// Sets the specified floats in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    /// * `count` - The number of provided IDs
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for floats, it only checks the [`WorkKind`]s and the indices
    set_floats_impl: extern "C" fn(this: &mut WorkModule, value: f32, ids: *const WorkId, count: u32),

    /// Assigns a random value in the specified range to the specified float in the module's internal storage
    /// ### Arguments
    /// * `minimum` - The minimum value of the random range
    /// * `maximum` - The maximum value of the random range
    /// * `what` - The [`WorkId`] to set the value of
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    /// * If `maximum` is less than `minimum`, then the value is not set at all.
    /// * If `maximum` is equal to `minimum`, then the value is set to `maximum`
    pub random_float: extern "C" fn(this: &mut WorkModule, minimum: f32, maximum: f32, what: WorkId),

    /// Adds the specified amount to the specified float in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to add
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    pub add_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Subtracts the specified amount from the specified float in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to subtract
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    pub sub_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Multiplies the specified amount by the specified float in the module's internal storage and sets it
    /// ### Arguments
    /// * `amount` - The amount to multiply by
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    pub mul_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Divides the specified float in the module's internal storage by the specified amount
    /// ### Arguments
    /// * `amount` - The amount to divide by
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    pub div_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Gets the specified 32-bit integer from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub get_int: extern "C" fn(this: &mut WorkModule, what: WorkId) -> i32,

    /// Sets the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variable
    /// * `what` - The work ID specify what to set
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub set_int: extern "C" fn(this: &mut WorkModule, value: i32, what: WorkId),

    /// Sets the specified 32-bit integers in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    /// * `count` - The number of provided IDs
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for integers, it only checks the [`WorkKind`]s and the indices
    set_ints_impl: extern "C" fn(this: &mut WorkModule, value: i32, ids: *const WorkId, count: u32),

    /// Gets the specified 64-bit integer from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub get_int64: extern "C" fn(this: &mut WorkModule, what: WorkId) -> i64,

    /// Sets the specified 64-bit integer in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variable
    /// * `what` - The work ID specify what to set
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub set_int64: extern "C" fn(this: &mut WorkModule, value: i64, what: WorkId),

    /// Assigns a random value in the specified range to the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `minimum` - The minimum value of the random range
    /// * `maximum` - The maximum value of the random range
    /// * `what` - The [`WorkId`] to set the value of
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    /// * If `maximum` is less than `minimum`, then the value is not set at all.
    /// * If `maximum` is equal to `minimum`, then the value is set to `maximum`
    pub random_int: extern "C" fn(this: &mut WorkModule, minimum: i32, maximum: i32, what: WorkId),

    /// Increments the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifying what to increment
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub inc_int: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Decrements the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifying what to increment
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub dec_int: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Adds the specified amount to the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to add
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub add_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Subtracts the specified amount from the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to subtract
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub sub_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Multiplies the specified amount by the specified 32-bit integer in the module's internal storage and sets it
    /// ### Arguments
    /// * `amount` - The amount to multiply by
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub mul_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Divides the specified 32-bit integer in the module's internal storage by the specified amount
    /// ### Arguments
    /// * `amount` - The amount to divide by
    /// * `what` - The [`WorkId`] to change the value of
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub div_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Counts down the specified 32-bit integer until it is equal to the specified minimum
    /// ### Arguments
    /// * `what` - The [`WorkId`] to count down
    /// * `floor` - The value to count down to
    /// 
    /// ### Behavior
    /// The module will fetch the specified value and then check if it is greater than `floor`. If it is,
    /// then it decrements the value.
    /// 
    /// This call does nothing if the value was already less than or equal to floor, and returns `false`.
    /// 
    /// ### Returns
    /// * `true` - The value was greater than `floor` before the call and is equal to `floor` after the call
    /// * `false` - The value was either less than or equal to `floor` before the call **or** it is still greater than
    /// `floor` after the call
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub count_down_int: extern "C" fn(this: &mut WorkModule, what: WorkId, floor: i32) -> bool,

    /// Counts down the specified 32-bit integer until it is equal to the specified minimum
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to count down
    /// * `count` - The number of provided IDs
    /// * `floor` - The value to count down to
    /// 
    /// ### Behavior
    /// For each value, the module will fetch the specified value and then check if it is greater than `floor`. If it is,
    /// then it decrements the value, doing nothing if the value was already less than or equal to `floor`.
    /// 
    /// Unlike `count_down_int`, this call does not return anything.
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    count_down_ints_impl: extern "C" fn(this: &mut WorkModule, ids: *const WorkId, count: u32, floor: i32),

    /// Gets the specified flag from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    pub is_flag: extern "C" fn(this: &WorkModule, what: WorkId) -> bool,

    /// Sets the specified flag to true
    /// ### Arguments
    /// * `what` - The work ID specifiying which flag to enable
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    pub on_flag: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the specified flags to true
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn on
    /// * `count` - The number of provided IDs
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    on_flags_impl: extern "C" fn(this: &mut WorkModule, ids: *const WorkId, count: u32),

    /// Sets the specified flag to false
    /// ### Arguments
    /// * `what` - The work ID specifiying which flag to disable
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    pub off_flag: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the specified flags to false
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn off
    /// * `count` - The number of provided IDs
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    off_flags_impl: extern "C" fn(this: &mut WorkModule, ids: *const WorkId, count: u32),

    /// Sets the specified flag in the module's internal storage
    /// ### Arguments
    /// * `value` - The value to set the flag to
    /// * `what` - The work ID specifiying which flag to set
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    pub set_flag: extern "C" fn(this: &mut WorkModule, value: bool, what: WorkId),

    /// Sets the specified flag to false and returns true if it was previously on
    /// ### Arguments
    /// * `what` - The work ID specifying which flag to disable
    /// 
    /// ### Returns
    /// * `true` - If the flag was on before the call and was turned off by the call
    /// * `false` - If the flag was off before the call
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    pub turn_off_flag: extern "C" fn(this: &mut WorkModule, what: WorkId) -> bool,

    /// Enables the specified transition term group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to enable
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * Enabling the transition term **group** does not enable all contained transition terms, you still must call
    /// `enable_transition_term_group_ex` to enable a transition term that is within the group.
    /// * Enabling a transition term as part of a group will override being enabled by `enable_transition_term`
    /// ## Example
    /// ```rs
    /// // this code will enable the landing group and specifically enable the landing transition
    /// //
    /// // this ensures that the landing group (which includes regular and light landings)
    /// // is enabled, so that some other check can turn on specifically
    /// // light landing with `enable_transition_term_group_ex`
    /// //
    /// // if we didn't enable the landing group, then that `ex` enable wouldn't do anything when
    /// // the transition is checked
    /// fn enable_air_landing_transitions(module_accessor: &mut BattleObjectModuleAccessor) {
    ///     let wm = module_accessor.work_module();
    ///     wm.enable_transition_term_group(app::transition_groups::CHECK_AIR_LANDING);
    ///     wm.enable_transition_term_group_ex(app::transition_terms::LANDING);
    /// }
    /// ```
    pub enable_transition_term_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disabled the specified transition term group
    /// ### Arguments
    /// * `what` - The work ID specifiying which transition term **group** to disable
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * Disabling the transition term **group** does not disable all contained transition terms inherently. If any of them
    /// have been enabled by a call to `enable_transition_term`, they will still be enabled after this call.
    pub unable_transition_term_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Clears both group flags for all flags in the transition term group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to clear
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This call has the same behavior as calling both `unable_transition_term_group` and `unable_transition_term_group_ex_all` with the argument `what`
    pub clear_transition_term_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Checks if the transition term group is enabled
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to check
    /// 
    /// ### Behavior
    /// This call will check the first transition term in the group exclusively to see if it's enabled. This means that if any
    /// transition term groups share the same first term in their list, the result of this function can be incorrect. Fortunately,
    /// none of the groups overlap that way
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term group, it only checks the index
    pub is_enable_transition_term_group: extern "C" fn(this: &WorkModule, what: WorkId) -> bool,

    /// Enables the EX flag on the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to enable the EX flag on
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * Calling this function (and by extension setting the flag) will do nothing until the group flag is also set
    /// for this transition term (can be set by calling `enable_transition_term_group` on a group that contains the flag)
    pub enable_transition_term_group_ex: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Enables the EX flag on all of the transition terms of the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to enable the EX flag on
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * Pairing this function to a call of `enable_transition_term_group` with the same group ID effectively enables
    /// every transition term in the group
    pub enable_transition_term_group_ex_all: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the EX flag on the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to disable the EX flag on
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * Calling this function (and by extension disabling the flag) will do nothing if the group flag is not also set
    /// for this transition term (can be set by calling `enable_transition_term_group` on a group that contains the flag)
    pub unable_transition_term_group_ex: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the EX flag on all of the transition terms of the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to disable the EX flag on
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term group, it only checks the index
    pub unable_transition_term_group_ex_all: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Checks if the transition term is enabled.
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to check
    /// 
    /// ### Behavior
    /// This call does all of the checks on the transition term, including checking if it's in a group or if it's forbidden.
    /// 
    /// It's functionality is:
    /// ```rs
    /// let term = get_term(what);
    /// if term.is_forbidden() {
    ///     return false;
    /// } 
    /// 
    /// if term.is_enabled_group() {
    ///     return term.is_enabled_group_ex();   
    /// } else {
    ///     return term.is_enabled();
    /// }
    /// ```
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term, it only checks the index
    pub is_enable_transition_term: extern "C" fn(this: &WorkModule, what: WorkId) -> bool,

    /// Enables the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to enable
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This call will set the flag regardless of whether or not this is enabled as part of a group, but if
    /// it is enabled as part of a group the flag means nothing.
    pub enable_transition_term: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to disable
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This call will disable the flag regardless of whether or not this is enabled as part of a group, but if
    /// it is enabled as part of a group the flag means nothing.
    pub unable_transition_term: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables all transition terms in the module
    pub clear_transition_term: extern "C" fn(this: &mut WorkModule),

    /// Checks if any of the `forbid` flags are set for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to check
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term, it only checks the index
    pub is_enable_transition_term_forbid: extern "C" fn(this: &WorkModule) -> bool,

    /// Sets the [`TransitionTerm::ENABLE_FORBID`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    pub enable_transition_term_forbid: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to no longer forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    pub unable_transition_term_forbid: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the [`TransitionTerm::ENABLE_FORBID`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    pub enable_transition_term_forbid_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to no longer forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    pub unable_transition_term_forbid_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID`] flag for all transition terms in the module
    /// 
    /// ### Notes
    /// This only disables one of the three flags which can forbid a transition term
    pub clear_transition_term_forbid: extern "C" fn(this: &mut WorkModule),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    pub enable_transition_term_forbid_other: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to no longer forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    pub unable_transition_term_forbid_other: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    pub enable_transition_term_forbid_group_other: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to no longer forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    pub unable_transition_term_forbid_group_other: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for all transition terms in the module
    /// 
    /// ### Notes
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    pub clear_transition_term_forbid_other: extern "C" fn(this: &mut WorkModule),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    pub enable_transition_term_forbid_indivi: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to no longer forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    pub unable_transition_term_forbid_indivi: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    pub enable_transition_term_forbid_group_indivi: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to no longer forbid
    /// 
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    pub unable_transition_term_forbid_group_indivi: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for all transition terms in the module
    /// 
    /// ### Notes
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    pub clear_transition_term_forbid_indivi: extern "C" fn(this: &mut WorkModule),

    /// Re-initializes all of the internal storage to 0 (floats are `0.0`, ints are `0`, and flags are `false`)
    pub clear_all: extern "C" fn(this: &mut WorkModule),

    /// Clears the status arrays of the module's internal storage
    /// ### Arguments
    /// * `keep_float` - Bitmask of the first 32 status float variables to keep (-1 keeps all)
    /// * `keep_int` - Bitmask of the first 32 status ints to keep (-1 keeps all)
    /// * `keep_flag` - Bitmask of the first 32 status flags to keep (-1 keeps all)
    /// 
    /// ### Notes
    /// * Passing `0` for all three arguments causes the implementation to turn into a `memset` call
    /// * If there are more than 32 variables for the status, you either clear everything (pass in 0 for all three arguments)
    /// or you anything with index 32 and greater must be cleared/kept manually
    /// * This function is called by `StatusModule::init_settings`
    /// * This function is not exported by the main executable, the name is assumed
    pub clear_status: extern "C" fn(this: &mut WorkModule, keep_float: i32, keep_int: i32, keep_flag: i32),

    /// Gets the internal work array info structure
    ///
    /// ### Notes
    /// This function is not exported by the main executable, the name is assumed
    get_work_array_info: extern "C" fn(this: &mut WorkModule) -> *mut WorkArrayInfo,

    /// Gets the specified internal work array
    /// 
    /// ### Notes
    /// This function is not exported by the main executable, the name is assumed
    get_work_array: extern "C" fn(this: &mut WorkModule) -> *mut WorkArray,

    unk_3: extern "C" fn(this: &mut WorkModule),

    /// Gets a 32-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    /// 
    /// ### Notes
    /// * This function is called internally by `get_param_int`
    /// * This function is not exported by the main executable, the name is assumed
    get_param_int_impl: extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i32,

    /// Gets a 32-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    pub get_param_int: extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i32,

    /// Gets a 64-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    /// 
    /// ### Notes
    /// * This function is called internally by `get_param_int64`
    /// * This function is not exported by the main executable, the name is assumed
    get_param_int64_impl: extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i64,

    /// Gets a 64-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    pub get_param_int64: extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i64,

    /// Gets a float from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    /// 
    /// ### Notes
    /// * This function is called internally by `get_param_float`
    /// * This function is not exported by the main executable, the name is assumed
    get_param_float_impl: extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> f32,

    /// Gets a float from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    pub get_param_float: extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> f32,

    /// Sets the move customization fields of the param object
    /// ### Arguments
    /// * `variant` - The variant number of the move
    /// * `move_no` - The number of the move to customize
    /// 
    /// ### Notes
    /// * This is only really used by mii fighters and joker that we are aware of (joker for arsene, mii fighters for their specials)
    /// * This is associated with the `FIGHTER_WAZA_CUSTOMIZE` constants and work IDs
    pub set_customize_no: extern "C" fn(this: &mut WorkModule, variant: i32, move_no: i32),

    /// Recalculates runtime-cached params based off of a stat change.
    /// ### Arguments
    /// * `_unused` - Doesn't appear to be used, is usually `0`
    /// * `is_abnormal` - Set if the stat change is not your normal stats (i.e. picking up a bunny hood)
    /// 
    /// ### Notes
    /// * This can be triggered when you pick up an item that changes some stats for the fighter, such as a bunny hood, mushroom, or
    /// getting struck by lightning
    /// * This function sends an event with ID `0x31`
    /// * This function is not exported by the main executable, the name is assumed
    pub realculate_params: extern "C" fn(this: &mut WorkModule, _unused: u64, is_abnormal: bool)
}

#[cfg(feature = "type_assert")]
impl WorkModuleVTable {
    pub fn assert() {
        assert_eq!(size_of!(WorkModuleVTable), 0x288);
        assert_eq!(offset_of!(WorkModuleVTable, destructor),                                 0x000);
        assert_eq!(offset_of!(WorkModuleVTable, deleter),                                    0x008);
        assert_eq!(offset_of!(WorkModuleVTable, is_implemented),                             0x010);
        assert_eq!(offset_of!(WorkModuleVTable, handle_int_msc_cmd),                         0x018);
        assert_eq!(offset_of!(WorkModuleVTable, handle_float_msc_cmd),                       0x020);
        assert_eq!(offset_of!(WorkModuleVTable, initialize),                                 0x028);
        assert_eq!(offset_of!(WorkModuleVTable, finalize),                                   0x030);
        assert_eq!(offset_of!(WorkModuleVTable, start_module),                               0x038);
        assert_eq!(offset_of!(WorkModuleVTable, end_module),                                 0x040);
        assert_eq!(offset_of!(WorkModuleVTable, calc_params),                                0x048);
        assert_eq!(offset_of!(WorkModuleVTable, set_transition_term_info),                   0x050);
        assert_eq!(offset_of!(WorkModuleVTable, get_float),                                  0x058);
        assert_eq!(offset_of!(WorkModuleVTable, set_float),                                  0x060);
        assert_eq!(offset_of!(WorkModuleVTable, set_floats_impl),                            0x068);
        assert_eq!(offset_of!(WorkModuleVTable, random_float),                               0x070);
        assert_eq!(offset_of!(WorkModuleVTable, add_float),                                  0x078);
        assert_eq!(offset_of!(WorkModuleVTable, sub_float),                                  0x080);
        assert_eq!(offset_of!(WorkModuleVTable, mul_float),                                  0x088);
        assert_eq!(offset_of!(WorkModuleVTable, div_float),                                  0x090);
        assert_eq!(offset_of!(WorkModuleVTable, get_int),                                    0x098);
        assert_eq!(offset_of!(WorkModuleVTable, set_int),                                    0x0A0);
        assert_eq!(offset_of!(WorkModuleVTable, set_ints_impl),                              0x0A8);
        assert_eq!(offset_of!(WorkModuleVTable, get_int64),                                  0x0B0);
        assert_eq!(offset_of!(WorkModuleVTable, set_int64),                                  0x0B8);
        assert_eq!(offset_of!(WorkModuleVTable, random_int),                                 0x0C0);
        assert_eq!(offset_of!(WorkModuleVTable, inc_int),                                    0x0C8);
        assert_eq!(offset_of!(WorkModuleVTable, dec_int),                                    0x0D0);
        assert_eq!(offset_of!(WorkModuleVTable, add_int),                                    0x0D8);
        assert_eq!(offset_of!(WorkModuleVTable, sub_int),                                    0x0E0);
        assert_eq!(offset_of!(WorkModuleVTable, mul_int),                                    0x0E8);
        assert_eq!(offset_of!(WorkModuleVTable, div_int),                                    0x0F0);
        assert_eq!(offset_of!(WorkModuleVTable, count_down_int),                             0x0F8);
        assert_eq!(offset_of!(WorkModuleVTable, count_down_ints_impl),                       0x100);
        assert_eq!(offset_of!(WorkModuleVTable, is_flag),                                    0x108);
        assert_eq!(offset_of!(WorkModuleVTable, on_flag),                                    0x110);
        assert_eq!(offset_of!(WorkModuleVTable, on_flags_impl),                              0x118);
        assert_eq!(offset_of!(WorkModuleVTable, off_flag),                                   0x120);
        assert_eq!(offset_of!(WorkModuleVTable, off_flags_impl),                             0x128);
        assert_eq!(offset_of!(WorkModuleVTable, set_flag),                                   0x130);
        assert_eq!(offset_of!(WorkModuleVTable, turn_off_flag),                              0x138);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_group),               0x140);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_group),               0x148);
        assert_eq!(offset_of!(WorkModuleVTable, clear_transition_term_group),                0x150);
        assert_eq!(offset_of!(WorkModuleVTable, is_enable_transition_term_group),            0x158);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_group_ex),            0x160);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_group_ex_all),        0x168);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_group_ex),            0x170);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_group_ex_all),        0x178);
        assert_eq!(offset_of!(WorkModuleVTable, is_enable_transition_term),                  0x180);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term),                     0x188);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term),                     0x190);
        assert_eq!(offset_of!(WorkModuleVTable, clear_transition_term),                      0x198);
        assert_eq!(offset_of!(WorkModuleVTable, is_enable_transition_term_forbid),           0x1A0);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_forbid),              0x1A8);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_forbid),              0x1B0);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_forbid_group),        0x1B8);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_forbid_group),        0x1C0);
        assert_eq!(offset_of!(WorkModuleVTable, clear_transition_term_forbid),               0x1C8);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_forbid_other),        0x1D0);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_forbid_other),        0x1D8);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_forbid_group_other),  0x1E0);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_forbid_group_other),  0x1E8);
        assert_eq!(offset_of!(WorkModuleVTable, clear_transition_term_forbid_other),         0x1F0);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_forbid_indivi),       0x1F8);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_forbid_indivi),       0x200);
        assert_eq!(offset_of!(WorkModuleVTable, enable_transition_term_forbid_group_indivi), 0x208);
        assert_eq!(offset_of!(WorkModuleVTable, unable_transition_term_forbid_group_indivi), 0x210);
        assert_eq!(offset_of!(WorkModuleVTable, clear_transition_term_forbid_indivi),        0x218);
        assert_eq!(offset_of!(WorkModuleVTable, clear_all),                                  0x220);
        assert_eq!(offset_of!(WorkModuleVTable, clear_status),                               0x228);
        assert_eq!(offset_of!(WorkModuleVTable, get_work_array_info),                        0x230);
        assert_eq!(offset_of!(WorkModuleVTable, get_work_array),                             0x238);
        assert_eq!(offset_of!(WorkModuleVTable, unk_3),                                      0x240);
        assert_eq!(offset_of!(WorkModuleVTable, get_param_int_impl),                         0x248);
        assert_eq!(offset_of!(WorkModuleVTable, get_param_int),                              0x250);
        assert_eq!(offset_of!(WorkModuleVTable, get_param_int64_impl),                       0x258);
        assert_eq!(offset_of!(WorkModuleVTable, get_param_int64),                            0x260);
        assert_eq!(offset_of!(WorkModuleVTable, get_param_float_impl),                       0x268);
        assert_eq!(offset_of!(WorkModuleVTable, get_param_float),                            0x270);
        assert_eq!(offset_of!(WorkModuleVTable, set_customize_no),                           0x278);
        assert_eq!(offset_of!(WorkModuleVTable, realculate_params),                          0x280);
    }
}

#[repr(C)]
pub struct WorkModule {
    vtable: &'static WorkModuleVTable,
    owner: *mut app::BattleObjectModuleAccessor,
}

impl WorkModule {
    /// Sets the specified floats in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for floats, it only checks the [`WorkKind`]s and the indices
    pub fn set_floats(&mut self, value: f32, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.set_floats_impl(value, ids.as_ptr(), ids.len() as u32)
    }

    /// Sets the specified 32-bit integers in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for integers, it only checks the [`WorkKind`]s and the indices
    pub fn set_ints(&mut self, value: i32, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.set_ints_impl(value, ids.as_ptr(), ids.len() as u32)
    }

    /// Counts down the specified 32-bit integer until it is equal to the specified minimum
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to count down
    /// * `floor` - The value to count down to
    /// 
    /// ### Behavior
    /// For each value, the module will fetch the specified value and then check if it is greater than `floor`. If it is,
    /// then it decrements the value, doing nothing if the value was already less than or equal to `floor`.
    /// 
    /// Unlike `count_down_int`, this call does not return anything.
    /// 
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub fn count_down_ints(&mut self, ids: impl AsRef<[WorkId]>, floor: i32) {
        let ids = ids.as_ref();
        self.count_down_ints_impl(ids.as_ptr(), ids.len() as u32, floor)
    }

    /// Sets the specified flags to true
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn on
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    pub fn on_flags(&mut self, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.on_flags_impl(ids.as_ptr(), ids.len() as u32)
    }

    /// Sets the specified flags to false
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn off
    /// * `count` - The number of provided IDs
    /// 
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    pub fn off_flags(&mut self, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.off_flags_impl(ids.as_ptr(), ids.len() as u32)
    }
}

#[repr(C)]
#[virtual_implementor(WorkModule)]
pub struct FighterWorkModuleImpl {
    parent: WorkModule,
    work_array_info: WorkArrayInfo,
    transition_term_info: TransitionTermInfo,
    transition_groups: *const *const TransitionGroup, // technically this is a pointer to (*const TransitionGroup, usize) but the size is never checked
    param_object: u64,
    fighter_kind: app::FighterKind,
    entry_id: app::FighterEntryID,
    _x48: u64,
    fighter_info: *mut app::FighterInformation,
    // more
}

#[cfg(feature = "type_assert")]
impl FighterWorkModuleImpl {
    pub fn assert() {
        assert_eq!(offset_of!(FighterWorkModuleImpl, parent), 0x0);
        assert_eq!(offset_of!(FighterWorkModuleImpl, work_array_info), 0x10);
        assert_eq!(offset_of!(FighterWorkModuleImpl, transition_term_info), 0x20);
        assert_eq!(offset_of!(FighterWorkModuleImpl, transition_groups), 0x30);
        assert_eq!(offset_of!(FighterWorkModuleImpl, param_object), 0x38);
        assert_eq!(offset_of!(FighterWorkModuleImpl, fighter_kind), 0x40);
        assert_eq!(offset_of!(FighterWorkModuleImpl, entry_id), 0x44);
        assert_eq!(offset_of!(FighterWorkModuleImpl, _x48), 0x48);
        assert_eq!(offset_of!(FighterWorkModuleImpl, fighter_info), 0x50);
    }
}