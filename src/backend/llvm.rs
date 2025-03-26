use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    values::FunctionValue,
};

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            module: context.create_module("minilang"),
            builder: context.create_builder(),
        }
    }

    pub fn compile(&self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Literal(Literal::Int(n)) => {
                let i32_type = self.context.i32_type();
                let value = i32_type.const_int(*n as u64, false);
                self.builder.build_return(Some(&value));
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}