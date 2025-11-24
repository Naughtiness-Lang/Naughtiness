use std::{fs, path::PathBuf, time::Instant};

use errors::CompileError;
use nagi_command_option::*;

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

    println!("{files:#?}");
    for file in files {
        let source_code = fs::read_to_string(file)?;

        // それぞれ専用のエラー型を作成してからコメントアウトを外す
        //let token_list = nagi_lexer::tokenize(&source_code)?;
        //let ast = nagi_parser::parse(&token_list)?;
    }

    Ok(())
}

fn get_source_files(
    path: PathBuf,
    target_extension: &str,
    recursive: bool,
) -> Result<Vec<PathBuf>, CompileError> {
    let mut files = vec![];
    let mut stack = vec![];

    // ディレクトリの場合はそのディレクトリ配下をstackに積む
    if let Ok(dirs) = path.read_dir() {
        for dir_entry in dirs {
            stack.push(dir_entry?.path());
        }
    } else {
        stack.push(path);
    }

    while let Some(target) = stack.pop() {
        // 拡張子チェック
        if let Some(extension) = target.extension() {
            let Some(extension) = extension.to_str() else {
                continue;
            };

            // 対応している拡張子であれば追加
            if target_extension == extension {
                let normalize_path = target.canonicalize()?; // パスの正規化
                files.push(normalize_path);
            }
        } else if let Ok(dirs) = target.read_dir() {
            if !recursive {
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
