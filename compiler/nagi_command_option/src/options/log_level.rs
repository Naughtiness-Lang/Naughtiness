use crate::*;

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
    ) -> Result<(), CommandOptionError> {
        nagi_command_option.log_level = match args[0] {
            "all" => LogLevel::All,
            "normal" => LogLevel::Normal,
            "detailed" => LogLevel::Detailed,
            "minimal" => LogLevel::Minimal,
            _ => return Err(CommandOptionError::InvalidOptionArgs),
        };

        Ok(())
    }
}
