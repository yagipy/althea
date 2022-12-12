use alc_command_option::CommandOptions;
use alc_diagnostic::{emit, Diagnostic, FileId, Files, Label, Result, Span};
use log::debug;
use std::{env, fs::File, io::Read, process, time::Instant};

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

pub fn drive() {
    let start_time = Instant::now();
    let command_options = alc_command_option::parse_args();
    init_env_logger(command_options.debug);
    debug!("{:#?}", command_options);
    let mut files = Files::new();
    let exit_code = match run_compiler(&command_options, &mut files) {
        Ok(_) => EXIT_SUCCESS,
        Err(diagnostic) => {
            emit(&files, &diagnostic);
            EXIT_FAILURE
        }
    };
    debug!(
        "compiler exited. code {}, time {}ms",
        exit_code,
        start_time.elapsed().as_millis()
    );
    process::exit(exit_code);
}

fn init_env_logger(is_debug: bool) {
    if is_debug {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
}

fn run_compiler(command_options: &CommandOptions, files: &mut Files) -> Result<()> {
    let file_id = open_file(command_options, files)?;
    let ast = alc_parser::parse(command_options, files, file_id)?;
    let (ir, ty_sess) = alc_ast_lowering::lower(command_options, file_id, &ast)?;
    // let _ty_env = alc_type_checker::check(command_options, file_id, &ty_sess, &ir)?;
    alc_codegen_llvm::generate(command_options, file_id, &ty_sess, &ir)
}

fn open_file(command_options: &CommandOptions, files: &mut Files) -> Result<FileId> {
    match File::open(&command_options.src) {
        Ok(mut file) => {
            let mut src = String::new();
            match file.read_to_string(&mut src) {
                Ok(_) => {
                    debug!("{}", src);
                    Ok(files.add(command_options.src.to_str().unwrap().to_owned(), src.as_str()))
                }
                Err(err) => {
                    let file_id = files.add(command_options.src_file_name().to_owned(), "");
                    Err(Box::from(Diagnostic::new_error(
                        "failed to read source file",
                        Label::new(file_id, Span::dummy(), err.to_string()),
                    )))
                }
            }
        }
        Err(err) => {
            let file_id = files.add(command_options.src_file_name().to_owned(), "");
            Err(Box::from(Diagnostic::new_error(
                "failed to open source file",
                Label::new(file_id, Span::dummy(), err.to_string()),
            )))
        }
    }
}
