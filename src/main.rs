use std::env;
use std::fs;
use std::io::{self, Write};

mod ast;
mod error;
mod scanner;
mod parser;
mod driver;
mod environment;
mod fileio;
mod json;
mod semantic;
mod module;
mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let _ = writeln!(
            io::stderr(),
            "Usage: {} <tokenize|parse|run|typecheck|compile> <filename> [options]\n\
             \n\
             run <file>                          AOT compile to a temp exe and run it\n\
             compile <file>                      same AOT run (default)\n\
             compile <file> --emit-ir            print Hyper IR only\n\
             compile <file> --emit-llvm [path]   emit LLVM IR (default out.ll)\n\
             compile <file> --emit-obj [path]    emit Cranelift object (default a.o)\n\
             compile <file> --emit-exe [path]    emit AOT executable (default hyper_out)\n\
             --backend llvm|cranelift            AOT backend (also HYPER_CODEGEN)\n\
             typecheck <file>                    typecheck only",
            args[0]
        );
        return;
    }

    // Apply --backend before any compile/run so HYPER_CODEGEN is set for codegen.
    apply_backend_flag(&args);

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        let _ = writeln!(io::stderr(), "Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            scanner::scan_tokens(&file_contents);
        }
        "parse" => {
            parser::run_parse(file_contents);
        }
        "run" => {
            // AOT: emit temp executable, run, propagate exit code.
            match compiler::try_aot_run(&file_contents, filename) {
                Ok(0) => {}
                Ok(code) => std::process::exit(code),
                Err(errors) => {
                    for e in &errors {
                        error::report_formatted(e);
                    }
                    std::process::exit(65);
                }
            }
        }
        "typecheck" => {
            semantic::run_typecheck(file_contents);
        }
        "compile" => {
            let mode = parse_compile_mode(&args[3..]);
            compiler::run_compile(file_contents, filename, mode);
        }
        "evaluate" => {
            let _ = writeln!(
                io::stderr(),
                "error: 'evaluate' was removed — Hyper is compiler-only.\n\
                 Use a small program with print(...) and `hyper run`, or `hyper compile`."
            );
            std::process::exit(64);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

fn apply_backend_flag(args: &[String]) {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--backend" {
            if let Some(val) = args.get(i + 1) {
                match val.to_ascii_lowercase().as_str() {
                    "llvm" => unsafe { env::set_var("HYPER_CODEGEN", "llvm") },
                    "cranelift" | "clif" => unsafe { env::set_var("HYPER_CODEGEN", "cranelift") },
                    other => {
                        let _ = writeln!(
                            io::stderr(),
                            "Unknown --backend {other} (expected llvm|cranelift)"
                        );
                        std::process::exit(64);
                    }
                }
            } else {
                let _ = writeln!(io::stderr(), "--backend requires llvm|cranelift");
                std::process::exit(64);
            }
            i += 2;
            continue;
        }
        i += 1;
    }
}

fn parse_compile_mode(args: &[String]) -> compiler::CompileMode {
    let filtered: Vec<&String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            if *a == "--backend" {
                return false;
            }
            if *i > 0 && args[*i - 1] == "--backend" {
                return false;
            }
            true
        })
        .map(|(_, a)| a)
        .collect();

    if filtered.is_empty() {
        return compiler::CompileMode::AotRun;
    }
    match filtered[0].as_str() {
        "--emit-ir" => compiler::CompileMode::EmitIr,
        "--emit-llvm" => {
            let path = filtered
                .get(1)
                .cloned()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "out.ll".to_string());
            compiler::CompileMode::EmitLlvm { path }
        }
        "--emit-obj" => {
            let path = filtered
                .get(1)
                .cloned()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "a.o".to_string());
            compiler::CompileMode::EmitObj { path }
        }
        "--emit-exe" => {
            let path = filtered
                .get(1)
                .cloned()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "hyper_out".to_string());
            compiler::CompileMode::EmitExe { path }
        }
        other => {
            let _ = writeln!(io::stderr(), "Unknown compile option: {other}");
            let _ = writeln!(
                io::stderr(),
                "Expected: --emit-ir | --emit-llvm [path] | --emit-obj [path] | --emit-exe [path]"
            );
            std::process::exit(64);
        }
    }
}
