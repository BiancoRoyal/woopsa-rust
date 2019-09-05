use crate::protocol::value::WoopsaValue;
use crate::protocol::value_type::WoopsaValueType;

struct WoopsaServerUtils {}

impl WoopsaServerUtils {
    fn is_same_value(left: WoopsaValue, right: WoopsaValue) -> bool {
        return left.eq(right);
    }
}
