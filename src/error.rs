#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("couldn't figure out where your program data directory is")]
    MissingLocalDataDirectory,
    #[error("stored json is invalid")]
    DeserializeData,
    #[error("couldn't convert program's data to valid json")]
    SerializeData,
}
