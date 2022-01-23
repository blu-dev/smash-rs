use std::mem::MaybeUninit;

use crate::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind35lib__Rect__load_from_l2c_table_implEPN3lib4RectERKNS1_8L2CValueE"]
    fn load_from_l2c_table(rect: *mut Rect, table: *const lib::L2CValue);

    #[link_name = "\u{1}_ZN3app8lua_bind31lib__Rect__store_l2c_table_implEPKN3lib4RectE"]
    fn store_l2c_table(rect: *const Rect) -> lib::L2CValueHack;
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

impl Rect {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom
        }
    }

    pub fn area(&self) -> f32 {
        let width = (self.right - self.left).abs();
        let height = (self.top - self.bottom).abs();
        width * height
    }
}

impl From<lib::L2CValue> for Rect {
    fn from(val: lib::L2CValue) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            load_from_l2c_table(value.as_mut_ptr(), &val);
            value.assume_init()
        }
    }
}

impl From<&lib::L2CValue> for Rect {
    fn from(val: &lib::L2CValue) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            load_from_l2c_table(value.as_mut_ptr(), val);
            value.assume_init()
        }
    }
}

impl Into<lib::L2CValue> for Rect {
    fn into(self) -> lib::L2CValue {
        unsafe {
            store_l2c_table(&self).into()
        }
    }
}

impl From<cpp::simd::Vector4> for Rect {
    fn from(other: cpp::simd::Vector4) -> Self {
        Rect {
            left: other.x,
            right: other.y,
            top: other.z,
            bottom: other.w
        }
    }
}

impl Into<cpp::simd::Vector4> for Rect {
    fn into(self) -> cpp::simd::Vector4 {
        cpp::simd::Vector4 {
            x: self.left,
            y: self.right,
            z: self.top,
            w: self.bottom
        }
    }
}

#[cfg(feature = "type_assert")]
impl Rect {
    pub fn assert() {
        assert_eq!(size_of!(Rect), 0x10);
        assert_eq!(offset_of!(Rect, left),   0x0);
        assert_eq!(offset_of!(Rect, right),  0x4);
        assert_eq!(offset_of!(Rect, top),    0x8);
        assert_eq!(offset_of!(Rect, bottom), 0xC);
    }
}