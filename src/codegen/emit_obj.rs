use std::path::Path;

use anyhow::{Result, anyhow};
use inkwell::{
    OptimizationLevel,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
};

use crate::codegen::Codegen;

impl Codegen<'_> {
    pub fn emit_object(&self, path: &Path) -> Result<()> {
        Target::initialize_native(&InitializationConfig::default()).map_err(|err| anyhow!(err))?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).map_err(|err| anyhow!(err.to_string()))?;

        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                OptimizationLevel::Default,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .ok_or_else(|| anyhow!("failed to create target machine"))?;

        self.module.set_triple(&triple);

        machine
            .write_to_file(&self.module, FileType::Object, path)
            .map_err(|err| anyhow!(err.to_string()))?;

        Ok(())
    }
}
