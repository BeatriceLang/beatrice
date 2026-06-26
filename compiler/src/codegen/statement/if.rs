use crate::{
    ast::{Block, Type, expression::Expression},
    codegen::Codegen,
};

impl Codegen<'_> {
    pub(super) fn compile_if(&mut self, cond: &Expression, body: &Block) {
        let cond = self.compile_expr(cond).unwrap();

        assert!(matches!(cond.ty, Type::Bool));

        let current_block = self.builder.get_insert_block().unwrap();
        let current_function = current_block.get_parent().unwrap();

        let then_block = self.ctx.append_basic_block(current_function, "if_then");
        let end_block = self.ctx.append_basic_block(current_function, "if_end");

        self.builder
            .build_conditional_branch(cond.inner.into_int_value(), then_block, end_block)
            .unwrap();

        self.builder.position_at_end(then_block);

        self.compile_block(body);

        // Force jump to end_block if then_block does not have a return.
        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.builder.build_unconditional_branch(end_block).unwrap();
        }

        self.builder.position_at_end(end_block);
    }
}
