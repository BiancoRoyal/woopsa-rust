use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum WoopsaVerb {
    Meta,
    Read,
    Write,
    Invoke,
}
