use std::{ops::{Index, IndexMut}, mem::MaybeUninit};

use crate::*;

extern "C" {
    #[link_name = "\u{1}_ZN3lib8L2CTableC1Ei"]
    fn table_ctor(table: *mut L2CTable, array_cap: i32);

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZN3lib8L2CTable14find_metatableERKN3phx6Hash40E"]
    fn find_metatable(table: *mut L2CTable, key: *const phx::Hash40) -> *mut lib::L2CValue;
}

#[repr(C)]
pub struct L2CTable {
    ref_count: u32,
    padding: u32,
    array: cpp::Vector<lib::L2CValue>,
    map: cpp::Tree<phx::Hash40, lib::L2CValue>,
    agent: *mut lib::L2CAgent,
    metatable: *mut L2CTable
}

impl L2CTable {
    pub(crate) fn initialize_raw_memory(this: *mut Self, cap: i32) {
        unsafe {
            table_ctor(this, cap);
        }
    }

    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }

    pub fn new_with_capacity(capacity: i32) -> Self {
        unsafe {
            let mut table = MaybeUninit::uninit();
            table_ctor(table.as_mut_ptr(), capacity);
            table.assume_init()
        }
    }

    pub fn rc(&self) -> usize {
        self.ref_count as usize
    }

    pub fn array_length(&self) -> usize {
        self.array.len()
    }

    pub fn array_iter(&self) -> cpp::VectorIter<lib::L2CValue> {
        self.array.iter()
    }

    pub fn map_length(&self) -> usize {
        self.map.len()
    }

    pub fn map_iter(&self) -> cpp::TreeKeyValueIter<phx::Hash40, lib::L2CValue> {
        self.map.iter()
    }

    pub fn metatable(&self) -> Option<&L2CTable> {
        if self.metatable.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.metatable)
            }
        }
    }

    pub fn metatable_mut(&mut self) -> Option<&mut L2CTable> {
        if self.metatable.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.metatable)
            }
        }
    }

    pub fn get_array(&self, index: usize) -> Option<&lib::L2CValue> {
        self.array.get(index)
    }

    pub fn get_array_mut(&mut self, index: usize) -> Option<&mut lib::L2CValue> {
        self.array.get_mut(index)
    }

    pub fn get_map(&self, key: phx::Hash40) -> Option<&lib::L2CValue> {
        self.map.get(&key)
    }

    pub fn get_map_mut(&mut self, key: phx::Hash40) -> Option<&mut lib::L2CValue> {
        self.map.get_mut(&key)
    }
}

impl Index<usize> for L2CTable {
    type Output = lib::L2CValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl IndexMut<usize> for L2CTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl Index<phx::Hash40> for L2CTable {
    type Output = lib::L2CValue;

    fn index(&self, index: phx::Hash40) -> &Self::Output {
        &self.map[index]
    }
}

impl IndexMut<phx::Hash40> for L2CTable {
    fn index_mut(&mut self, index: phx::Hash40) -> &mut Self::Output {
        &mut self.map[index]
    }
}

#[cfg(feature = "type_assert")]
impl L2CTable {
    pub fn assert() {
        assert_eq!(size_of!(L2CTable), 0x48);
        assert_eq!(offset_of!(L2CTable, ref_count), 0x0);
        assert_eq!(offset_of!(L2CTable, array), 0x8);
        assert_eq!(offset_of!(L2CTable, map), 0x20);
        assert_eq!(offset_of!(L2CTable, agent), 0x38);
        assert_eq!(offset_of!(L2CTable, metatable), 0x40);
    }
}