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
        args: &[&str],
        nagi_command_option: &mut NagiCommandOption,
    ) -> Result<(), CommandOptionError> {
        let path = PathBuf::from(&args[0]);

        nagi_command_option.target_dir = path;

        Ok(())
    }
}
