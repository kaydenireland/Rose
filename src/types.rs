use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use crate::analyzer::Type;

pub fn llvm_basic_type<'ctx>(ctx: &'ctx Context, ty: &Type) -> Option<BasicTypeEnum<'ctx>> {
    match ty {
        Type::Int => Some(ctx.i32_type().into()),
        Type::Bool => Some(ctx.bool_type().into()),
        Type::Unknown => None,
    }
}
