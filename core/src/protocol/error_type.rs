#[derive(Clone, Copy, PartialEq)]
pub enum WoopsaErrorType {
    WoopsaException,
    WoopsaNotFoundException,
    WoopsaInvalidOperationException,
    WoopsaNotificationsLostException,
    WoopsaInvalidSubscriptionChannelException,
}
