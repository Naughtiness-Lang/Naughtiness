use crate::{CommandOption, LogLevel, NagiCommandOption, OptionErrorKind};

pub(crate) struct LogLevelOption;

impl CommandOption for LogLevelOption {
    fn help(&self) -> &str {
        "ログの出力段階を指定します"
    }

    fn option(&self) -> &str {
        "log-level"
    }

    fn help_option_args(&self) -> Vec<&str> {
        vec!["LEVEL"]
    }

    fn parse_option_args(
        &self,
        args: &[&str],
        nagi_command_option: &mut NagiCommandOption,
    ) -> Result<(), OptionErrorKind> {
        let Some(&arg) = args.first() else {
            unreachable!();
        };

        nagi_command_option.log_level = match arg {
            "all" => LogLevel::All,
            "normal" => LogLevel::Normal,
            "detailed" => LogLevel::Detailed,
            "minimal" => LogLevel::Minimal,
            _ => return Err(OptionErrorKind::InvalidOptionArgs),
        };

        Ok(())
    }
}
