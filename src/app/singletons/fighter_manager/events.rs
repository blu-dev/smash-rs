use super::*;
use std::ops::{Deref, DerefMut};

/// The event IDs that the game uses internally for it's fighter events
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum FighterEventID {
    JackUpdateRebelGuage = 0x58
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
    ($(($name:tt, $id:tt)),*) => {
        $(
            impl_as_ref_event!($name)
        )*
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct JackUpdateRebelGuageEvent {
    base: FighterEvent,
    pub ratio: f32
}

impl JackUpdateRebelGuageEvent {
    pub fn new(ratio: f32, entry_id: app::FighterEntryID) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::JackUpdateRebelGuage, entry_id),
            ratio
        }
    }
}

impl_event!(JackUpdateRebelGuageEvent, JackUpdateRebelGuage);