use crate::ast::{AST, Expr, Op, Type};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};
use std::collections::HashMap;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub variables: Vec<HashMap<String, (Type, PointerValue<'ctx>)>>,
    pub current_function: Option<FunctionValue<'ctx>>,
    pub current_return_type: Option<Type>,
    pub loop_exit_blocks: Vec<BasicBlock<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        CodeGen {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            variables: vec![HashMap::new()],
            current_function: None,
            current_return_type: None,
            loop_exit_blocks: Vec::new(),
        }
    }

    fn get_var(&self, name: &str) -> Option<(Type, PointerValue<'ctx>)> {
        for scope in self.variables.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    fn set_var(&mut self, name: String, ty: Type, ptr: PointerValue<'ctx>) {
        if let Some(scope) = self.variables.last_mut() {
            scope.insert(name, (ty, ptr));
        }
    }

    fn llvm_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::I8 => self.context.i8_type().into(),
            Type::I16 => self.context.i16_type().into(),
            Type::I32 => self.context.i32_type().into(),
            Type::I64 => self.context.i64_type().into(),
            Type::I128 => self.context.i128_type().into(),
            Type::F32 => self.context.f32_type().into(),
            Type::F64 => self.context.f64_type().into(),
            Type::Bool => self.context.bool_type().into(),
            Type::Void => panic!("void cannot be represented as BasicTypeEnum"),
            Type::Ptr(_inner) => self.context.ptr_type(AddressSpace::default()).into(),
            Type::Custom(name) => panic!("custom type '{}' not supported in codegen", name),
        }
    }

    fn llvm_metadata_type(&self, ty: &Type) -> BasicMetadataTypeEnum<'ctx> {
        self.llvm_type(ty).into()
    }

    /// Нулевое значение для типа (для неявного return в конце функции)
    fn default_value(&self, ty: &Type) -> BasicValueEnum<'ctx> {
        match ty {
            Type::I8 => self.context.i8_type().const_zero().into(),
            Type::I16 => self.context.i16_type().const_zero().into(),
            Type::I32 => self.context.i32_type().const_zero().into(),
            Type::I64 => self.context.i64_type().const_zero().into(),
            Type::I128 => self.context.i128_type().const_zero().into(),
            Type::F32 => self.context.f32_type().const_zero().into(),
            Type::F64 => self.context.f64_type().const_zero().into(),
            Type::Bool => self.context.bool_type().const_zero().into(),
            Type::Ptr(_) => self
                .context
                .ptr_type(AddressSpace::default())
                .const_null()
                .into(),
            Type::Void => panic!("default_value called for void"),
            Type::Custom(name) => panic!("custom type '{}' has no default value", name),
        }
    }

    fn create_entry_block_alloca(&self, name: &str, ty: &Type) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();
        let func = self.current_function.expect("alloca outside function");
        let entry = func.get_first_basic_block().unwrap();

        if let Some(first_instr) = entry.get_first_instruction() {
            builder.position_before(&first_instr);
        } else {
            builder.position_at_end(entry);
        }

        builder
            .build_alloca(self.llvm_type(ty), name)
            .expect("alloca failed")
    }

    fn infer_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Number(_) => Type::I64,
            Expr::FloatLit(_) => Type::F64,
            Expr::Boolean(_) => Type::Bool,
            _ => panic!("cannot infer type for this expression, annotate it explicitly"),
        }
    }

    pub fn generate_program(&mut self, ast: Vec<AST>) {
        for stmt in ast {
            self.generate_stmt(&stmt);
        }
    }

    fn generate_stmt(&mut self, stmt: &AST) {
        match stmt {
            AST::VarDecl { name, value, ty } => {
                let final_ty = ty.clone().unwrap_or_else(|| self.infer_type(value));
                let val = self.generate_expr(value).expect("codegen expr failed");
                let ptr = self.create_entry_block_alloca(name, &final_ty);
                self.builder.build_store(ptr, val).expect("store failed");
                self.set_var(name.clone(), final_ty, ptr);
            }

            AST::Return { value } => match &self.current_return_type {
                None | Some(Type::Void) => {
                    self.builder.build_return(None).expect("void return failed");
                }
                Some(_) => {
                    let ret_val = self.generate_expr(value).expect("codegen return failed");
                    self.builder
                        .build_return(Some(&ret_val))
                        .expect("build_return failed");
                }
            },

            AST::FuncDecl {
                name,
                args,
                body,
                return_type,
            } => {
                self.current_return_type = return_type.clone();

                let arg_types: Vec<BasicMetadataTypeEnum> = args
                    .iter()
                    .map(|(_, ty)| self.llvm_metadata_type(ty))
                    .collect();

                let fn_type = match return_type {
                    None | Some(Type::Void) => self.context.void_type().fn_type(&arg_types, false),
                    Some(rt) => self.llvm_type(rt).fn_type(&arg_types, false),
                };

                let func = self.module.add_function(name, fn_type, None);
                let entry = self.context.append_basic_block(func, "entry");
                self.builder.position_at_end(entry);

                self.current_function = Some(func);
                self.variables.push(HashMap::new());

                for (i, (arg_name, arg_ty)) in args.iter().enumerate() {
                    let param = func.get_nth_param(i as u32).expect("param not found");
                    let ptr = self.create_entry_block_alloca(arg_name, arg_ty);
                    self.builder
                        .build_store(ptr, param)
                        .expect("store param failed");
                    self.set_var(arg_name.clone(), arg_ty.clone(), ptr);
                }

                for stmt in body {
                    self.generate_stmt(stmt);
                }

                let current_block = self.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    match &self.current_return_type {
                        None | Some(Type::Void) => {
                            self.builder.build_return(None).expect("void return failed");
                        }
                        Some(ret_ty) => {
                            let default_val = self.default_value(ret_ty);
                            self.builder
                                .build_return(Some(&default_val))
                                .expect("return failed");
                        }
                    }
                }

                self.variables.pop();
                self.current_function = None;
                self.current_return_type = None;
            }

            AST::Loop { body } => {
                let func = self.current_function.expect("loop outside function");
                let loop_block = self.context.append_basic_block(func, "loop");
                let after_block = self.context.append_basic_block(func, "after_loop");

                self.loop_exit_blocks.push(after_block);

                self.builder
                    .build_unconditional_branch(loop_block)
                    .expect("br to loop failed");
                self.builder.position_at_end(loop_block);

                self.variables.push(HashMap::new());
                for stmt in body {
                    self.generate_stmt(stmt);
                }
                self.variables.pop();

                let current_block = self.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    self.builder
                        .build_unconditional_branch(loop_block)
                        .expect("loop backedge failed");
                }

                self.builder.position_at_end(after_block);
                self.loop_exit_blocks.pop();
            }


            AST::While { condition, body } => {
                let func = self.current_function.expect("while outside function");
                let cond_block = self.context.append_basic_block(func, "while_cond");
                let body_block = self.context.append_basic_block(func, "while_body");
                let after_block = self.context.append_basic_block(func, "after_while");

                self.loop_exit_blocks.push(after_block);

                self.builder
                    .build_unconditional_branch(cond_block)
                    .expect("br to while_cond failed");
                self.builder.position_at_end(cond_block);

                let cond_val = self
                    .generate_expr(condition)
                    .expect("while condition codegen failed");
                self.builder
                    .build_conditional_branch(cond_val.into_int_value(), body_block, after_block)
                    .expect("conditional branch in while failed");

                self.builder.position_at_end(body_block);
                self.variables.push(HashMap::new());
                for stmt in body {
                    self.generate_stmt(stmt);
                }
                self.variables.pop();

                let current_block = self.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    self.builder
                        .build_unconditional_branch(cond_block)
                        .expect("while backedge failed");
                }

                self.builder.position_at_end(after_block);
                self.loop_exit_blocks.pop();
            }

            AST::For { var, start, end, body } => {
                let func = self.current_function.expect("for outside function");
                let cond_block = self.context.append_basic_block(func, "for_cond");
                let body_block = self.context.append_basic_block(func, "for_body");
                let after_block = self.context.append_basic_block(func, "after_for");

                let start_val = self.generate_expr(start).expect("for start codegen failed");
                let end_val = self.generate_expr(end).expect("for end codegen failed");

                let i64_ty = Type::I64;
                let ptr = self.create_entry_block_alloca(var, &i64_ty);
                self.builder.build_store(ptr, start_val).expect("for init store failed");
                self.set_var(var.clone(), i64_ty, ptr);

                self.loop_exit_blocks.push(after_block);

                self.builder
                    .build_unconditional_branch(cond_block)
                    .expect("br to for_cond failed");
                self.builder.position_at_end(cond_block);

                let var_val = self.builder
                    .build_load(self.context.i64_type(), ptr, var)
                    .expect("for var load failed")
                    .into_int_value();
                let cmp = self.builder
                    .build_int_compare(inkwell::IntPredicate::SLT, var_val, end_val.into_int_value(), "for_cmp")
                    .expect("for cmp failed");
                self.builder
                    .build_conditional_branch(cmp, body_block, after_block)
                    .expect("conditional branch in for failed");

                self.builder.position_at_end(body_block);
                self.variables.push(HashMap::new());
                for stmt in body {
                    self.generate_stmt(stmt);
                }
                self.variables.pop();

                let current_block = self.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    let loaded = self.builder
                        .build_load(self.context.i64_type(), ptr, var)
                        .expect("for inc load failed")
                        .into_int_value();
                    let one = self.context.i64_type().const_int(1, false);
                    let inc = self.builder
                        .build_int_add(loaded, one, "for_inc")
                        .expect("for inc failed");
                    self.builder.build_store(ptr, inc).expect("for inc store failed");
                    self.builder
                        .build_unconditional_branch(cond_block)
                        .expect("for backedge failed");
                }

                self.builder.position_at_end(after_block);
                self.loop_exit_blocks.pop();
            }

            AST::Break => {
                let after_block = self.loop_exit_blocks.last().expect("break outside loop");
                self.builder
                    .build_unconditional_branch(*after_block)
                    .expect("break branch failed");
            }

            AST::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let func = self.current_function.expect("if outside function");
                let then_block = self.context.append_basic_block(func, "then");
                let else_block = self.context.append_basic_block(func, "else");
                let merge_block = self.context.append_basic_block(func, "merge");

                let cond_val = self
                    .generate_expr(condition)
                    .expect("if condition codegen failed");
                self.builder
                    .build_conditional_branch(cond_val.into_int_value(), then_block, else_block)
                    .expect("conditional branch failed");

                self.builder.position_at_end(then_block);
                self.variables.push(HashMap::new());
                for stmt in then_branch {
                    self.generate_stmt(stmt);
                }
                self.variables.pop();
                let current_block = self.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    self.builder
                        .build_unconditional_branch(merge_block)
                        .expect("then branch to merge failed");
                }

                self.builder.position_at_end(else_block);
                if let Some(else_stmts) = else_branch {
                    self.variables.push(HashMap::new());
                    for stmt in else_stmts {
                        self.generate_stmt(stmt);
                    }
                    self.variables.pop();
                }
                let current_block = self.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    self.builder
                        .build_unconditional_branch(merge_block)
                        .expect("else branch to merge failed");
                }

                self.builder.position_at_end(merge_block);
            }

            AST::ExternFn {
                name,
                args,
                return_type,
            } => {
                let arg_types: Vec<BasicMetadataTypeEnum> = args
                    .iter()
                    .map(|(_, ty)| self.llvm_metadata_type(ty))
                    .collect();

                let fn_type = match return_type {
                    None | Some(Type::Void) => self.context.void_type().fn_type(&arg_types, false),
                    Some(rt) => self.llvm_type(rt).fn_type(&arg_types, false),
                };

                self.module
                    .add_function(name, fn_type, Some(inkwell::module::Linkage::External));
            }
            AST::AsmBlock { assembly } => {
                let _ = self.current_function.expect("asm block outside function");
                let void_ty = self.context.void_type().fn_type(&[], false);
                let asm_ptr = self.context.create_inline_asm(
                    void_ty,
                    assembly.clone(),
                    "".to_string(),
                    true,
                    false,
                    Some(inkwell::InlineAsmDialect::Intel),
                    false,
                );
                self.builder
                    .build_indirect_call(void_ty, asm_ptr, &[], "asm")
                    .expect("asm call failed");
            }

            AST::ExprStmt(expr) => {
                let _ = self.generate_expr(expr);
            }
        }
    }

    fn generate_expr(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expr::Number(n) => Ok(self.context.i64_type().const_int(*n as u64, false).into()),

            Expr::FloatLit(f) => Ok(self.context.f64_type().const_float(*f).into()),

            Expr::Boolean(val) => Ok(self
                .context
                .bool_type()
                .const_int(*val as u64, false)
                .into()),

            Expr::StringLit(s) => {
                let global = self
                    .builder
                    .build_global_string_ptr(s, "str")
                    .map_err(|e| e.to_string())?;
                Ok(global.as_pointer_value().into())
            }

            Expr::Identifier(name) => {
                let (ty, ptr) = self
                    .get_var(name)
                    .ok_or_else(|| format!("unknown variable: {}", name))?;
                let llvm_ty = self.llvm_type(&ty);
                self.builder
                    .build_load(llvm_ty, ptr, name)
                    .map_err(|e| e.to_string())
                    .map(|v| v.into())
            }

            Expr::BinOp { left, op, right } => {
                let lhs_val = self.generate_expr(left)?;
                let rhs_val = self.generate_expr(right)?;

                match (lhs_val, rhs_val) {
                    (BasicValueEnum::IntValue(lhs), BasicValueEnum::IntValue(rhs)) => match op {
                        Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Percent => {
                            let res = match op {
                                Op::Add => self.builder.build_int_add(lhs, rhs, "add"),
                                Op::Sub => self.builder.build_int_sub(lhs, rhs, "sub"),
                                Op::Mul => self.builder.build_int_mul(lhs, rhs, "mul"),
                                Op::Div => self.builder.build_int_signed_div(lhs, rhs, "div"),
                                Op::Percent => self.builder.build_int_signed_rem(lhs, rhs, "rem"),
                                _ => unreachable!(),
                            }
                            .map_err(|e| e.to_string())?;
                            Ok(res.into())
                        }
                        Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                            let pred = match op {
                                Op::Eq => IntPredicate::EQ,
                                Op::Neq => IntPredicate::NE,
                                Op::Lt => IntPredicate::SLT,
                                Op::Gt => IntPredicate::SGT,
                                Op::Le => IntPredicate::SLE,
                                Op::Ge => IntPredicate::SGE,
                                _ => unreachable!(),
                            };
                            Ok(self
                                .builder
                                .build_int_compare(pred, lhs, rhs, "icmp")
                                .map_err(|e| e.to_string())?
                                .into())
                        }
                    },

                    (BasicValueEnum::FloatValue(lhs), BasicValueEnum::FloatValue(rhs)) => {
                        match op {
                            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Percent => {
                                let res = match op {
                                    Op::Add => self.builder.build_float_add(lhs, rhs, "fadd"),
                                    Op::Sub => self.builder.build_float_sub(lhs, rhs, "fsub"),
                                    Op::Mul => self.builder.build_float_mul(lhs, rhs, "fmul"),
                                    Op::Div => self.builder.build_float_div(lhs, rhs, "fdiv"),
                                    Op::Percent => self.builder.build_float_rem(lhs, rhs, "frem"),
                                    _ => unreachable!(),
                                }
                                .map_err(|e| e.to_string())?;
                                Ok(res.into())
                            }
                            Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                                let pred = match op {
                                    Op::Eq => FloatPredicate::OEQ,
                                    Op::Neq => FloatPredicate::ONE,
                                    Op::Lt => FloatPredicate::OLT,
                                    Op::Gt => FloatPredicate::OGT,
                                    Op::Le => FloatPredicate::OLE,
                                    Op::Ge => FloatPredicate::OGE,
                                    _ => unreachable!(),
                                };
                                Ok(self
                                    .builder
                                    .build_float_compare(pred, lhs, rhs, "fcmp")
                                    .map_err(|e| e.to_string())?
                                    .into())
                            }
                        }
                    }

                    _ => Err("type mismatch in binary operation".into()),
                }
            }

            Expr::FuncCall { name, args } => {
                let func = self
                    .module
                    .get_function(name)
                    .ok_or_else(|| format!("function not found: {}", name))?;

                let mut arg_vals: Vec<BasicMetadataValueEnum> = Vec::new();
                for arg in args {
                    arg_vals.push(self.generate_expr(arg)?.into());
                }

                let call = self
                    .builder
                    .build_call(func, &arg_vals, "call")
                    .map_err(|e| e.to_string())?;

                match call.try_as_basic_value().basic() {
                    Some(v) => Ok(v),
                    None => Err("void call used in value context".into()),
                }
            }
        }
    }

    pub fn get_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}
