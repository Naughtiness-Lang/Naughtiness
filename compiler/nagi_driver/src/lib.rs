use std::{fs, path::PathBuf, time::Instant};

use errors::CompileError;
use nagi_command_option::*;

mod errors;

const SUPPORT_EXTENSIONS: [&str; 2] = ["nagi", "spec"];

pub fn driver() {
    let start_time = Instant::now();

    match run_compiler() {
        Ok(_) => println!("{:?}", start_time.elapsed()),
        Err(e) => println!("error: {e}"),
    }
}

fn run_compiler() -> Result<(), CompileError> {
    let args = NagiCommandOption::new()?;
    let files = get_files(args.target_dir.clone(), true)?;

    println!("{files:#?}");
    for file in files {
        let source_code = fs::read_to_string(file)?;
        let token_list = nagi_lexer::tokenize(&source_code)?;
        //let ast = nagi_parser::parse(&token_list)?;
    }

    Ok(())
}

fn get_files(path: PathBuf, recursed: bool) -> Result<Vec<PathBuf>, CompileError> {
    let mut files = vec![];
    let mut stack = vec![path];
    while let Some(target) = stack.pop() {
        // 拡張子チェック
        if let Some(extension) = target.extension() {
            let Some(extention) = extension.to_str() else {
                continue;
            };

            // パスの正規化
            let normalize_path = target.canonicalize()?;

            // 対応している拡張子であれば追加
            if SUPPORT_EXTENSIONS.contains(&extention) {
                files.push(normalize_path);
            }
        } else if let Ok(dirs) = target.read_dir() {
            if !recursed {
                continue;
            }

            // ディレクトリの場合は再帰的に探す
            for dir_entry in dirs {
                stack.push(dir_entry?.path());
            }
        }
    }

    Ok(files)
}
