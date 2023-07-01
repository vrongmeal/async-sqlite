#[derive(Debug)]
pub enum Error {
    Closed,
    Rusqlite(rusqlite::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Rusqlite(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Closed => write!(f, "connection to sqlite database closed"),
            Error::Rusqlite(err) => err.fmt(f),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error::Rusqlite(value)
    }
}

impl<T> From<crossbeam::channel::SendError<T>> for Error {
    fn from(_value: crossbeam::channel::SendError<T>) -> Self {
        Error::Closed
    }
}

impl From<futures_channel::oneshot::Canceled> for Error {
    fn from(_value: futures_channel::oneshot::Canceled) -> Self {
        Error::Closed
    }
}
