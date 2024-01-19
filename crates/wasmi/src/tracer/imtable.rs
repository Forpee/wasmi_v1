use wasmi_core::ValueType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LocationType {
    Stack = 1,
    Heap = 2,
    Global = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VarType {
    I64 = 0,
    I32 = 1,
    F32 = 2,
    F64 = 3,
    FuncRef = 4,
    ExternRef = 5,
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
}
