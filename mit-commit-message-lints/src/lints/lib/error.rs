#[derive(Debug, PartialEq)]
pub enum Error {
    FailedToParseTrailer { string: String },
}
