use options::{
    emit::EmitOption, help::HelpOption, log_level::LogLevelOption, target::TargetOption,
};
use std::{collections::HashMap, env, fmt::Debug, iter::from_fn, path::PathBuf};

mod options;

#[derive(Debug)]
pub struct NagiCommandOption {
    pub target_dir: PathBuf,
    pub output_file_name: PathBuf,
    pub log_level: LogLevel,
    pub output_file_type: OutputFileType,
}

impl NagiCommandOption {
    pub fn new() -> Result<Self, String> {
        parse_command_option()
    }

    fn default() -> Self {
        Self {
            target_dir: PathBuf::from("./src"),
            output_file_name: PathBuf::from("a"),
            log_level: LogLevel::Normal,
            output_file_type: OutputFileType::Binary,
        }
    }
}

fn parse_command_option() -> Result<NagiCommandOption, String> {
    let options = HashMap::from([
        make_option(HelpOption {}),
        make_option(LogLevelOption {}),
        make_option(EmitOption {}),
        make_option(TargetOption {}),
    ]);
    let mut short_options = HashMap::new();
    for value in options.values() {
        if let Some(short_option) = value.shorten_option() {
            short_options.insert(short_option, value);
        }
    }

    let mut nagi_command_option = NagiCommandOption::default();
    let mut args = env::args().skip(1).peekable();

    while let Some(arg) = args.next() {
        let Some(arg) = arg.strip_prefix("-") else {
            return Err(HelpOption::help(&options));
        };

        let option_args: Vec<String> =
            from_fn(|| args.next_if(|arg| !arg.starts_with("-"))).collect();

        if let Some(option) = short_options.get(arg) {
            if !option.parse_option_args(&option_args, &mut nagi_command_option) {
                return Err(HelpOption::help(&options));
            }
        } else if let Some(option) = arg.strip_prefix("-") {
            let Some(option) = options.get(option) else {
                return Err(HelpOption::help(&options));
            };

            if !option.parse_option_args(&option_args, &mut nagi_command_option) {
                return Err(HelpOption::help(&options));
            }
        } else {
            return Err(HelpOption::help(&options));
        }
    }

    Ok(nagi_command_option)
}

fn make_option(option: impl CommandOption + 'static) -> (String, Box<dyn CommandOption>) {
    (option.option().to_string(), Box::new(option))
}

#[derive(Debug)]
pub enum LogLevel {
    Minimal,
    Normal,
    Detailed,
    All,
}

#[derive(Debug)]
pub enum OutputFileType {
    Binary,
    Object,
    AST,
}

pub(crate) trait CommandOption {
    // --hogehoge
    fn option(&self) -> &str;

    // -H
    fn shorten_option(&self) -> Option<&str> {
        None
    }

    fn help(&self) -> &str;

    fn help_option_args(&self) -> Vec<&str> {
        vec![]
    }

    //fn usage(&self) -> &str;

    fn parse_option_args(
        &self,
        args: &[String],
        nagi_command_option: &mut NagiCommandOption,
    ) -> bool;
}
