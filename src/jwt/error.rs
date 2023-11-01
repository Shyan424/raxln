
#[derive(Debug, PartialEq, Eq)]
pub enum AuthError {
    CreateError,
    ValidateError,
    WithoutToken
}