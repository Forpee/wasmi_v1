pub mod etable;
pub mod imtable;
pub mod mtable;
use wasmi_core::UntypedValue;

use crate::{AsContext, Global, Memory};

use self::{
    etable::ETable,
    imtable::{IMTable, VarType},
    mtable::{memory_event_of_step, MTable},
};

#[derive(Debug)]
pub struct Tracer {
    pub imtable: IMTable,
    pub etable: ETable,
}

impl Tracer {
    pub fn new() -> Self {
        Tracer {
            imtable: IMTable::default(),
            etable: ETable::default(),
        }
    }

    pub fn push_init_memory(&mut self, memref: Memory, context: impl AsContext) {
        let pages: u32 = memref.ty(&context).initial_pages().into();
        for i in 0..(pages * 8192) {
            let mut buf = [0u8; 8];
            memref
                .read(&context, (i * 8).try_into().unwrap(), &mut buf)
                .unwrap();
            self.imtable
                .push(false, true, i, i, VarType::I64, u64::from_le_bytes(buf));
        }

        let max_pages = memref.ty(&context).maximum_pages();
        self.imtable.push(
            false,
            true,
            pages * 8192,
            max_pages
                .map(|limit| u32::from(limit) * 8192 - 1)
                .unwrap_or(u32::MAX),
            VarType::I64,
            0,
        );
    }

    pub(crate) fn push_global(
        &mut self,
        globalidx: u32,
        globalref: &Global,
        context: impl AsContext,
    ) {
        let vtype = globalref.ty(&context);
        let vtype_content = globalref.ty(&context).content();
        let val = UntypedValue::from(globalref.get(&context));
        self.imtable.push(
            true,
            vtype.mutability().is_mut(),
            globalidx,
            globalidx,
            vtype_content.into(),
            val.to_bits(),
        );
    }

    pub fn get_mtable(&self) -> MTable {
        let mentries = self
            .etable
            .entries()
            .iter()
            .map(|eentry| memory_event_of_step(eentry, &mut 1))
            .collect::<Vec<Vec<_>>>()
            .concat();

        MTable::new(mentries, &self.imtable)
    }
}
