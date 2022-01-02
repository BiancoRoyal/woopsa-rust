use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum WoopsaErrorType {
    WoopsaException,
    WoopsaNotFoundException,
    WoopsaInvalidOperationException,
    WoopsaNotificationsLostException,
    WoopsaInvalidSubscriptionChannelException,
}
