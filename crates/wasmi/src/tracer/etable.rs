use serde::{Deserialize, Serialize};
use wasmi_core::UntypedValue;

use crate::{
    engine::stack::ValueStackPtr,
    mtable::{memory_event_of_step, MTable},
};

use super::imtable::{MemoryReadSize, MemoryStoreSize, VarType};

pub fn from_untyped_value_to_u64_with_typ(vtype: VarType, val: UntypedValue) -> u64 {
    match vtype {
        VarType::I32 => val.to_bits(),
        VarType::I64 => val.to_bits(),
        VarType::F32 => val.to_bits(),
        VarType::F64 => val.to_bits(),
        _ => panic!("Unsupported type"),
    }
}

#[derive(Deserialize, Serialize)]
pub enum RunInstructionTracePre {
    BrIfEqz {
        value: i32,
    },
    BrIfNez {
        value: i32,
    },
    BrTable {
        index: i32,
    },
    Return {
        drop: u32,
        keep_values: Vec<u64>,
    },
    CallInternal {
        args: Vec<UntypedValue>,
    },
    CallIndirect {
        idx: u32,
    },
    SetLocal {
        depth: usize,
        value: UntypedValue,
    },
    SetGlobal {
        idx: u32,
        value: UntypedValue,
    },
    Load {
        offset: u32,
        raw_address: u32,
        effective_address: Option<usize>, // use option in case of memory out of bound
        vtype: VarType,
        load_size: MemoryReadSize,
    },
    Store {
        offset: u32,
        raw_address: u32,
        effective_address: Option<usize>,
        value: u64,
        vtype: VarType,
        store_size: MemoryStoreSize,
        pre_block_value1: Option<u64>,
        pre_block_value2: Option<u64>,
    },
    GrowMemory(i32),

    I32BinOp {
        left: i32,
        right: i32,
    },
    I32BinShiftOp {
        left: u64,
        right: u64,
    },

    I64BinOp {
        left: i64,
        right: i64,
    },

    I32Single(i32),
    I32Comp {
        left: i32,
        right: i32,
    },
    I64Single(i64),
    I64Comp {
        left: i64,
        right: i64,
    },

    I32WrapI64 {
        value: i64,
    },
    I64ExtendI32 {
        value: i32,
        sign: bool,
    },
    I32SignExtendI8 {
        value: i32,
    },
    I32SignExtendI16 {
        value: i32,
    },
    I64SignExtendI8 {
        value: i64,
    },
    I64SignExtendI16 {
        value: i64,
    },
    I64SignExtendI32 {
        value: i64,
    },

    I32TruncF32 {
        value: f32,
        sign: bool,
    },

    I32TruncF64 {
        value: f64,
        sign: bool,
    },

    I64TruncF32 {
        value: f32,
        sign: bool,
    },

    I64TruncF64 {
        value: f64,
        sign: bool,
    },

    F32ConvertI32 {
        value: i32,
        sign: bool,
    },
    F32ConvertI64 {
        value: i64,
        sign: bool,
    },
    F64ConvertI32 {
        value: i32,
        sign: bool,
    },
    F64ConvertI64 {
        value: i64,
        sign: bool,
    },
    I32ReinterpretF32 {
        value: f32,
    },
    I64ReinterpretF64 {
        value: f64,
    },
    F32ReinterpretI32 {
        value: i32,
    },
    F64ReinterpretI64 {
        value: i64,
    },
    F32DemoteF64 {
        value: f64,
    },
    F64PromoteF32 {
        value: f32,
    },
    UnaryOp {
        operand: u64,
        vtype: VarType,
    },
    Drop,
    Select {
        val1: u64,
        val2: u64,
        cond: u64,
    },

    F32Comp {
        left: f32,
        right: f32,
    },

    F64Comp {
        left: f64,
        right: f64,
    },

    F32BinOp {
        left: f32,
        right: f32,
    },

