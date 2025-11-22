use std::{io::Error, path::PathBuf, time::Instant};

use nagi_command_option::*;
use nagi_lexer::*;
use nagi_parser::*;

const SUPPORT_EXTENSIONS: [&str; 1] = ["nagi"];

pub fn driver() {
    let start_time = Instant::now();
    let option = match NagiCommandOption::new() {
        Ok(option) => option,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    compile(&option);

    println!("{:?}", start_time.elapsed());
}

fn compile(command_option: &NagiCommandOption) {
    let files = get_files(command_option.target_dir.clone(), true).unwrap();

    println!("{files:#?}");
}

fn get_files(path: PathBuf, recursed: bool) -> Result<Vec<PathBuf>, Error> {
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
