use std::{fs, path::PathBuf, time::Instant};

use errors::CompileError;
use nagi_command_option::*;
use walkdir::WalkDir;

mod errors;

const SOURCE_FILE_EXTENSION: &str = "nagi";

pub fn driver() {
    let start_time = Instant::now();

    match run_compiler() {
        Ok(_) => println!("{:?}", start_time.elapsed()),
        Err(e) => eprintln!("{e}"),
    }
}

fn run_compiler() -> Result<(), CompileError> {
    let args = NagiCommandOption::new()?;
    let files = get_source_files(args.target_dir, SOURCE_FILE_EXTENSION, true)?;

    for file in files {
        let source_code = fs::read_to_string(file)?;

        let token_list = nagi_lexer::tokenize(&source_code)?;
        let ast = nagi_parser::parse_nagi_code(&token_list)?;
    }

    Ok(())
}

fn get_source_files(
    path: PathBuf,
    target_extension: &str,
    recursive: bool,
) -> Result<Vec<PathBuf>, CompileError> {
    let walker = WalkDir::new(path).max_depth(if recursive { usize::MAX } else { 1 });
    let files = walker
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some(target_extension))
        .map(|e| e.into_path())
        .collect();

    Ok(files)
}
