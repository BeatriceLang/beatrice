use crate::{
    ast::{Block, Type, expression::Expression},
    codegen::Codegen,
};

impl Codegen<'_> {
    pub(super) fn compile_while(&mut self, cond: &Expression, body: &Block) {
        let current_block = self.builder.get_insert_block().unwrap();
        let current_function = current_block.get_parent().unwrap();

        let cond_block = self.ctx.append_basic_block(current_function, "while_cond");
        let body_block = self.ctx.append_basic_block(current_function, "while_body");
        let end_block = self.ctx.append_basic_block(current_function, "while_end");

        self.builder.build_unconditional_branch(cond_block).unwrap();

        self.builder.position_at_end(cond_block);
        let cond = self.compile_expr(cond).unwrap();

        assert_eq!(cond.ty, Type::Bool);

        self.builder
            .build_conditional_branch(cond.inner.into_int_value(), body_block, end_block)
            .unwrap();

        self.builder.position_at_end(body_block);

        self.compile_block(body);

        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.builder.build_unconditional_branch(cond_block).unwrap();
        }

        self.builder.position_at_end(end_block);
    }
}
