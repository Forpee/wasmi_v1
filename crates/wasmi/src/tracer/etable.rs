pub enum RunInstructionTracePre {
    BrIfEqz { value: i32 },
    BrIfNez { value: i32 },
    BrTable { index: i32 },

    // Call {
    //     args: Vec<ValueInternal>,
    // },
    // CallIndirect {
    //     table_idx: u32,
    //     type_idx: u32,
    //     offset: u32,
    // },

    // SetLocal {
    //     depth: u32,
    //     value: ValueInternal,
    //     vtype: ValueType,
    // },
    // SetGlobal {
    //     idx: u32,
    //     value: ValueInternal,
    // },

    // Load {
    //     offset: u32,
    //     raw_address: u32,
    //     effective_address: Option<u32>, // use option in case of memory out of bound
    //     vtype: ValueType,
    //     load_size: MemoryReadSize,
    // },
    // Store {
    //     offset: u32,
    //     raw_address: u32,
    //     effective_address: Option<u32>,
    //     value: u64,
    //     vtype: ValueType,
    //     store_size: MemoryStoreSize,
    //     pre_block_value1: Option<u64>,
    //     pre_block_value2: Option<u64>,
    // },
    GrowMemory(i32),

    I32BinOp { left: i32, right: i32 },
    I32BinShiftOp { left: u64, right: u64 },

    I64BinOp { left: i64, right: i64 },

    I32Single(i32),
    I32Comp { left: i32, right: i32 },
    I64Single(i64),
    I64Comp { left: i64, right: i64 },

    I32WrapI64 { value: i64 },
    I64ExtendI32 { value: i32, sign: bool },
    I32SignExtendI8 { value: i32 },
    I32SignExtendI16 { value: i32 },
    I64SignExtendI8 { value: i64 },
    I64SignExtendI16 { value: i64 },
    I64SignExtendI32 { value: i64 },

    I32TruncF32 { value: f32, sign: bool },

    I32TruncF64 { value: f64, sign: bool },

    I64TruncF32 { value: f32, sign: bool },

    I64TruncF64 { value: f64, sign: bool },

    F32ConvertI32 { value: i32, sign: bool },
    F32ConvertI64 { value: i64, sign: bool },
    F64ConvertI32 { value: i32, sign: bool },
    F64ConvertI64 { value: i64, sign: bool },
    I32ReinterpretF32 { value: f32 },
    I64ReinterpretF64 { value: f64 },
    F32ReinterpretI32 { value: i32 },
    F64ReinterpretI64 { value: i64 },
    F32DemoteF64 { value: f32 },
    F64PromoteF32 { value: f64 },
    // UnaryOp {
    //     operand: u64,
    //     vtype: VarType,
    // },
    Drop,
    Select { val1: u64, val2: u64, cond: u64 },

    F32Comp { left: f32, right: f32 },

    F64Comp { left: f64, right: f64 },

    F32BinOp { left: f32, right: f32 },

    F64BinOp { left: f64, right: f64 },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
pub enum StepInfo {
    // Br {
    //     dst_pc: u32,
    //     drop: u32,
    //     keep: Vec<ValueType>,
    //     keep_values: Vec<u64>,
    // },
    // BrIfEqz {
    //     condition: i32,
    //     dst_pc: u32,
    //     drop: u32,
    //     keep: Vec<ValueType>,
    //     keep_values: Vec<u64>,
    // },
    // BrIfNez {
    //     condition: i32,
    //     dst_pc: u32,
    //     drop: u32,
    //     keep: Vec<ValueType>,
    //     keep_values: Vec<u64>,
    // },
    // BrTable {
    //     index: i32,
    //     dst_pc: u32,
    //     drop: u32,
    //     keep: Vec<ValueType>,
    //     keep_values: Vec<u64>,
    // },
    // Return {
    //     drop: u32,
    //     keep: Vec<ValueType>,
    //     keep_values: Vec<u64>,
    // },
    Drop,
    // Select {
    //     val1: u64,
    //     val2: u64,
    //     cond: u64,
    //     result: u64,
    //     vtype: VarType,
    // },
    Call {
        index: u32,
    },
    CallIndirect {
        table_index: u32,
        type_index: u32,
        offset: u32,
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

    // GetLocal {
    //     vtype: VarType,
    //     depth: u32,
    //     value: u64,
    // },
    // SetLocal {
    //     vtype: VarType,
    //     depth: u32,
    //     value: u64,
    // },
    // TeeLocal {
    //     vtype: VarType,
    //     depth: u32,
    //     value: u64,
    // },

    // GetGlobal {
    //     idx: u32,
    //     vtype: VarType,
    //     is_mutable: bool,
    //     value: u64,
    // },
    // SetGlobal {
    //     idx: u32,
    //     vtype: VarType,
    //     is_mutable: bool,
    //     value: u64,
    // },

    // Load {
    //     vtype: VarType,
    //     load_size: MemoryReadSize,
    //     offset: u32,
    //     raw_address: u32,
    //     effective_address: u32,
    //     value: u64,
    //     block_value1: u64,
    //     block_value2: u64,
    // },
    // Store {
    //     vtype: VarType,
    //     store_size: MemoryStoreSize,
    //     offset: u32,
    //     raw_address: u32,
    //     effective_address: u32,
    //     pre_block_value1: u64,
    //     updated_block_value1: u64,
    //     pre_block_value2: u64,
    //     updated_block_value2: u64,
    //     value: u64,
    // },
    MemorySize,
    MemoryGrow {
        grow_size: i32,
        result: i32,
    },

    I32Const {
        value: i32,
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
    // I32BinShiftOp {
    //     class: ShiftOp,
    //     left: i32,
    //     right: i32,
    //     value: i32,
    // },
    // I32BinBitOp {
    //     class: BitOp,
    //     left: i32,
    //     right: i32,
    //     value: i32,
    // },

    // I64BinOp {
    //     class: BinOp,
    //     left: i64,
    //     right: i64,
    //     value: i64,
    // },
    // I64BinShiftOp {
    //     class: ShiftOp,
    //     left: i64,
    //     right: i64,
    //     value: i64,
    // },
    // I64BinBitOp {
    //     class: BitOp,
    //     left: i64,
    //     right: i64,
    //     value: i64,
    // },

    // UnaryOp {
    //     class: UnaryOp,
    //     vtype: VarType,
    //     operand: u64,
    //     result: u64,
    // },

    // Test {
    //     vtype: VarType,
    //     value: u64,
    //     result: i32,
    // },
    // I32Comp {
    //     class: RelOp,
    //     left: i32,
    //     right: i32,
    //     value: bool,
    // },
    // I64Comp {
    //     class: RelOp,
    //     left: i64,
    //     right: i64,
    //     value: bool,
    // },
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
        value: f32,
        result: f64,
    },
    F64PromoteF32 {
        value: f64,
        result: f32,
    },

    F32Const {
        value: f32,
    },

    F64Const {
        value: f64,
    },
    // F32Comp {
    //     class: RelOp,
    //     left: f32,
    //     right: f32,
    //     value: bool,
    // },

    // F64Comp {
    //     class: RelOp,
    //     left: f64,
    //     right: f64,
    //     value: bool,
    // },

    // F32BinOp {
    //     class: BinOp,
    //     left: f32,
    //     right: f32,
    //     value: f32,
    // },

    // F64BinOp {
    //     class: BinOp,
    //     left: f64,
    //     right: f64,
    //     value: f64,
    // },
}

#[derive(Debug, Clone)]
pub struct ETEntry {
    eid: u32,
    // sp: u32,
    // allocated_memory_pages: u32,
    step_info: StepInfo,
}
#[derive(Debug, Default)]
pub struct ETable(Vec<ETEntry>);

impl ETable {
    pub fn entries(&self) -> &Vec<ETEntry> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut Vec<ETEntry> {
        &mut self.0
    }
    pub fn push(&mut self, step_info: StepInfo) {
        let et_entry = ETEntry {
            eid: (self.entries().len() + 1).try_into().unwrap(),
            // sp,
            // allocated_memory_pages,
            step_info,
        };

        self.entries_mut().push(et_entry.clone());
    }
}
