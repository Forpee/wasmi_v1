use serde::{Deserialize, Serialize};
use wasmi_core::ValueType;

use super::mtable::LocationType;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MemoryReadSize {
    U8 = 1,
    S8,
    U16,
    S16,
    U32,
    S32,
    I64,
    F32,
    F64,
}

impl MemoryReadSize {
    pub fn byte_size(&self) -> usize {
        match self {
            MemoryReadSize::U8 => 1,
            MemoryReadSize::S8 => 1,
            MemoryReadSize::U16 => 2,
            MemoryReadSize::S16 => 2,
            MemoryReadSize::U32 => 4,
            MemoryReadSize::S32 => 4,
            MemoryReadSize::I64 => 8,
            MemoryReadSize::F32 => 4,
            MemoryReadSize::F64 => 8,
        }
    }

    pub fn encode(self) -> u8 {
        match self {
            MemoryReadSize::U8 => 0,
            MemoryReadSize::S8 => 1,
            MemoryReadSize::U16 => 2,
            MemoryReadSize::S16 => 3,
            MemoryReadSize::U32 => 4,
            MemoryReadSize::S32 => 5,
            MemoryReadSize::I64 => 6,
            MemoryReadSize::F32 => 7,
            MemoryReadSize::F64 => 8,
        }
    }

    pub fn decode(byte: u8) -> Self {
        match byte {
            0 => Self::U8,
            1 => Self::S8,
            2 => Self::U16,
            3 => Self::S16,
            4 => Self::U32,
            5 => Self::S32,
            6 => Self::I64,
            7 => Self::F32,
            8 => Self::F64,
            _ => panic!("invalid type"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MemoryStoreSize {
    Byte8 = 1,
    Byte16,
    Byte32,
    Byte64,
}

impl MemoryStoreSize {
    pub fn byte_size(&self) -> usize {
        match self {
            MemoryStoreSize::Byte8 => 1,
            MemoryStoreSize::Byte16 => 2,
            MemoryStoreSize::Byte32 => 4,
            MemoryStoreSize::Byte64 => 8,
        }
    }

    pub fn encode(self) -> u8 {
        match self {
            Self::Byte8 => 0,
            Self::Byte16 => 1,
            Self::Byte32 => 2,
            Self::Byte64 => 3,
        }
    }

    pub fn decode(byte: u8) -> Self {
        match byte {
            0 => Self::Byte8,
            1 => Self::Byte16,
            2 => Self::Byte32,
            3 => Self::Byte64,
            _ => panic!("invalid type"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum VarType {
    I64 = 0,
    I32 = 1,
    F32 = 2,
    F64 = 3,
    FuncRef = 4,
    ExternRef = 5,
}

impl VarType {
    pub fn decode(byte: u8) -> Self {
        match byte {
            0 => VarType::I64,
            1 => VarType::I32,
            2 => VarType::F32,
            3 => VarType::F64,
            4 => VarType::FuncRef,
            5 => VarType::ExternRef,
            _ => panic!("invalid type"),
        }
    }
}

impl From<ValueType> for VarType {
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::I32 => Self::I32,
            ValueType::I64 => Self::I64,
            ValueType::F32 => Self::F32,
            ValueType::F64 => Self::F64,
            ValueType::FuncRef => Self::FuncRef,
            ValueType::ExternRef => Self::ExternRef,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IMTableEntry {
    pub ltype: LocationType,
    pub is_mutable: bool,
    pub start_offset: u32,
    pub end_offset: u32,
    pub vtype: VarType,
    /// convert from [u8; 8] via u64::from_le_bytes
    pub value: u64,
}

#[derive(Debug, Default)]
pub struct IMTable(Vec<IMTableEntry>);

impl IMTable {
    pub fn push(
        &mut self,
        is_global: bool,
        is_mutable: bool,
        start_offset: u32,
        end_offset: u32,
        vtype: VarType,
        value: u64,
    ) {
        self.0.push(IMTableEntry {
            is_mutable,
            ltype: if is_global {
                LocationType::Global
            } else {
                LocationType::Heap
            },
            start_offset,
            end_offset,
            vtype,
            value,
        })
    }

    pub fn try_find(&self) -> Option<(u32, u32, u64)> {
        Some((0, 0, 0))
        // match ltype {
        //     LocationType::Heap => {
        //         let idx = self
        //             .sorted_heap_init_entries
        //             .binary_search_by(|entry| {
        //                 if offset >= entry.start_offset && offset <= entry.end_offset {
        //                     Ordering::Equal
        //                 } else if offset < entry.start_offset {
        //                     Ordering::Greater
        //                 } else {
        //                     Ordering::Less
        //                 }
        //             })
        //             .unwrap();

        //         return Some((
        //             self.sorted_heap_init_entries[idx].start_offset,
        //             self.sorted_heap_init_entries[idx].end_offset,
        //             self.sorted_heap_init_entries[idx].value,
        //         ));
        //     }
        //     LocationType::Global => {
        //         for entry in self.sorted_global_init_entries.iter() {
        //             if entry.start_offset == offset {
        //                 return Some((offset, offset, entry.value));
        //             }
        //         }
        //     }
        //     LocationType::Stack => unreachable!(),
        // }

        // None
    }
}
