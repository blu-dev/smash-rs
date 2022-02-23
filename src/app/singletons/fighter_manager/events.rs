use super::*;
use std::ops::{Deref, DerefMut};

/// The event IDs that the game uses internally for it's fighter events
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum FighterEventID {
    UIDamageUpdate = 0x11,
    JackUpdateRebelGauge = 0x58
}

macro_rules! impl_event {
    ($name:tt, $id:tt) => {
        #[sealed::sealed]
        impl FighterEventInheriter for $name {
            fn get_fighter_event_id() -> FighterEventID {
                FighterEventID::$id
            }
        }

        impl Deref for $name {
            type Target = FighterEvent;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        impl AsRef<FighterEvent> for $name {
            fn as_ref(&self) -> &FighterEvent {
                &self.base
            }
        }

        impl AsMut<FighterEvent> for $name {
            fn as_mut(&mut self) -> &mut FighterEvent {
                &mut self.base
            }
        }
    };
    ($(($name:tt, $id:tt))*) => {
        $(
            impl_event!($name, $id);
        )*
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct UIDamageUpdateEvent {
    base: FighterEvent,
    pub total_damage: f32,
    pub change_in_damage: f32,
    pub is_increase: bool,
    pub is_decrease: bool
}

impl UIDamageUpdateEvent {
    pub fn new(entry_id: app::FighterEntryID, total_damage: f32, change_in_damage: f32, is_damage: bool, is_heal: bool) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::UIDamageUpdate, entry_id),
            total_damage,
            change_in_damage,
            is_increase: is_damage,
            is_decrease: is_heal
        }
    }

    pub fn is_absolute(&self) -> bool {
        !self.is_increase && !self.is_decrease
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct JackUpdateRebelGaugeEvent {
    base: FighterEvent,
    pub ratio: f32
}

impl JackUpdateRebelGaugeEvent {
    pub fn new(entry_id: app::FighterEntryID, ratio: f32) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::JackUpdateRebelGauge, entry_id),
            ratio
        }
    }
}

impl_event!(
    (JackUpdateRebelGaugeEvent, JackUpdateRebelGauge)
    (UIDamageUpdateEvent, UIDamageUpdate)

);