    F64BinOp {
        left: f64,
        right: f64,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum UnaryOp {
    Ctz,
    Clz,
    Popcnt,
    Abs,
    Neg,
    Ceil,
    Floor,
    Trunc,
    Nearest,
    Sqrt,
}

impl UnaryOp {
    fn encode(self) -> u8 {
        match self {
            UnaryOp::Ctz => 0,
            UnaryOp::Clz => 1,
            UnaryOp::Popcnt => 2,
            UnaryOp::Abs => 3,
            UnaryOp::Neg => 4,
            UnaryOp::Ceil => 5,
            UnaryOp::Floor => 6,
            UnaryOp::Trunc => 7,
            UnaryOp::Nearest => 8,
            UnaryOp::Sqrt => 9,
        }
    }

    fn decode(byte: u8) -> Self {
        match byte {
            0 => UnaryOp::Ctz,
            1 => UnaryOp::Clz,
            2 => UnaryOp::Popcnt,
            3 => UnaryOp::Abs,
            4 => UnaryOp::Neg,
            5 => UnaryOp::Ceil,
            6 => UnaryOp::Floor,
            7 => UnaryOp::Trunc,
            8 => UnaryOp::Nearest,
            9 => UnaryOp::Sqrt,
            _ => panic!("invalid type"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Copysign,
    UnsignedDiv,
    UnsignedRem,
    SignedDiv,
    SignedRem,
}

impl BinOp {
    fn encode(self) -> u8 {
        match self {
            BinOp::Add => 0,
            BinOp::Sub => 1,
            BinOp::Mul => 2,
            BinOp::Div => 3,
            BinOp::Min => 4,
            BinOp::Max => 5,
            BinOp::Copysign => 6,
            BinOp::UnsignedDiv => 7,
            BinOp::UnsignedRem => 8,
            BinOp::SignedDiv => 9,
            BinOp::SignedRem => 10,
        }
    }

    fn decode(byte: u8) -> Self {
        match byte {
            0 => BinOp::Add,
            1 => BinOp::Sub,
            2 => BinOp::Mul,
            3 => BinOp::Div,
            4 => BinOp::Min,
            5 => BinOp::Max,
            6 => BinOp::Copysign,
            7 => BinOp::UnsignedDiv,
            8 => BinOp::UnsignedRem,
            9 => BinOp::SignedDiv,
            10 => BinOp::SignedRem,
            _ => panic!("invalid type"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum ShiftOp {
    Shl,
    UnsignedShr,
    SignedShr,
    Rotl,
    Rotr,
}

impl ShiftOp {
    fn encode(self) -> u8 {
        match self {
            ShiftOp::Shl => 0,
            ShiftOp::UnsignedShr => 1,
            ShiftOp::SignedShr => 2,
            ShiftOp::Rotl => 3,
            ShiftOp::Rotr => 4,
        }
    }

    fn decode(byte: u8) -> Self {
        match byte {
            0 => ShiftOp::Shl,
            1 => ShiftOp::UnsignedShr,
            2 => ShiftOp::SignedShr,
            3 => ShiftOp::Rotl,
            4 => ShiftOp::Rotr,
            _ => panic!("invalid type"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum BitOp {
    And = 0,
    Or = 1,
    Xor = 2,
}

impl BitOp {
    fn encode(self) -> u8 {
        match self {
            BitOp::And => 0,
            BitOp::Or => 1,
            BitOp::Xor => 2,
        }
    }

    fn decode(byte: u8) -> Self {
        match byte {
            0 => BitOp::And,
            1 => BitOp::Or,
            2 => BitOp::Xor,
            _ => panic!("invalid type"),
        }
    }
}

impl BitOp {
    pub fn eval(&self, left: u64, right: u64) -> u64 {
        match self {
            BitOp::And => left & right,
            BitOp::Or => left | right,
            BitOp::Xor => left ^ right,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum RelOp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    SignedGt,
    UnsignedGt,
    SignedGe,
    UnsignedGe,
    SignedLt,
    UnsignedLt,
    SignedLe,
    UnsignedLe,
}

impl RelOp {
    fn encode(self) -> u8 {
        match self {
            RelOp::Eq => 0u8,
            RelOp::Ne => 1u8,
            RelOp::Lt => 2u8,
            RelOp::Gt => 3u8,
            RelOp::Le => 4u8,
            RelOp::Ge => 5u8,
            RelOp::SignedGt => 6u8,
            RelOp::UnsignedGt => 7u8,
            RelOp::SignedGe => 8u8,
            RelOp::UnsignedGe => 9u8,
            RelOp::SignedLt => 10u8,
            RelOp::UnsignedLt => 11u8,
            RelOp::SignedLe => 12u8,
            RelOp::UnsignedLe => 13u8,
        }
    }

    fn decode(byte: u8) -> Self {
        match byte {
            0 => RelOp::Eq,
            1 => RelOp::Ne,
            2 => RelOp::Lt,
            3 => RelOp::Gt,
            4 => RelOp::Le,
            5 => RelOp::Ge,
            6 => RelOp::SignedGt,
            7 => RelOp::UnsignedGt,
            8 => RelOp::SignedGe,
            9 => RelOp::UnsignedGe,
            10 => RelOp::SignedLt,
            11 => RelOp::UnsignedLt,
            12 => RelOp::SignedLe,
            13 => RelOp::UnsignedLe,
            _ => panic!("invalid type"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum TestOp {
    Eqz,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum ConversionOp {
    I32WrapI64,
    I64ExtendI32s,
    I64ExtendI32u,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I32TruncSF32,
    I32TruncUF32,
    I32TruncSF64,
    I32TruncUF64,
    I64TruncSF32,
    I64TruncUF32,
    I64TruncSF64,
    I64TruncUF64,
    F32ConvertSI32,
    F32ConvertUI32,
    F32ConvertSI64,
    F32ConvertUI64,
    F64ConvertSI32,
    F64ConvertUI32,
    F64ConvertSI64,
    F64ConvertUI64,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    F32DemoteF64,
    F64PromoteF32,
}

trait ToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

impl ToBytes for i32 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().try_into().unwrap()
    }
}

impl ToBytes for usize {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().try_into().unwrap()
    }
}

impl ToBytes for u32 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().try_into().unwrap()
    }
}

impl ToBytes for u64 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().try_into().unwrap()
    }
}

impl ToBytes for i64 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().try_into().unwrap()
    }
}

impl ToBytes for f64 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().try_into().unwrap()
    }
}

impl ToBytes for f32 {
    fn to_bytes(self) -> Vec<u8> {
        self.to_be_bytes().try_into().unwrap()
    }
}

impl<T: ToBytes> ToBytes for Vec<T> {
    fn to_bytes(self) -> Vec<u8> {
        let mut len: Vec<u8> = (self.len() as u32).to_be_bytes().try_into().unwrap();

        for elem in self {
            len.extend(elem.to_bytes());
        }

        len
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum StepInfo {
    Br {
        offset: i32,
    },
    BrIfEqz {
        condition: i32,
        offset: i32,
    },
    BrIfNez {
        condition: i32,
        offset: i32,
    },
    BrAdjust {
        offset: i32,
    },
    BrTable {
        index: i32,
        offset: usize,
    },
    Return {
        drop: u32,
        keep_values: Vec<u64>,
    },
    Drop,
    Select {
        val1: u64,
        val2: u64,
        cond: u64,
        result: u64,
    },
    CallInternal {
        args: Vec<UntypedValue>,
    },
    CallIndirect {
        func_index: u32,
    },
    // CallHost {
    //     plugin: HostPlugin,
    //     host_function_idx: usize,
    //     function_name: String,
    //     signature: Signature,
    //     args: Vec<u64>,
    //     ret_val: Option<u64>,
    //     op_index_in_plugin: usize,
    // },
    // ExternalHostCall {
    //     op: usize,
    //     value: Option<u64>,
    //     sig: ExternalHostCallSignature,
    // },
    LocalGet {
        depth: usize,
        value: u64,
    },
    SetLocal {
        depth: usize,
        value: u64,
    },
    TeeLocal {
        depth: usize,
        value: u64,
    },
    GetGlobal {
        idx: u32,
        value: u64,
    },
    SetGlobal {
        idx: u32,
        value: u64,
    },
    Load {
        vtype: VarType,
        load_size: MemoryReadSize,
        offset: u32,
        raw_address: u32,
        effective_address: usize,
        value: u64,
        block_value1: u64,
        block_value2: u64,
    },
    Store {
        vtype: VarType,
        store_size: MemoryStoreSize,
        offset: u32,
        raw_address: u32,
        effective_address: usize,
        pre_block_value1: u64,
        updated_block_value1: u64,
        pre_block_value2: u64,
        updated_block_value2: u64,
        value: u64,
    },
    MemorySize,
    MemoryGrow {
        grow_size: i32,
        result: i32,
    },

    I32Const {
        value: i32,
    },
    Const32 {
        value: u32,
    },
    ConstRef {
        value: u64,
    },
    I64Const {
        value: i64,
    },

    I32BinOp {
        class: BinOp,
        left: i32,
        right: i32,
        value: i32,
    },
    I32BinShiftOp {
        class: ShiftOp,
        left: i32,
        right: i32,
        value: i32,
    },
    I32BinBitOp {
        class: BitOp,
        left: i32,
        right: i32,
        value: i32,
    },

    I64BinOp {
        class: BinOp,
        left: i64,
        right: i64,
        value: i64,
    },
    I64BinShiftOp {
        class: ShiftOp,
        left: i64,
        right: i64,
        value: i64,
    },
    I64BinBitOp {
        class: BitOp,
        left: i64,
        right: i64,
        value: i64,
    },

    UnaryOp {
        class: UnaryOp,
        vtype: VarType,
        operand: u64,
        result: u64,
    },
    CompZ {
        vtype: VarType,
        value: u64,
        result: i32,
    },
    I32Comp {
        class: RelOp,
        left: i32,
        right: i32,
        value: bool,
    },
    I64Comp {
        class: RelOp,
        left: i64,
        right: i64,
        value: bool,
    },
    I32WrapI64 {
        value: i64,
        result: i32,
    },
    I64ExtendI32 {
        value: i32,
        result: i64,
        sign: bool,
    },
    I32SignExtendI8 {
        value: i32,
        result: i32,
    },
    I32SignExtendI16 {
        value: i32,
        result: i32,
    },
    I64SignExtendI8 {
        value: i64,
        result: i64,
    },
    I64SignExtendI16 {
        value: i64,
        result: i64,
    },
    I64SignExtendI32 {
        value: i64,
        result: i64,
    },
    I32TruncF32 {
        value: f32,
        result: i32,
        sign: bool,
    },
    I32TruncF64 {
        value: f64,
        result: i32,
        sign: bool,
    },
    I64TruncF32 {
        value: f32,
        result: i64,
        sign: bool,
    },
    I64TruncF64 {
        value: f64,
        result: i64,
        sign: bool,
    },

    F32ConvertI32 {
        value: i32,
        result: f32,
        sign: bool,
    },

    F32ConvertI64 {
        value: i64,
        result: f32,
        sign: bool,
    },

    F64ConvertI32 {
        value: i32,
        result: f64,
        sign: bool,
    },

    F64ConvertI64 {
        value: i64,
        result: f64,
        sign: bool,
    },

    I32ReinterpretF32 {
        value: f32,
        result: i32,
    },
    I64ReinterpretF64 {
        value: f64,
        result: i64,
    },
    F32ReinterpretI32 {
        value: i32,
        result: f32,
    },

    F64ReinterpretI64 {
        value: i64,
        result: f64,
    },

    F32DemoteF64 {
        value: f64,
        result: f32,
    },
    F64PromoteF32 {
        value: f32,
        result: f64,
    },

    F32Const {
        value: f32,
    },

    F64Const {
        value: f64,
    },
    F32Comp {
        class: RelOp,
        left: f32,
        right: f32,
        value: bool,
    },
    F64Comp {
        class: RelOp,
        left: f64,
        right: f64,
        value: bool,
    },
    F32BinOp {
        class: BinOp,
        left: f32,
        right: f32,
        value: f32,
    },

    F64BinOp {
        class: BinOp,
        left: f64,
        right: f64,
        value: f64,
    },
}

impl StepInfo {
    fn encode(self) -> Vec<u8> {
        match self {
            StepInfo::Br { offset } => {
                let mut bytes = vec![0u8; 1];
                bytes.extend(offset.to_bytes());

                bytes
            }
            StepInfo::BrIfEqz { condition, offset } => {
                let mut bytes = vec![1u8; 1];
                bytes.extend(condition.to_bytes());
                bytes.extend(offset.to_bytes());

                bytes
            }
            StepInfo::BrIfNez { condition, offset } => {
                let mut bytes = vec![2u8; 1];
                bytes.extend(condition.to_bytes());
                bytes.extend(offset.to_bytes());

                assert_eq!(bytes.len(), 9);

                bytes
            }
            StepInfo::BrAdjust { offset } => {
                let mut bytes = vec![3u8; 1];
                bytes.extend(offset.to_bytes());

                bytes
            }
            StepInfo::BrTable { index, offset } => {
                let mut bytes = vec![4u8; 1];
                bytes.extend(index.to_bytes());
                bytes.extend(offset.to_bytes());

                bytes
            }
            StepInfo::Return { drop, keep_values } => {
                let mut bytes = vec![5u8; 1];
                bytes.extend(drop.to_bytes());
                bytes.extend(keep_values.to_bytes());

                bytes
            }
            StepInfo::Drop => {
                vec![6u8; 1]
            }
            StepInfo::Select {
                val1,
                val2,
                cond,
                result,
            } => {
                let mut bytes = vec![7u8; 1];
                bytes.extend(val1.to_bytes());
                bytes.extend(val2.to_bytes());
                bytes.extend(cond.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::CallInternal { args } => {
                let mut bytes = vec![8u8; 1];
                bytes.extend((args.len() as u32).to_bytes());

                args.into_iter().fold(bytes, |mut acc, arg| {
                    let value: Vec<u8> = arg.to_bits().to_be_bytes().try_into().unwrap();
                    acc.extend(value);
                    acc
                })
            }
            StepInfo::CallIndirect { func_index } => {
                let mut bytes = vec![9u8; 1];
                bytes.extend(func_index.to_bytes());

                bytes
            }
            StepInfo::LocalGet { depth, value } => {
                let mut bytes = vec![10u8; 1];
                bytes.extend(depth.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::SetLocal { depth, value } => {
                let mut bytes = vec![11u8; 1];
                bytes.extend(depth.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::TeeLocal { depth, value } => {
                let mut bytes = vec![12u8; 1];
                bytes.extend(depth.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::GetGlobal { idx, value } => {
                let mut bytes = vec![13u8; 1];
                bytes.extend(idx.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::SetGlobal { idx, value } => {
                let mut bytes = vec![14u8; 1];
                bytes.extend(idx.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::Load {
                vtype,
                load_size,
                offset,
                raw_address,
                effective_address,
                value,
                block_value1,
                block_value2,
            } => {
                let mut bytes = vec![15u8, vtype as u8, load_size.encode()];

                bytes.extend(offset.to_bytes());
                bytes.extend(raw_address.to_bytes());
                bytes.extend(effective_address.to_bytes());
                bytes.extend(value.to_bytes());
                bytes.extend(block_value1.to_bytes());
                bytes.extend(block_value2.to_bytes());

                bytes
            }
            StepInfo::Store {
                vtype,
                store_size,
                offset,
                raw_address,
                effective_address,
                pre_block_value1,
                updated_block_value1,
                pre_block_value2,
                updated_block_value2,
                value,
            } => {
                let mut bytes = vec![16u8, vtype as u8, store_size.encode()];

                bytes.extend(offset.to_bytes());
                bytes.extend(raw_address.to_bytes());
                bytes.extend(effective_address.to_bytes());
                bytes.extend(pre_block_value1.to_bytes());
                bytes.extend(updated_block_value1.to_bytes());
                bytes.extend(pre_block_value2.to_bytes());
                bytes.extend(updated_block_value2.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::MemorySize => vec![17u8; 1],
            StepInfo::MemoryGrow { grow_size, result } => {
                let mut bytes = vec![18u8; 1];
                bytes.extend(grow_size.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I32Const { value } => {
                let mut bytes = vec![19u8; 1];
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::Const32 { value } => {
                let mut bytes = vec![20u8; 1];
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::ConstRef { value } => {
                let mut bytes = vec![21u8; 1];
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::I64Const { value } => {
                let mut bytes = vec![22u8; 1];
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::I32BinOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![23u8, class.encode()];

                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::I32BinShiftOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![24u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::I32BinBitOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![25u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::I64BinOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![26u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::I64BinShiftOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![27u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::I64BinBitOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![28u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::UnaryOp {
                class,
                vtype,
                operand,
                result,
            } => {
                let mut bytes = vec![29u8, class.encode(), vtype as u8];
                bytes.extend(operand.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::CompZ {
                vtype,
                value,
                result,
            } => {
                let mut bytes = vec![30u8, vtype as u8];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                match vtype {
                    VarType::I64 => {}
                    _ => {
                        assert_ne!(bytes[1], 0);
                    }
                }

                bytes
            }
            StepInfo::I32Comp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![31u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.push(match value {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::I64Comp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![32u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.push(match value {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::I32WrapI64 { value, result } => {
                let mut bytes = vec![33u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I64ExtendI32 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![34u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::I32SignExtendI8 { value, result } => {
                let mut bytes = vec![35u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I32SignExtendI16 { value, result } => {
                let mut bytes = vec![36u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I64SignExtendI8 { value, result } => {
                let mut bytes = vec![37u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I64SignExtendI16 { value, result } => {
                let mut bytes = vec![38u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I64SignExtendI32 { value, result } => {
                let mut bytes = vec![39u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I32TruncF32 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![40u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::I32TruncF64 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![41u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::I64TruncF32 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![42u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::I64TruncF64 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![43u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::F32ConvertI32 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![44u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::F32ConvertI64 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![45u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::F64ConvertI32 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![46u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::F64ConvertI64 {
                value,
                result,
                sign,
            } => {
                let mut bytes = vec![47u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());
                bytes.push(match sign {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::I32ReinterpretF32 { value, result } => {
                let mut bytes = vec![48u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::I64ReinterpretF64 { value, result } => {
                let mut bytes = vec![49u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::F32ReinterpretI32 { value, result } => {
                let mut bytes = vec![50u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::F64ReinterpretI64 { value, result } => {
                let mut bytes = vec![51u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::F32DemoteF64 { value, result } => {
                let mut bytes = vec![52u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::F64PromoteF32 { value, result } => {
                let mut bytes = vec![53u8; 1];
                bytes.extend(value.to_bytes());
                bytes.extend(result.to_bytes());

                bytes
            }
            StepInfo::F32Const { value } => {
                let mut bytes = vec![54u8; 1];
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::F64Const { value } => {
                let mut bytes = vec![55u8; 1];
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::F32Comp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![56u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.push(match value {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::F64Comp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![57u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.push(match value {
                    true => 1,
                    false => 0,
                });

                bytes
            }
            StepInfo::F32BinOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![58u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
            StepInfo::F64BinOp {
                class,
                left,
                right,
                value,
            } => {
                let mut bytes = vec![59u8, class.encode()];
                bytes.extend(left.to_bytes());
                bytes.extend(right.to_bytes());
                bytes.extend(value.to_bytes());

                bytes
            }
        }
    }

    fn decode(bytes: Vec<u8>) -> Self {
        match bytes[0] {
            0 => Self::Br {
                offset: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..]).unwrap()),
            },

            1 => Self::BrIfEqz {
                condition: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                offset: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
            },
            2 => {
                let condition =
                    i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap());
                let offset =
                    i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap());
                Self::BrIfNez { condition, offset }
            }
            3 => {
                let offset =
                    i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap());
                Self::BrAdjust { offset }
            }
            4 => {
                let index = i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap());
                let offset =
                    usize::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[5..9]).unwrap());
                Self::BrTable { index, offset }
            }
            5 => {
                let drop = u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap());
                let num_values =
                    u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap());
                let mut values = Vec::new();

                for i in 0..num_values {
                    let value = u64::from_be_bytes(
                        TryInto::<[u8; 8]>::try_into(
                            &bytes[9 + i as usize * 8..9 + i as usize * 8 + 8],
                        )
                        .unwrap(),
                    );
                    values.push(value);
                }

                Self::Return {
                    drop,
                    keep_values: values,
                }
            }
            6 => Self::Drop,
            7 => Self::Select {
                val1: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                val2: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
                cond: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[17..25]).unwrap()),
                result: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[25..33]).unwrap()),
            },
            8 => {
                let num_args =
                    u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap());
                let mut args = Vec::new();

                for i in 0..num_args {
                    let arg = u64::from_be_bytes(
                        TryInto::<[u8; 8]>::try_into(
                            &bytes[5 + i as usize * 8..5 + i as usize * 8 + 8],
                        )
                        .unwrap(),
                    );
                    args.push(UntypedValue::from(arg));
                }

                Self::CallInternal { args }
            }
            9 => Self::CallIndirect {
                func_index: u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..9]).unwrap()),
            },
            10 => Self::LocalGet {
                depth: usize::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            11 => Self::SetLocal {
                depth: usize::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            12 => Self::TeeLocal {
                depth: usize::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            13 => Self::GetGlobal {
                idx: u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[5..13]).unwrap()),
            },
            14 => Self::SetGlobal {
                idx: u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[5..13]).unwrap()),
            },
            15 => Self::Load {
                vtype: VarType::decode(bytes[1]),
                load_size: MemoryReadSize::decode(bytes[2]),
                offset: u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[3..7]).unwrap()),
                raw_address: u32::from_be_bytes(
                    TryInto::<[u8; 4]>::try_into(&bytes[7..11]).unwrap(),
                ),
                effective_address: usize::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[11..19]).unwrap(),
                ),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[19..27]).unwrap()),
                block_value1: u64::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[27..35]).unwrap(),
                ),
                block_value2: u64::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[35..43]).unwrap(),
                ),
            },
            16 => Self::Store {
                vtype: VarType::decode(bytes[1]),
                store_size: MemoryStoreSize::decode(bytes[2]),
                offset: u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[3..7]).unwrap()),
                raw_address: u32::from_be_bytes(
                    TryInto::<[u8; 4]>::try_into(&bytes[7..11]).unwrap(),
                ),
                effective_address: usize::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[11..19]).unwrap(),
                ),
                pre_block_value1: u64::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[19..27]).unwrap(),
                ),
                updated_block_value1: u64::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[27..35]).unwrap(),
                ),
                pre_block_value2: u64::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[35..43]).unwrap(),
                ),
                updated_block_value2: u64::from_be_bytes(
                    TryInto::<[u8; 8]>::try_into(&bytes[43..51]).unwrap(),
                ),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[51..59]).unwrap()),
            },
            17 => Self::MemorySize,
            18 => Self::MemoryGrow {
                grow_size: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
            },
            19 => Self::I32Const {
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
            },
            20 => Self::Const32 {
                value: u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
            },
            21 => Self::ConstRef {
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
            },
            22 => Self::I64Const {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
            },
            23 => Self::I32BinOp {
                class: BinOp::decode(bytes[1]),
                left: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[2..6]).unwrap()),
                right: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[6..10]).unwrap()),
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[10..14]).unwrap()),
            },
            24 => Self::I32BinShiftOp {
                class: ShiftOp::decode(bytes[1]),
                left: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[2..6]).unwrap()),
                right: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[6..10]).unwrap()),
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[10..14]).unwrap()),
            },
            25 => Self::I32BinBitOp {
                class: BitOp::decode(bytes[1]),
                left: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[2..6]).unwrap()),
                right: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[6..10]).unwrap()),
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[10..14]).unwrap()),
            },
            26 => Self::I64BinOp {
                class: BinOp::decode(bytes[1]),
                left: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[2..10]).unwrap()),
                right: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[10..18]).unwrap()),
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[18..26]).unwrap()),
            },
            27 => Self::I64BinShiftOp {
                class: ShiftOp::decode(bytes[1]),
                left: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[2..10]).unwrap()),
                right: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[10..18]).unwrap()),
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[18..26]).unwrap()),
            },
            28 => Self::I64BinBitOp {
                class: BitOp::decode(bytes[1]),
                left: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[2..10]).unwrap()),
                right: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[10..18]).unwrap()),
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[18..26]).unwrap()),
            },
            29 => Self::UnaryOp {
                class: UnaryOp::decode(bytes[1]),
                vtype: VarType::decode(bytes[2]),
                operand: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[3..11]).unwrap()),
                result: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[11..19]).unwrap()),
            },
            30 => Self::CompZ {
                vtype: VarType::decode(bytes[1]),
                value: u64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[2..10]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[10..14]).unwrap()),
            },
            31 => Self::I32Comp {
                class: RelOp::decode(bytes[1]),
                left: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[2..6]).unwrap()),
                right: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[6..10]).unwrap()),
                value: match bytes[10] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            32 => Self::I64Comp {
                class: RelOp::decode(bytes[1]),
                left: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[2..10]).unwrap()),
                right: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[10..18]).unwrap()),
                value: match bytes[17] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            33 => Self::I32WrapI64 {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[9..13]).unwrap()),
            },
            34 => Self::I64ExtendI32 {
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[5..13]).unwrap()),
                sign: match bytes[13] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            35 => Self::I32SignExtendI8 {
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
            },
            36 => Self::I32SignExtendI16 {
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
            },
            37 => Self::I64SignExtendI8 {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            38 => Self::I64SignExtendI16 {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            39 => Self::I64SignExtendI32 {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            40 => Self::I32TruncF32 {
                value: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
                sign: match bytes[9] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            41 => Self::I32TruncF64 {
                value: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[9..13]).unwrap()),
                sign: match bytes[9] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            42 => Self::I64TruncF32 {
                value: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[5..13]).unwrap()),
                sign: match bytes[13] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            43 => Self::I64TruncF64 {
                value: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
                sign: match bytes[17] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            44 => Self::F32ConvertI32 {
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
                sign: match bytes[9] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            45 => Self::F32ConvertI64 {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[9..13]).unwrap()),
                sign: match bytes[13] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            46 => Self::F64ConvertI32 {
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[5..13]).unwrap()),
                sign: match bytes[13] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            47 => Self::F64ConvertI64 {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
                sign: match bytes[17] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            48 => Self::I32ReinterpretF32 {
                value: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
            },
            49 => Self::I64ReinterpretF64 {
                value: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            50 => Self::F32ReinterpretI32 {
                value: i32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[5..9]).unwrap()),
            },
            51 => Self::F64ReinterpretI64 {
                value: i64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[9..17]).unwrap()),
            },
            52 => Self::F32DemoteF64 {
                value: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
                result: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[9..13]).unwrap()),
            },
            53 => Self::F64PromoteF32 {
                value: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
                result: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[5..13]).unwrap()),
            },
            54 => Self::F32Const {
                value: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[1..5]).unwrap()),
            },
            55 => Self::F64Const {
                value: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[1..9]).unwrap()),
            },
            56 => Self::F32Comp {
                class: RelOp::decode(bytes[1]),
                left: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[2..6]).unwrap()),
                right: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[6..10]).unwrap()),
                value: match bytes[9] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            57 => Self::F64Comp {
                class: RelOp::decode(bytes[1]),
                left: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[2..10]).unwrap()),
                right: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[10..18]).unwrap()),
                value: match bytes[17] {
                    0 => false,
                    1 => true,
                    _ => panic!("invalid type"),
                },
            },
            58 => Self::F32BinOp {
                class: BinOp::decode(bytes[1]),
                left: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[2..6]).unwrap()),
                right: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[6..10]).unwrap()),
                value: f32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&bytes[10..14]).unwrap()),
            },
            59 => Self::F64BinOp {
                class: BinOp::decode(bytes[1]),
                left: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[2..10]).unwrap()),
                right: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[10..18]).unwrap()),
                value: f64::from_be_bytes(TryInto::<[u8; 8]>::try_into(&bytes[18..26]).unwrap()),
            },
            _ => panic!("invalid type"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ETEntry {
    pub eid: u32,
    pub allocated_memory_pages: usize,
    pub step_info: StepInfo,
    pub sp: ValueStackPtr,
}

#[derive(Debug, Default)]
pub struct ETable(Vec<ETEntry>);

#[derive(Deserialize, Serialize, Clone)]
pub struct Shard {
    eid: u32,
    allocated_memory_pages: usize,
    stack_pointer: ValueStackPtr,
    pub steps: Vec<St>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum St {
    M(usize),
    I(Vec<u8>),
    D(isize),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Step {
    sd: isize,
    si: StepInfo,
}

impl ETable {
    pub fn into_shards(&self, num_shards: usize) -> Vec<Shard> {
        let chunk_size = self.0.len() / num_shards;
        let remainder = self.0.len() % num_shards;
        let mut chunks = Vec::new();

        let mut start_index = 0;
        for i in 0..num_shards {
            let end_index = start_index + chunk_size + if i < remainder { 1 } else { 0 };
            let chunk = &self.0[start_index..end_index];
            chunks.push(chunk.to_vec());
            start_index = end_index;
        }

        chunks
            .into_iter()
            .map(|chunk| {
                let mut stack_pointer = chunk[0].sp;
                let num_memory_pages = chunk[0].allocated_memory_pages;
                let eid = chunk[0].eid;

                let mut shard = Shard {
                    eid,
                    allocated_memory_pages: num_memory_pages,
                    stack_pointer,
                    steps: Vec::new(),
                };

                shard.steps = chunk
                    .iter()
                    .map(|step| {
                        let sp_delta = step.sp.offset_from(stack_pointer);
                        stack_pointer = step.sp;

                        let mut v = Vec::new();

                        if num_memory_pages != step.allocated_memory_pages {
                            v.push(St::M(step.allocated_memory_pages));
                        }

                        if sp_delta != 0 {
                            v.push(St::D(sp_delta));
                        }

                        v.push(St::I(step.step_info.clone().encode()));
                        v
                    })
                    .flatten()
                    .collect::<Vec<_>>();

                shard
            })
            .collect()
    }

    pub fn from_shards(shards: Vec<Shard>) -> Self {
        assert!(!shards.is_empty());

        let mut entries: Vec<ETEntry> = Vec::new();

        for shard in shards.into_iter() {
            let mut allocated_memory_pages = shard.allocated_memory_pages;
            let mut stack_pointer = shard.stack_pointer;
            let eid = shard.eid;
            let mut index = 0;

            let shard_entries = shard
                .steps
                .into_iter()
                .filter_map(|step| match step {
                    St::I(step_info) => Some(ETEntry {
                        eid: {
                            let prev = index;
                            index += 1;
                            eid + prev
                        },
                        allocated_memory_pages,
                        sp: stack_pointer,
                        step_info: StepInfo::decode(step_info),
                    }),
                    St::M(mem) => {
                        allocated_memory_pages = mem;
                        None
                    }
                    St::D(delta) => {
                        if delta < 0 {
                            stack_pointer = stack_pointer.into_sub(delta.abs() as usize);
                        } else if delta > 0 {
                            stack_pointer = stack_pointer.into_add(delta as usize);
                        }

                        None
                    }
                })
                .collect::<Vec<_>>();

            entries.extend(shard_entries);
        }

        Self(entries)
    }

    pub fn from_shard(shard: Shard) -> Self {
        Self::from_shards(vec![shard])
    }

    pub fn new(entries: Vec<ETEntry>) -> Self {
        ETable(entries)
    }

    pub fn entries(&self) -> &Vec<ETEntry> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut Vec<ETEntry> {
        &mut self.0
    }
    pub fn push(&mut self, allocated_memory_pages: u32, step_info: StepInfo, sp: ValueStackPtr) {
        let et_entry = ETEntry {
            eid: (self.entries().len() + 1).try_into().unwrap(),
            allocated_memory_pages: allocated_memory_pages as usize,
            step_info,
            sp,
        };

        self.entries_mut().push(et_entry.clone());
    }

    pub fn get_mtable(&self) -> MTable {
        let mentries = self
            .entries()
            .iter()
            .map(|eentry| memory_event_of_step(eentry, &mut 1))
            .collect::<Vec<Vec<_>>>()
            .concat();

        MTable::new(mentries)
    }
}
