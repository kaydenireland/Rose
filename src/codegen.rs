use std::path::Path;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};

use crate::analyzer::MTree;

pub struct LlvmCodegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: inkwell::builder::Builder<'ctx>,
}

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self { context, module, builder }
    }

    pub fn emit_program(&mut self, tree: &MTree) -> Result<(), String> {
        match tree {
            MTree::START { funcs } => {
                // Phase 1: declare all functions
                for f in funcs {
                    self.declare_function(f)?;
                }
                // Phase 2: define bodies
                for f in funcs {
                    self.define_function(f)?;
                }
                Ok(())
            }
            _ => Err("Top-level node must be START".into()),
        }
    }

    fn declare_function(&mut self, node: &MTree) -> Result<(), String> {
        let MTree::FUNC_DECL { name, params, ret_type, .. } = node else {
            return Ok(());
        };

        // FIX: explicit Option<BasicTypeEnum>
        let fn_ret: Option<BasicTypeEnum<'ctx>> = match ret_type {
            crate::analyzer::Type::Int =>
                Some(self.context.i32_type().into()),

            crate::analyzer::Type::Bool =>
                Some(self.context.bool_type().into()),

            crate::analyzer::Type::Unknown =>
                None, // treat Unknown as void
        };

        let mut param_types = Vec::new();
        for (_, ty) in params {
            match ty {
                crate::analyzer::Type::Int =>
                    param_types.push(self.context.i32_type().into()),

                crate::analyzer::Type::Bool =>
                    param_types.push(self.context.bool_type().into()),

                crate::analyzer::Type::Unknown => {
                    return Err(format!(
                        "Function '{}' has unknown param type",
                        name
                    ));
                }
            }
        }

        let fn_type = match fn_ret {
            Some(ret) => ret.fn_type(&param_types, false),
            None => self.context.void_type().fn_type(&param_types, false),
        };

        self.module.add_function(name, fn_type, None);
        Ok(())
    }

    fn define_function(&mut self, node: &MTree) -> Result<(), String> {
        let MTree::FUNC_DECL { name, body, ret_type, .. } = node else {
            return Ok(());
        };

        let function = self
            .module
            .get_function(name)
            .ok_or_else(|| format!("Internal: function '{}' not declared", name))?;

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Stub return (unchanged logic)
        match ret_type {
            crate::analyzer::Type::Int => {
                self.builder.build_return(Some(
                    &self.context.i32_type().const_int(0, false)
                ));
            }
            crate::analyzer::Type::Bool => {
                self.builder.build_return(Some(
                    &self.context.bool_type().const_int(0, false)
                ));
            }
            crate::analyzer::Type::Unknown => {
                self.builder.build_return(None);
            }
        }

        Ok(())
    }

    pub fn verify(&self) -> Result<(), String> {
        self.module.verify().map_err(|e| e.to_string())
    }

    pub fn write_ir_to_file(&self, path: impl AsRef<Path>) -> Result<(), String> {
        self.module.print_to_file(path).map_err(|e| e.to_string())
    }
}
