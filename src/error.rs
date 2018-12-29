use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    Writer(fmt::Error),
    MissingTerminal(String),
    UnknownBlock(String),
    NoSuchVariable(String),
    ShouldBeInteger(String),
    NoAssignmentInWith,
    TooManyAssignmentsInWith,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Writer(_) => write!(f, "Error writing to output"),
            Error::MissingTerminal(terminal) => write!(f, "A {:?} is missing", terminal),
            Error::UnknownBlock(name) => write!(f, "Unknown block {:?}", name),
            Error::NoSuchVariable(name) => write!(f, "No variable called {:?} in scope", name),
            Error::ShouldBeInteger(name) => write!(f, "Expected an integer, but got {:?} instead", name),
            Error::NoAssignmentInWith => write!(f, "Expected an equal sign in with block"),
            Error::TooManyAssignmentsInWith => write!(f, "Expected only one equal sign in with block"),
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
