use alc_ast_lowering::{ir, ty};
use alc_command_option::{CommandOptions, Gc};
use alc_diagnostic::{FileId, Result};
use log::debug;

pub fn collect(
    command_options: &CommandOptions,
    _file_id: FileId,
    _ty_sess: &ty::TySess,
    ir: ir::Ir,
) -> Result<ir::Ir> {
    match command_options.gc {
        Gc::OwnRc => {
            // TODO: implement
            debug!("garbage collection: OwnRc");
            Ok(ir)
        }
        Gc::None => Ok(ir),
    }
}
