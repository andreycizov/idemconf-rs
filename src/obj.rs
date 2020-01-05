use structopt::StructOpt;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(StructOpt)]
#[structopt(version = "1.0", author = "Andrey C.")]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[structopt(short = "c", long = "config", default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: Option<String>,
    /// A level of verbosity, and can be used multiple times
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,
    #[structopt(subcommand)]
    subcmd: FormatSubCommand,
}

/// A subcommand for controlling testing
#[derive(StructOpt)]
pub struct TextFormatSubCommand {
    /// Print debug info
    #[structopt(short = "d")]
    debug: bool,
    #[structopt(subcommand)]
    subcmd: TextFormatSubCommandChoice,
}

#[derive(StructOpt)]
pub enum FormatSubCommand {
    /// A help message for the Test subcommand
    #[structopt(name = "text", version = "1.3", author = "Andrey C.")]
    Text(TextFormatSubCommand),
}

#[derive(StructOpt)]
pub struct IdTextFormat {
    id: String
}

arg_enum! {
    #[derive(Debug)]
    pub enum MultiMatchStrategy {
        error,
        first,
        last,
    }
}

/// Safely and idempotently inject a piece of text given from stdin into file
#[derive(StructOpt)]
pub struct InjectTextFormatSubCommand {
    #[structopt(flatten)]
    id: IdTextFormat,
    filename: String,
    #[structopt(long = "no-create")]
    no_create: bool,
    #[structopt(
        long = "multi-match",
        possible_values = &MultiMatchStrategy::variants(),
        case_insensitive = true, default_value = "error",
        help = "what should happen if we encounter several matches to the bounds"
    )]
    multi_match: MultiMatchStrategy,
}

/// Generate a unique ID used for injection correlation
#[derive(StructOpt)]
pub struct IdGenerateTextFormatSubCommand {
}

#[derive(StructOpt)]
/// Generate the bounds with the given ID. Similar to `inject` command given "" as stdin and an empty file.
pub struct IdBoundsTextFormatSubCommand {
    #[structopt(flatten)]
    id: IdTextFormat,
}

#[derive(StructOpt)]
pub enum TextFormatSubCommandChoice {
    #[structopt(name = "inject", version = "1.3", author = "Andrey C.")]
    Inject(InjectTextFormatSubCommand),
    #[structopt(name = "id", version = "1.3", author = "Andrey C.")]
    IdGenerate(IdGenerateTextFormatSubCommand),
    #[structopt(name = "bounds", version = "1.3", author = "Andrey C.")]
    IdBounds(IdBoundsTextFormatSubCommand),
}
