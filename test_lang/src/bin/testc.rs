use inkwell::context::Context;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, TargetMachineOptions};
use testl::source::SourceID;
use std::error::Error;
use std::io;
use testl::ast_store::ASTStore;
use testl::codegen::CodeGen;
use testl::parser;

const PASSES: &str = "mem2reg,instcombine,reassociate,simplifycfg";

fn main() -> Result<(), Box<dyn Error>> {
    let src = io::read_to_string(io::stdin())?;

    let (program, ast_store) = parser::run(&src, SourceID::new(1), ASTStore::new())?;

    Target::initialize_native(&InitializationConfig::default()).unwrap();

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let target_options = TargetMachineOptions::default();
    let target_machine = target
        .create_target_machine_from_options(&triple, target_options)
        .unwrap();

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "test");
    codegen.ir.module.set_triple(&triple);

    codegen.run(&ast_store, &program)?;

    codegen
        .ir
        .module
        .run_passes(PASSES, &target_machine, PassBuilderOptions::create())
        .unwrap();
    codegen.ir.print()?;

    Ok(())
}
