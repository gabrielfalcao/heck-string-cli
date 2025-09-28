#[doc(hidden)]
pub(crate) mod errors;
#[doc(inline)]
pub use errors::{Error, Exit, Result};
pub mod dispatch;
#[doc(hidden)]
pub use dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)]
pub(crate) mod cli;
#[doc(hidden)]
pub use cli::Cli;

#[doc(hidden)]
pub(crate) mod convert;
#[doc(inline)]
pub use convert::ToCase;
