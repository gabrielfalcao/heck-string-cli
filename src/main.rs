use clap::Parser;
use heck_string_cli::dispatch::ParserDispatcher;
use heck_string_cli::{Cli, Exit};

fn main() -> Exit {
    Cli::main()
}
