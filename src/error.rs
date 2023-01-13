
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http parsing failed")]
    HttpParsing,

    #[error("Only GET supported")]
    HttpMethod,

    #[error("No such file")]
    NoSuchFile,

    #[error("This resource requires admin privilege")]
    Forbidden,

    #[error("Problem with file {}", 0)]
    FileError(String),

    #[error("Padding Error")]
    PaddingError,

    #[error("Error writing hex string")]
    Write,

    #[error("Error reading stream")]
    Read,
}
