#[derive(Debug)]
pub struct CommandOptionError {
    pub(crate) kind: OptionErrorKind,
    pub message: String,
}

#[derive(Debug)]
pub(crate) enum OptionErrorKind {
    HelpRequested,
    UnknownOption,
    InvalidOptionArgs,
}
