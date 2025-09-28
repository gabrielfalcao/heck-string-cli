use clap::Parser;

use crate::dispatch::ParserDispatcher;
use crate::{Error, Exit, Result, ToCase};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "heck-string-cli command-line"
)]
pub struct Cli {
    #[arg(
        long,
        help = "target case-conversion type"
    )]
    to: ToCase,

    #[arg(help = "the input string to be converted")]
    input: Vec<String>,
}
impl Cli {
    pub fn input(&self) -> String {
        let sep = std::env::var("IPS").map(|var|var.to_string()).unwrap_or_else(|_|" ".to_string());
        self.input.join(&sep)
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let output = self.to.convert(self.input());
        println!("{output}");
        Ok(())
    }
}
