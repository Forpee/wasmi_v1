use alloc::collections::HashSet;

use crate::etable::{ETEntry, StepInfo};

use super::imtable::IMTable;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LocationType {
    Stack = 1,
    Heap = 2,
    Global = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum AccessType {
    Read = 1,
    Write = 2,
    Init = 3,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct MemoryTableEntry {
    pub eid: u32,
    /*
       Emid is sub memory op id of eid.
       E.g. an opcode gets a value from stack top and changes it.
       This event has two memory ops on the same memory address,
       So we need emid to seq the r/w op, which is an incremental value starting from 1.
    */
    pub emid: u32,
    pub addr: usize,
    pub ltype: LocationType,
    pub atype: AccessType,
    pub is_mutable: bool,
    pub value: u64,
}

#[derive(Default, Debug, Clone)]
pub struct MTable(Vec<MemoryTableEntry>);

impl MTable {
    pub fn new(entries: Vec<MemoryTableEntry>, imtable: &IMTable) -> Self {
        let mtable = MTable(entries);
        mtable
    }

    fn push_accessed_memory_initialization(&mut self, imtable: &IMTable) {
        let mut set = HashSet::<MemoryTableEntry>::default();
        self.0.iter().for_each(|entry| {
            if entry.ltype == LocationType::Heap || entry.ltype == LocationType::Global {
                let (_, _, value) = imtable.try_find().unwrap();

                set.insert(MemoryTableEntry {
                    eid: 0,
                    emid: 0,
                    addr: entry.addr,
                    ltype: entry.ltype,
                    atype: AccessType::Init,
                    is_mutable: entry.is_mutable,
                    value,
                });
            }
        });

        let mut entries = set.into_iter().collect();

        self.0.append(&mut entries);
    }

    pub fn entries(&self) -> &Vec<MemoryTableEntry> {
        &self.0
    }

    pub fn get_heap_entries(&self) -> Self {
        let entries = self
            .0
            .iter()
            .filter(|entry| entry.ltype == LocationType::Heap)
            .cloned()
            .collect();

        MTable(entries)
    }
}

// fn mem_op_from_stack_only_step(
//     sp_before_execution: u32,
//     eid: u32,
//     emid: &mut u32,
//     pop_value: &[u64],
//     push_value: &[u64],
// ) -> Vec<MemoryTableEntry> {
//     let mut mem_op = vec![];
//     let mut sp = sp_before_execution;

//     for i in 0..pop_value.len() {
//         mem_op.push(MemoryTableEntry {
//             eid,
//             emid: *emid,
//             offset: sp + 1,
//             ltype: LocationType::Stack,
//             atype: AccessType::Read,
//             is_mutable: true,
//             value: pop_value[i],
//         });
//         *emid = (*emid).checked_add(1).unwrap();
//         sp = sp + 1;
//     }

//     for i in 0..push_value.len() {
//         mem_op.push(MemoryTableEntry {
//             eid,
//             emid: *emid,
//             offset: sp,
//             ltype: LocationType::Stack,
//             atype: AccessType::Write,
//             is_mutable: true,
//             value: push_value[i],
//         });
//         sp = sp - 1;
//         *emid = (*emid).checked_add(1).unwrap();
//     }

//     mem_op
// }

pub fn memory_event_of_step(event: &ETEntry, emid: &mut u32) -> Vec<MemoryTableEntry> {
    let eid = event.eid;
    let sp_before_execution = 0;

    match &event.step_info {
        // StepInfo::Br { offset, .. } => {
        //     let mut sp = sp_before_execution + 1;
        //     let mut ops = vec![];

        //     ops
        // }
        // StepInfo::BrIfEqz {
        //     condition, offset, ..
        // } => {
        //     let mut sp = sp_before_execution + 1;

        //     let mut ops = vec![MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *condition as u32 as u64,
        //     }];

        //     sp = sp + 1;
        //     *emid = (*emid).checked_add(1).unwrap();

        //     if *condition != 0 {
        //         return ops;
        //     }

        //     ops
        // }
        // StepInfo::BrIfNez {
        //     condition, offset, ..
        // } => {
        //     let mut sp = sp_before_execution + 1;

        //     let mut ops = vec![MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *condition as u32 as u64,
        //     }];

        //     ops
        // }
        // StepInfo::BrTable { index, offset, .. } => {
        //     let mut sp = sp_before_execution + 1;

        //     let ops = vec![MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *index as u32 as u64,
        //     }];

        //     sp = sp + 1;
        //     *emid = (*emid).checked_add(1).unwrap();

        //     ops
        // }
        // StepInfo::BrAdjust { .. } => vec![],
        StepInfo::Return { ..} => vec![],
        // StepInfo::Return { drop, keep_values } => {
        //     let mut sp = sp_before_execution + 1;
        //     let mut ops = vec![];

        //     {
        //         for i in 0..keep_values.len() {
        //             ops.push(MemoryTableEntry {
        //                 eid,
        //                 emid: *emid,
        //                 offset: sp,
        //                 ltype: LocationType::Stack,
        //                 atype: AccessType::Read,
        //                 is_mutable: true,
        //                 value: keep_values[i],
        //             });

        //             sp = sp + 1;
        //             *emid = (*emid).checked_add(1).unwrap();
        //         }
        //     }

        //     sp += drop;
        //     sp -= 1;

        //     {
        //         for i in 0..keep_values.len() {
        //             ops.push(MemoryTableEntry {
        //                 eid,
        //                 emid: *emid,
        //                 offset: sp,
        //                 ltype: LocationType::Stack,
        //                 atype: AccessType::Write,
        //                 is_mutable: true,
        //                 value: keep_values[i],
        //             });

        //             sp = sp - 1;
        //             *emid = (*emid).checked_add(1).unwrap();
        //         }
        //     }

        //     ops
        // }
        StepInfo::Drop { .. } => vec![],
        // StepInfo::Select {
        //     val1,
        //     val2,
        //     cond,
        //     result,
        // } => {
        //     let mut sp = sp_before_execution + 1;
        //     let mut ops = vec![];

        //     ops.push(MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *cond,
        //     });
        //     sp = sp + 1;
        //     *emid = (*emid).checked_add(1).unwrap();

        //     ops.push(MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *val2,
        //     });
        //     sp = sp + 1;
        //     *emid = (*emid).checked_add(1).unwrap();

        //     ops.push(MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *val1,
        //     });

        //     *emid = (*emid).checked_add(1).unwrap();

        //     ops.push(MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Write,
        //         is_mutable: true,
        //         value: *result,
        //     });
        //     *emid = (*emid).checked_add(1).unwrap();

        //     ops
        // }
        // StepInfo::CallInternal { .. } => {
        //     vec![]
        // }
        // StepInfo::CallIndirect { func_index, .. } => {
        //     let stack_read = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp_before_execution + 1,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *func_index as u64,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     vec![stack_read]
        // }
        // StepInfo::CallHost {
        //     args,
        //     ret_val,
        //     signature,
        //     ..
        // } => {
        //     let mut mops = vec![];
        //     let mut sp = sp_before_execution;

        //     for (i, (ty, val)) in signature.params.iter().zip(args.iter()).enumerate() {
        //         mops.push(MemoryTableEntry {
        //             eid,
        //             emid: *emid,
        //             offset: sp_before_execution + args.len() as u32 - i as u32,
        //             ltype: LocationType::Stack,
        //             atype: AccessType::Read,
        //             vtype: (*ty).into(),
        //             is_mutable: true,
        //             value: *val,
        //         });

        //         *emid = (*emid).checked_add(1).unwrap();
        //     }

        //     sp = sp + args.len() as u32;

        //     if let Some(ty) = signature.return_type {
        //         mops.push(MemoryTableEntry {
        //             eid,
        //             emid: *emid,
        //             offset: sp,
        //             ltype: LocationType::Stack,
        //             atype: AccessType::Write,
        //             vtype: ty.into(),
        //             is_mutable: true,
        //             value: ret_val.unwrap(),
        //         });

        //         *emid = (*emid).checked_add(1).unwrap();
        //     }

        //     mops
        // }
        // StepInfo::ExternalHostCall { value, sig, .. } => match sig {
        //     ExternalHostCallSignature::Argument => {
        //         let stack_read = MemoryTableEntry {
        //             eid,
        //             emid: *emid,
        //             offset: sp_before_execution + 1,
        //             ltype: LocationType::Stack,
        //             atype: AccessType::Read,
        //             vtype: VarType::I64,
        //             is_mutable: true,
        //             value: value.unwrap(),
        //         };
        //         *emid = (*emid).checked_add(1).unwrap();

        //         vec![stack_read]
        //     }
        //     ExternalHostCallSignature::Return => {
        //         let stack_write = MemoryTableEntry {
        //             eid,
        //             emid: *emid,
        //             offset: sp_before_execution,
        //             ltype: LocationType::Stack,
        //             atype: AccessType::Write,
        //             vtype: VarType::I64,
        //             is_mutable: true,
        //             value: value.unwrap(),
        //         };
        //         *emid = (*emid).checked_add(1).unwrap();

        //         vec![stack_write]
        //     }
        // },
        // StepInfo::LocalGet { depth, value } => {
        //     let read = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp_before_execution + *depth as u32,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     let write = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp_before_execution,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Write,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();
        //     vec![read, write]
        // }
        // StepInfo::SetLocal { depth, value } => {
        //     let mut sp = sp_before_execution;

        //     let read = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp + 1,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     sp += 1;

        //     let write = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp + *depth as u32,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Write,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     vec![read, write]
        // }
        // StepInfo::TeeLocal { depth, value } => {
        //     let read = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp_before_execution + 1,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *value,
        //     };

        //     *emid = (*emid).checked_add(1).unwrap();

        //     let write = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp_before_execution + *depth as u32,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Write,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();
        //     vec![read, write]
        // }

        // StepInfo::GetGlobal { idx, value, .. } => {
        //     let global_get = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: *idx,
        //         ltype: LocationType::Global,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     let stack_write = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp_before_execution,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Write,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     vec![global_get, stack_write]
        // }
        // StepInfo::SetGlobal { idx, value } => {
        //     let stack_read = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: sp_before_execution + 1,
        //         ltype: LocationType::Stack,
        //         atype: AccessType::Read,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     let global_set = MemoryTableEntry {
        //         eid,
        //         emid: *emid,
        //         offset: *idx,
        //         ltype: LocationType::Global,
        //         atype: AccessType::Write,
        //         is_mutable: true,
        //         value: *value,
        //     };
        //     *emid = (*emid).checked_add(1).unwrap();

        //     vec![stack_read, global_set]
        // }
        StepInfo::Load {
            vtype,
            load_size,
            raw_address,
            effective_address,
            value,
            block_value1,
            block_value2,
            ..
        } => {
            let load_address_from_stack = MemoryTableEntry {
                eid,
                emid: *emid,
                addr: sp_before_execution + 1,
                ltype: LocationType::Stack,
                atype: AccessType::Read,
                is_mutable: true,
                value: *raw_address as u64,
            };
            *emid = (*emid).checked_add(1).unwrap();

            let load_value1 = MemoryTableEntry {
                eid,
                emid: *emid,
                addr: (*effective_address) / 8,
                ltype: LocationType::Heap,
                atype: AccessType::Read,
                // Load u64 from address which align with 8
                is_mutable: true,
                // The value will be used to lookup within imtable, hence block_value is given here
                value: *block_value1,
            };

            let load_value2 = if *effective_address % 8 + load_size.byte_size() > 8 {
                *emid = (*emid).checked_add(1).unwrap();
                Some(MemoryTableEntry {
                    eid,
                    emid: *emid,
                    addr: *effective_address / 8 + 1,
                    ltype: LocationType::Heap,
                    atype: AccessType::Read,
                    is_mutable: true,
                    // The value will be used to lookup within imtable, hence block_value is given here
                    value: *block_value2,
                })
            } else {
                None
            };

            *emid = (*emid).checked_add(1).unwrap();
            let push_value = MemoryTableEntry {
                eid,
                emid: *emid,
                addr: sp_before_execution + 1,
                ltype: LocationType::Stack,
                atype: AccessType::Write,
                is_mutable: true,
                value: *value,
            };

            vec![
                vec![load_address_from_stack, load_value1],
                load_value2.map_or(vec![], |v| vec![v]),
                vec![push_value],
            ]
            .concat()
        }
        StepInfo::Store {
            vtype,
            store_size,
            raw_address,
            effective_address,
            value,
            pre_block_value1,
            updated_block_value1,
            pre_block_value2,
            updated_block_value2,
            ..
        } => {
            let load_value_from_stack = MemoryTableEntry {
                eid,
                emid: *emid,
                addr: sp_before_execution + 1,
                ltype: LocationType::Stack,
                atype: AccessType::Read,
                is_mutable: true,
                value: *value,
            };
            *emid = (*emid).checked_add(1).unwrap();

            let load_address_from_stack = MemoryTableEntry {
                eid,
                emid: *emid,
                addr: sp_before_execution + 2,
                ltype: LocationType::Stack,
                atype: AccessType::Read,
                is_mutable: true,
                value: *raw_address as u64,
            };
            *emid = (*emid).checked_add(1).unwrap();

            let load_value1 = MemoryTableEntry {
                eid,
                emid: *emid,
                addr: *effective_address / 8,
                ltype: LocationType::Heap,
                atype: AccessType::Read,
                is_mutable: true,
                // The value will be used to lookup within imtable, hence block_value is given here
                value: *pre_block_value1,
            };
            *emid = (*emid).checked_add(1).unwrap();

            let write_value1 = MemoryTableEntry {
                eid,
                emid: *emid,
                addr: *effective_address / 8,
                ltype: LocationType::Heap,
                atype: AccessType::Write,
                is_mutable: true,
                // The value will be used to lookup within imtable, hence block_value is given here
                value: *updated_block_value1,
            };

            if *effective_address % 8 + store_size.byte_size() > 8 {
                *emid = (*emid).checked_add(1).unwrap();
                let load_value2 = MemoryTableEntry {
                    eid,
                    emid: *emid,
                    addr: *effective_address / 8 + 1,
                    ltype: LocationType::Heap,
                    atype: AccessType::Read,
                    is_mutable: true,
                    // The value will be used to lookup within imtable, hence block_value is given here
                    value: *pre_block_value2,
                };

                *emid = (*emid).checked_add(1).unwrap();
                let write_value2 = MemoryTableEntry {
                    eid,
                    emid: *emid,
                    addr: *effective_address / 8 + 1,
                    ltype: LocationType::Heap,
                    atype: AccessType::Write,
                    is_mutable: true,
                    // The value will be used to lookup within imtable, hence block_value is given here
                    value: *updated_block_value2,
                };
                vec![
                    load_value_from_stack,
                    load_address_from_stack,
                    load_value1,
                    write_value1,
                    load_value2,
                    write_value2,
                ]
            } else {
                vec![
                    load_value_from_stack,
                    load_address_from_stack,
                    load_value1,
                    write_value1,
                ]
            }
        }
        // StepInfo::MemorySize => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[],
        //     &[event.allocated_memory_pages as u32 as u64],
        // ),
        // StepInfo::MemoryGrow { grow_size, result } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*grow_size as u32 as u64],
        //     &[*result as u32 as u64],
        // ),
        // StepInfo::I32Const { value } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[],
        //     &[*value as u32 as u64],
        // ),
        // StepInfo::Const32 { value, addr } => {
        //     mem_op_from_stack_only_step(sp_before_execution, eid, emid, &[], &[*value as u64])
        // }
        StepInfo::Const32 { value, addr } => vec![MemoryTableEntry {
            eid,
            emid: *emid,
            addr: *addr,
            ltype: LocationType::Stack,
            atype: AccessType::Write,
            is_mutable: false,
            value: *value as u64,
        }],
        // StepInfo::ConstRef { value } => {
        //     mem_op_from_stack_only_step(sp_before_execution, eid, emid, &[], &[*value])
        // }
        // StepInfo::F32Const { value } => {
        //     mem_op_from_stack_only_step(sp_before_execution, eid, emid, &[], &[*value as u64])
        // }
        // StepInfo::F64Const { value } => {
        //     mem_op_from_stack_only_step(sp_before_execution, eid, emid, &[], &[*value as u64])
        // }
        // StepInfo::I32BinShiftOp {
        //     left, right, value, ..
        // }
        // | StepInfo::I32BinBitOp {
        //     left, right, value, ..
        // } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*right as u32 as u64, *left as u32 as u64],
        //     &[*value as u32 as u64],
        // ),
        // StepInfo::I32BinOp {
        //     left, right, value, ..
        // } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*right as u32 as u64, *left as u32 as u64],
        //     &[*value as u32 as u64],
        // ),
        StepInfo::I32BinOp {
            left,
            right,
            value,
            left_addr,
            right_addr,
            ..
        } => {
            let mut ops = vec![];
            ops.push(MemoryTableEntry {
                eid,
                emid: *emid,
                addr: *right_addr,
                ltype: LocationType::Stack,
                atype: AccessType::Read,
                is_mutable: true,
                value: *right as u32 as u64,
            });

            *emid = (*emid).checked_add(1).unwrap();

            ops.push(MemoryTableEntry {
                eid,
                emid: *emid,
                addr: *left_addr,
                ltype: LocationType::Stack,
                atype: AccessType::Read,
                is_mutable: true,
                value: *left as u32 as u64,
            });

            *emid = (*emid).checked_add(1).unwrap();

            ops.push(MemoryTableEntry {
                eid,
                emid: *emid,
                addr: *left_addr,
                ltype: LocationType::Stack,
                atype: AccessType::Write,
                is_mutable: true,
                value: *value as u32 as u64,
            });

            ops
        }
        // StepInfo::I32Comp {
        //     left, right, value, ..
        // } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*right as u32 as u64, *left as u32 as u64],
        //     &[*value as u32 as u64],
        // ),

        // StepInfo::I64BinOp {
        //     left, right, value, ..
        // }
        // | StepInfo::I64BinShiftOp {
        //     left, right, value, ..
        // }
        // | StepInfo::I64BinBitOp {
        //     left, right, value, ..
        // } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*right as u64, *left as u64],
        //     &[*value as u64],
        // ),

        // StepInfo::I64Const { value } => {
        //     mem_op_from_stack_only_step(sp_before_execution, eid, emid, &[], &[*value as u64])
        // }
        // StepInfo::I64Comp {
        //     left, right, value, ..
        // } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*right as u64, *left as u64],
        //     &[*value as u32 as u64],
        // ),
        // StepInfo::UnaryOp {
        //     vtype,
        //     operand,
        //     result,
        //     ..
        // } => mem_op_from_stack_only_step(sp_before_execution, eid, emid, &[*operand], &[*result]),
        // StepInfo::CompZ {
        //     vtype,
        //     value,
        //     result,
        // } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*value],
        //     &[*result as u32 as u64],
        // ),
        // StepInfo::I32WrapI64 { value, result } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*value as u64],
        //     &[*result as u32 as u64],
        // ),
        // StepInfo::I64ExtendI32 { value, result, .. } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*value as u32 as u64],
        //     &[*result as u64],
        // ),
        // StepInfo::I32SignExtendI8 { value, result }
        // | StepInfo::I32SignExtendI16 { value, result } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*value as u32 as u64],
        //     &[*result as u32 as u64],
        // ),
        // StepInfo::I64SignExtendI8 { value, result }
        // | StepInfo::I64SignExtendI16 { value, result }
        // | StepInfo::I64SignExtendI32 { value, result } => mem_op_from_stack_only_step(
        //     sp_before_execution,
        //     eid,
        //     emid,
        //     &[*value as u64],
        //     &[*result as u64],
        // ),
        _ => vec![],
    }
}
