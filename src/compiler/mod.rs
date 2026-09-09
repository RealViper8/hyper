//! Hyper compiler core — IR, lowering, LLVM / Cranelift AOT codegen, and linking.

#[path = "../../compiler/ir.rs"]
pub mod ir;

// Legacy Rust runtime (formerly JIT symbol implementations). Kept for unit tests
// and ABI reference; execution uses the C runtime linked by `emit_exe`.
#[allow(dead_code)]
#[path = "../../compiler/runtime/mod.rs"]
pub mod runtime;

#[path = "../../compiler/codegen.rs"]
pub mod codegen;

#[path = "../../compiler/llvm_emit.rs"]
pub mod llvm_emit;

#[path = "../../compiler/lowering.rs"]
pub mod lowering;

pub use lowering::{run_compile, try_aot_run, CompileMode};
