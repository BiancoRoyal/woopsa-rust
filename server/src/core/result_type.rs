#[derive(Clone, Copy, PartialEq)]
pub enum WoopsaRequestResultType {
    Success,
    ClientRequestError,
    InternalServerError,
    Error,
    OtherResponse
}
