use crate::*;
use std::fmt;
use skyline::libc::{
    c_void,
    c_char
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    Nil = 0,
    Bool = 1,
    Integer = 2,
    Number = 3,
    UserData = 4,
    Table = 5,
    InnerFunction = 6,
    Hash = 7,
    String = 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union InnerData {
    pub integer: u64,
    pub float: f32,
    pub user_data: *mut c_void,
    pub table: *mut lib::L2CTable,
    pub inner_function: *mut c_void,
    pub hash: phx::Hash40,
    pub c_string: *const c_char
}

#[repr(C)]
pub struct L2CValue {
    pub ty: ValueType,
    pub padding: u32,
    pub raw: InnerData
}

impl L2CValue {
    pub fn is_nil(&self) -> bool {
        matches!(self.ty, ValueType::Nil)
    }

    pub fn try_bool(&self) -> Option<bool> {
        unsafe {
            if matches!(self.ty, ValueType::Bool) {
                Some(self.raw.integer != 0)
            } else {
                None
            }
        }
    }

    pub fn try_integer(&self) -> Option<u64> {
        unsafe {
            if matches!(self.ty, ValueType::Integer) {
                Some(self.raw.integer)
            } else {
                None
            }
        }
    }

    pub fn try_float(&self) -> Option<f32> {
        unsafe {
            if matches!(self.ty, ValueType::Number) {
                Some(self.raw.float)
            } else {
                None
            }
        }
    }

    pub fn try_pointer(&self) -> Option<*mut c_void> {
        unsafe {
            if matches!(self.ty, ValueType::UserData) {
                if self.raw.user_data.is_null() {
                    None
                } else {
                    Some(self.raw.user_data)
                }
            } else {
                None
            }
        }
    }

    pub fn try_table(&self) -> Option<&lib::L2CTable> {
        unsafe {
            if matches!(self.ty, ValueType::Table) {
                if self.raw.table.is_null() {
                    None
                } else {
                    Some(&*self.raw.table)
                }
            } else {
                None
            }
        }
    }

    pub fn try_table_mut(&mut self) -> Option<&mut lib::L2CTable> {
        unsafe {
            if matches!(self.ty, ValueType::Table) {
                if self.raw.table.is_null() {
                    None
                } else {
                    Some(&mut *self.raw.table)
                }
            } else {
                None
            }
        }
    }
}

impl fmt::Display for L2CValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            ValueType::Nil if f.alternate() => write!(f, "(nil)"),
            ValueType::Nil => write!(f, "nil"),
            ValueType::Bool if f.alternate() => write!(f, "Bool({})", self.try_bool().unwrap()),
            ValueType::Bool => write!(f, "{}", self.try_bool().unwrap()),
            ValueType::Integer if f.alternate() => write!(f, "Integer({})", self.try_integer().unwrap()),
            ValueType::Integer => write!(f, "{}", self.try_integer().unwrap()),
            ValueType::Number if f.alternate() => write!(f, "Number({})", self.try_float().unwrap()),
            ValueType::Number => write!(f, "{}", self.try_float().unwrap()),
            ValueType::UserData if f.alternate() => write!(f, "UserData({:#x})", self.try_pointer().unwrap() as u64),
            ValueType::UserData => write!(f, "{:#x}", self.try_pointer().unwrap() as u64),
            ValueType::Table => write!(f, "Table"),
            ValueType::InnerFunction => write!(f, "InnerFunction"),
            ValueType::Hash => write!(f, "Hash"),
            ValueType::String => write!(f, "String")
        }
    }
}