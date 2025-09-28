use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Error {
    IOError(String),
    RuntimeError(String),
    HeckError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Error::IOError(e) => e.to_string(),
                Error::RuntimeError(e) => e.to_string(),
                Error::HeckError(e) => e.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::RuntimeError(_) => "RuntimeError",
            Error::HeckError(_) => "HeckError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}
// impl From<iocore::Error> for Error {
//     fn from(e: iocore::Error) -> Self {
//         Error::IOError(e.to_string())
//     }
// }
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Exit {
    Success,
    Error(Error),
}
impl std::process::Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        match &self {
            Exit::Success => std::process::ExitCode::from(0),
            Exit::Error(error) => {
                eprintln!("{}", error);
                std::process::ExitCode::from(1)
            },
        }
    }
}
impl<T> From<std::result::Result<T, Error>> for Exit {
    fn from(result: std::result::Result<T, Error>) -> Exit {
        match result {
            Ok(_) => Exit::Success,
            Err(e) => Exit::Error(e),
        }
    }
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name.strip_suffix("::f").unwrap();
        name
    }};
}
#[macro_export]
macro_rules! traceback {
    ($variant:ident, $error:expr ) => {{
        let name = $crate::function_name!();
        $crate::Error::$variant(format!("{} [{}:[{}:{}]]\n", $error, name, file!(), line!()))
    }};
    ($variant:ident, $format:literal, $arg:expr  ) => {{
        $crate::traceback!($variant, format!($format, $arg))
    }};
    ($variant:ident, $format:literal, $( $arg:expr ),* ) => {{
        $crate::traceback!($variant, format!($format, $($arg,)*))
    }};
}
