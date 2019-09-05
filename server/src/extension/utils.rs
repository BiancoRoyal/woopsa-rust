extern crate woopsa;

use crate::woopsa::protocol::value::WoopsaValue;

struct WoopsaServerUtils {}

impl WoopsaServerUtils {
    fn is_same_value(left: WoopsaValue, right: WoopsaValue) -> bool {
        return left.eq(right);
    }
}
