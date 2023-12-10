
#[derive(Debug)]
pub enum Error {
    UriError,
    ConnectFail,
    RequestDataError(String),
    IDontKnow(String)
}