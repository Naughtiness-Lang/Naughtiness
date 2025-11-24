use errors::{CommandOptionError, OptionErrorKind};
use options::{
    emit::EmitOption, help::HelpOption, log_level::LogLevelOption, target::TargetOption,
};
use std::{collections::HashMap, env, fmt::Debug, iter::from_fn, path::PathBuf};

pub mod errors;
mod options;

#[derive(Debug)]
pub struct NagiCommandOption {
    pub target_dir: PathBuf,
    pub output_file_name: PathBuf,
    pub log_level: LogLevel,
    pub output_file_type: OutputFileType,
}

impl NagiCommandOption {
    pub fn new() -> Result<Self, CommandOptionError> {
        let args: Vec<String> = env::args().skip(1).collect();
        Self::from(&args)
    }

    pub fn from(args: &[String]) -> Result<Self, CommandOptionError> {
        parse_command_option(args)
    }
}

impl Default for NagiCommandOption {
    fn default() -> Self {
        Self {
            target_dir: PathBuf::from("./src"),
            output_file_name: PathBuf::from("a"),
            log_level: LogLevel::Normal,
            output_file_type: OutputFileType::Binary,
        }
    }
}

fn parse_command_option(args: &[String]) -> Result<NagiCommandOption, CommandOptionError> {
    let options = HashMap::from([
        make_option(HelpOption {}),
        make_option(LogLevelOption {}),
        make_option(EmitOption {}),
        make_option(TargetOption {}),
    ]);
    let options_list: Vec<&dyn CommandOption> = options.values().map(|c| &**c).collect();
    let short_options: HashMap<&str, &Box<dyn CommandOption>> = HashMap::from_iter(
        options
            .iter()
            .filter_map(|(_, v)| Some((v.shorten_option()?, v))),
    );

    let mut args = args.iter().peekable();
    let mut nagi_command_option = NagiCommandOption::default();
    while let Some(arg) = args.next() {
        if !arg.starts_with("-") {
            return Err(CommandOptionError {
                kind: OptionErrorKind::UnknownOption,
                message: HelpOption::help(&options_list),
            });
        }

        let option_args: Vec<&str> = from_fn(|| {
            args.next_if(|arg| !arg.starts_with("-"))
                .map(|f| f.as_str())
        })
        .collect();

        let option = if let Some(option) = arg.strip_prefix("--") {
            options.get(option).map(|o| &**o)
        } else if let Some(option) = arg.strip_prefix("-") {
            short_options.get(option).map(|o| &***o)
        } else {
            unreachable!()
        };

        let Some(option) = option else {
            return Err(CommandOptionError {
                kind: OptionErrorKind::UnknownOption,
                message: HelpOption::help(&options_list),
            });
        };

        parse_option_args(
            &mut nagi_command_option,
            option,
            &option_args,
            &options_list,
        )?;
    }

    Ok(nagi_command_option)
}

fn parse_option_args(
    nagi_command_option: &mut NagiCommandOption,
    option: &dyn CommandOption,
    args: &[&str],
    options: &[&dyn CommandOption],
) -> Result<(), CommandOptionError> {
    if option.help_option_args().len() != args.len() {
        return Err(CommandOptionError {
            kind: OptionErrorKind::InvalidOptionArgs,
            message: HelpOption::help_usage(option),
        });
    }

    match option.parse_option_args(args, nagi_command_option) {
        Ok(()) => Ok(()),
        Err(e) => {
            let message = match e {
                OptionErrorKind::HelpRequested => HelpOption::help(options),
                _ => HelpOption::help_usage(option),
            };

            Err(CommandOptionError { kind: e, message })
        }
    }
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

    fn parse_option_args(
        &self,
        args: &[&str],
        nagi_command_option: &mut NagiCommandOption,
    ) -> Result<(), OptionErrorKind>;
}
