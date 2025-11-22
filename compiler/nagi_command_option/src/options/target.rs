use crate::*;

pub(crate) struct TargetOption;

impl CommandOption for TargetOption {
    fn help(&self) -> &str {
        "コンパイルする対象を指定します"
    }

    fn option(&self) -> &str {
        "target"
    }

    fn help_option_args(&self) -> Vec<&str> {
        vec!["TARGET"]
    }

    fn parse_option_args(
        &self,
        args: &[String],
        nagi_command_option: &mut NagiCommandOption,
    ) -> bool {
        true
    }
}
