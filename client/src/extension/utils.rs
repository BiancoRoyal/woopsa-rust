extern crate woopsa;

use crate::woopsa::protocol::value::WoopsaValue;

struct WoopsaClientUtils {}

impl WoopsaClientUtils {
    fn is_same_value(left: WoopsaValue, right: WoopsaValue) -> bool {
        return left.eq(right);
    }
}
