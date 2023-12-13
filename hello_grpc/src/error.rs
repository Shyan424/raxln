
#[derive(Debug)]
pub enum Error {
    UriError,
    ConnectFail(String),
    RequestDataError(String),
    IDontKnow(String)
}