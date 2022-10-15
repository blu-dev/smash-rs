use crate::*;

#[repr(u64)]
pub enum NfpPersonalityKind {
    Unk0  = 0x00,
    Unk1  = 0x01,
    Unk2  = 0x02,
    Unk3  = 0x03,
    Unk4  = 0x04,
    Unk5  = 0x05,
    Unk6  = 0x06,
    Unk7  = 0x07,
    Unk8  = 0x08,
    Unk9  = 0x09,
    Unk10 = 0x0A,
    Unk11 = 0x0B,
    Unk12 = 0x10,
    Unk13 = 0x11,
    Unk14 = 0x12,
    Unk15 = 0x13,
    Unk16 = 0x14,
    Unk17 = 0x15,
    Unk18 = 0x22,
    Unk19 = 0x23,
    Unk20 = 0x24,
    Unk21 = 0x25,
    Unk22 = 0x26,
    Unk23 = 0x27,
    Unk24 = 0x28,
    Unk25 = 0x29,
    Unk26 = 0x2A,
    Unk27 = 0x2B,
    Unk28 = 0x2C,
    Unk29 = 0x2D,
    Unk30 = 0x2E,
    Unk31 = 0x2F,
    Unk32 = 0x30,
    Unk33 = 0x31,
    Unk34 = 0x32,
    Unk35 = 0x33,
    Unk36 = 0x34,
    Unk37 = 0x35,
    Unk38 = 0x36,
    Unk39 = 0x37,
    Unk40 = 0x38,
    Unk41 = 0x39,
    Unk42 = 0x3A,
    Unk43 = 0x3B,
    Unk44 = 0x3F,
    Unk45 = 0x40,
    Unk46 = 0x41,
    Unk47 = 0x42,
    Unk48 = 0x43,
    Unk49 = 0x44,
    Unk50 = 0x45,
    Unk51 = 0x46
}

mod impl_ {
    use crate::*;
    use super::NfpPersonalityKind;

    extern "C" {
        #[link_name = "_ZN3app3nfp10is_enabledEP9lua_State"]
        pub(super) fn is_enabled(state: *mut lua_State) -> bool;

        #[link_name = "_ZN3app3nfp11param_floatEN3phx6Hash40E"]
        pub(super) fn param_float(hash: crate::phx::Hash40) -> f32;

        #[link_name = "_ZN3app3nfp13has_air_catchENS_11FighterKindE"]
        pub(super) fn has_air_catch(fighter_kind: app::FighterKind) -> bool;

        #[link_name = "_ZN3app3nfp16personality_rateEP9lua_StateNS_18NfpPersonalityKindE"]
        pub(super) fn personality_rate(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32;

        #[link_name = "_ZN3app3nfp19personality_up_rateEP9lua_StateNS_18NfpPersonalityKindE"]
        pub(super) fn personality_up_rate(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32;

        #[link_name = "_ZN3app3nfp21personality_down_rateEP9lua_StateNS_18NfpPersonalityKindE"]
        pub(super) fn personality_down_rate(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32;

        #[link_name = "_ZN3app3nfp23personality_probabilityEP9lua_StateNS_18NfpPersonalityKindE"]
        pub(super) fn personality_probability(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32;

        #[link_name = "_ZN3app3nfp24personality_rate_inverseEP9lua_StateNS_18NfpPersonalityKindE"]
        pub(super) fn personality_rate_inverse(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32;

        #[link_name = "_ZN3app3nfp31personality_probability_inverseEP9lua_StateNS_18NfpPersonalityKindE"]
        pub(super) fn personality_probability_inverse(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32;
    }

}

pub fn is_enabled(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_enabled(state)
    }
}

pub fn param_float(hash: crate::phx::Hash40) -> f32 {
    unsafe {
        impl_::param_float(hash)

    }
}

pub fn has_air_catch(fighter_kind: app::FighterKind) -> bool {
    unsafe {
        impl_::has_air_catch(fighter_kind)
    }
}

pub fn personality_rate(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32 {
    unsafe {
        impl_::personality_rate(state, personality_kind)
    }
}

pub fn personality_up_rate(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32 {
    unsafe {
        impl_::personality_up_rate(state, personality_kind)
    }
}

pub fn personality_down_rate(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32 {
    unsafe {
        impl_::personality_down_rate(state, personality_kind)
    }
}

pub fn personality_probability(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32 {
    unsafe {
        impl_::personality_probability(state, personality_kind)
    }
}

pub fn personality_rate_inverse(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32 {
    unsafe {
        impl_::personality_rate_inverse(state, personality_kind)
    }
}

pub fn personality_probability_inverse(state: *mut lua_State, personality_kind: NfpPersonalityKind) -> f32 {
    unsafe {
        impl_::personality_probability_inverse(state, personality_kind)
    }
}