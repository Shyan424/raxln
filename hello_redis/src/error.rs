
#[derive(Debug)]
pub enum Error {
    ClientBuildError(String),
    ConnectError(String),
    QueryError(String),
}