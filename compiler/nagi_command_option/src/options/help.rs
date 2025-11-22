use crate::*;

pub(crate) struct HelpOption;

impl HelpOption {
    pub fn help(options: &HashMap<String, Box<dyn CommandOption + 'static>>) -> String {
        let mut help_text = vec![];
        let options: Vec<&dyn CommandOption> = options.values().map(|f| &**f).collect();
        let short_option_length = Self::short_option_length(&options);
        let option_length = Self::option_length(&options);

        for value in options.iter() {
            help_text.push(Self::make_help_message(
                *value,
                short_option_length,
                option_length,
            ))
        }

        help_text.join("\n")
    }

    pub fn make_help_message(
        command_option: &dyn CommandOption,
        short_offset: usize,
        option_offset: usize,
    ) -> String {
        let option = format!(
            "{}{}{}",
            Self::option_prefix(),
            command_option.option(),
            Self::option_postfix()
        );
        let short_option = command_option
            .shorten_option()
            .map(|s| {
                format!(
                    "{}{}{}",
                    Self::short_option_prefix(),
                    s,
                    Self::short_option_postfix()
                )
            })
            .unwrap_or_default();
        let option_args: String = command_option
            .help_option_args()
            .iter()
            .map(|s| {
                format!(
                    "{}{}{}",
                    Self::option_args_prefix(),
                    s,
                    Self::option_args_postfix()
                )
            })
            .collect();

        let help_text = command_option.help();
        let short_option_length = short_option.len();
        let option_length = option.len() + option_args.len();
        let help_text_offset = " ".repeat(option_offset.saturating_sub(option_length));
        let option_offset = " ".repeat(short_offset.saturating_sub(short_option_length));
        format!("{short_option}{option_offset}{option}{option_args}{help_text_offset}{help_text}")
    }

    fn short_option_length(options: &[&dyn CommandOption]) -> usize {
        let adjust = Self::short_option_prefix().len() + Self::short_option_postfix().len();
        options
            .iter()
            .filter_map(|v| v.shorten_option().map(|s| s.len() + adjust))
            .max()
            .unwrap_or(0)
    }

    fn option_length(options: &[&dyn CommandOption]) -> usize {
        let option_adjust = Self::option_prefix().len() + Self::option_postfix().len();
        let option_args_adjust =
            Self::option_args_prefix().len() + Self::option_args_postfix().len();
        options
            .iter()
            .map(|v| {
                let option_length = v.option().len() + option_adjust;
                let args_length = v
                    .help_option_args()
                    .iter()
                    .map(|s| s.len() + option_args_adjust)
                    .sum::<usize>();
                option_length + args_length
            })
            .max()
            .unwrap_or(0)
    }

    fn short_option_prefix() -> &'static str {
        "-"
    }

    fn short_option_postfix() -> &'static str {
        ", "
    }

    fn option_prefix() -> &'static str {
        "--"
    }

    fn option_postfix() -> &'static str {
        " "
    }

    fn option_args_prefix() -> &'static str {
        "<"
    }

    fn option_args_postfix() -> &'static str {
        "> "
    }
}

impl CommandOption for HelpOption {
    fn help(&self) -> &str {
        "ヘルプを表示します"
    }

    fn option(&self) -> &str {
        "help"
    }

    fn shorten_option(&self) -> Option<&str> {
        Some("h")
    }

    fn parse_option_args(&self, _: &[String], _: &mut NagiCommandOption) -> bool {
        false
    }
}
