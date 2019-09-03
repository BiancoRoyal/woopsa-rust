#[derive(Clone, Copy, PartialEq)]
pub enum WoopsaValueType {
    Null,
    Logical,
    Integer,
    Real,
    DateTime,
    TimeSpan,
    Text,
    WoopsaLink,
    JsonData,
    ResourceUrl,
}
