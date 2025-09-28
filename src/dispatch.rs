pub trait ParserDispatcher<E: std::error::Error>: clap::Parser
where
    crate::Exit: From<std::result::Result<(), E>>,
{
    fn dispatch(&self) -> Result<(), E>;
    fn dispatch_cargo(&self) -> Result<(), E> {
        Ok(self.dispatch()?)
    }
    fn run() -> Result<(), E> {
        let (args, is_cargo) = Self::args();
        if is_cargo {
            Self::dispatch_cargo(&Self::parse_from(&args))?;
        } else {
            Self::dispatch(&Self::parse_from(&args))?;
        }
        Ok(())
    }
    fn main() -> crate::Exit {
        Self::run().into()
    }
    fn args() -> (Vec<String>, bool) {
        let argv = std::env::args().map(|arg|arg.to_string()).collect::<Vec<String>>();

        (argv, false)
    }
}
pub trait SubcommandDispatcher<E: std::error::Error>: clap::Subcommand {
    fn dispatch(&self) -> Result<(), E>;
}

pub trait ArgsDispatcher<E: std::error::Error>: clap::Args {
    fn dispatch(&self) -> Result<(), E>;
}
