use crate::*;

pub(crate) struct EmitOption;

impl CommandOption for EmitOption {
    fn help(&self) -> &str {
        "出力するファイルを指定します"
    }

    fn help_option_args(&self) -> Vec<&str> {
        vec!["TYPE"]
    }

    fn option(&self) -> &str {
        "emit"
    }

    fn parse_option_args(
        &self,
        args: &[String],
        nagi_command_option: &mut NagiCommandOption,
    ) -> bool {
        false
    }
}
