use inkwell::basic_block::BasicBlock;

use crate::codegen::Codegen;

impl<'a> Codegen<'a> {
    pub(super) fn current_block(&self) -> BasicBlock<'a> {
        self.builder.get_insert_block().unwrap()
    }
}
