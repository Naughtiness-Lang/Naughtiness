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
        args: &[String],
        nagi_command_option: &mut NagiCommandOption,
    ) -> bool {
        if args.len() != 1 {
            return false;
        }

        true
    }
}
