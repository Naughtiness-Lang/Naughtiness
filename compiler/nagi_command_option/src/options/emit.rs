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
        args: &[&str],
        nagi_command_option: &mut NagiCommandOption,
    ) -> Result<(), CommandOptionError> {
        let Some(&arg) = args.first() else {
            unreachable!();
        };

        nagi_command_option.output_file_type = match arg {
            "bin" => OutputFileType::Binary,
            "obj" => OutputFileType::Object,
            "ast" => OutputFileType::AST,
            _ => return Err(CommandOptionError::InvalidOptionArgs),
        };

        Ok(())
    }
}
