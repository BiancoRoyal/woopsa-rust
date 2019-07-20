#[derive(Clone, Copy)]
pub enum WoopsaErrorType {
    WoopsaException,
    WoopsaNotFoundException,
    WoopsaInvalidOperationException,
    WoopsaNotificationsLostException,
    WoopsaInvalidSubscriptionChannelException,
}
