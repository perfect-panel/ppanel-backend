// Error codes and their human-readable messages.
//
// Ported from the Go package `xerr` (errCode.go + errMsg.go). The first three
// digits identify the business area; the last three identify the specific
// error within that area.

use std::collections::HashMap;
use std::sync::LazyLock;

/// General error codes.
pub const SUCCESS: u32 = 200;
pub const ERROR: u32 = 500;

/// Database errors.
pub const DATABASE_QUERY_ERROR: u32 = 10001;
pub const DATABASE_UPDATE_ERROR: u32 = 10002;
pub const DATABASE_INSERT_ERROR: u32 = 10003;
pub const DATABASE_DELETED_ERROR: u32 = 10004;

/// User errors.
pub const USER_EXIST: u32 = 20001;
pub const USER_NOT_EXIST: u32 = 20002;
pub const USER_PASSWORD_ERROR: u32 = 20003;
pub const USER_DISABLED: u32 = 20004;
pub const INSUFFICIENT_BALANCE: u32 = 20005;
pub const STOP_REGISTER: u32 = 20006;
pub const TELEGRAM_NOT_BOUND: u32 = 20007;
pub const USER_NOT_BIND_OAUTH: u32 = 20008;
pub const INVITE_CODE_ERROR: u32 = 20009;
pub const USER_COMMISSION_NOT_ENOUGH: u32 = 20010;

/// Node errors.
pub const NODE_EXIST: u32 = 30001;
pub const NODE_NOT_EXIST: u32 = 30002;
pub const NODE_GROUP_EXIST: u32 = 30003;
pub const NODE_GROUP_NOT_EXIST: u32 = 30004;
pub const NODE_GROUP_NOT_EMPTY: u32 = 30005;

/// Request errors.
pub const INVALID_PARAMS: u32 = 400;
pub const TOO_MANY_REQUESTS: u32 = 401;
pub const ERROR_TOKEN_EMPTY: u32 = 40002;
pub const ERROR_TOKEN_INVALID: u32 = 40003;
pub const ERROR_TOKEN_EXPIRE: u32 = 40004;
pub const INVALID_ACCESS: u32 = 40005;
pub const INVALID_CIPHERTEXT: u32 = 40006;
pub const SECRET_IS_EMPTY: u32 = 40007;

/// Coupon errors.
pub const COUPON_NOT_EXIST: u32 = 50001;
pub const COUPON_ALREADY_USED: u32 = 50002;
pub const COUPON_NOT_APPLICABLE: u32 = 50003;
pub const COUPON_INSUFFICIENT_USAGE: u32 = 50004;
pub const COUPON_EXPIRED: u32 = 50005;
pub const COUPON_DISABLED: u32 = 50006;

/// Subscribe errors.
pub const SUBSCRIBE_EXPIRED: u32 = 60001;
pub const SUBSCRIBE_NOT_AVAILABLE: u32 = 60002;
pub const USER_SUBSCRIBE_EXIST: u32 = 60003;
pub const SUBSCRIBE_IS_USED_ERROR: u32 = 60004;
pub const SINGLE_SUBSCRIBE_MODE_EXCEEDS_LIMIT: u32 = 60005;
pub const SUBSCRIBE_QUOTA_LIMIT: u32 = 60006;
pub const SUBSCRIBE_OUT_OF_STOCK: u32 = 60007;

/// Order errors.
pub const ORDER_NOT_EXIST: u32 = 61001;
pub const PAYMENT_METHOD_NOT_FOUND: u32 = 61002;
pub const ORDER_STATUS_ERROR: u32 = 61003;
pub const INSUFFICIENT_OF_PERIOD: u32 = 61004;
pub const EXIST_AVAILABLE_TRAFFIC: u32 = 61005;

/// Auth errors.
pub const VERIFY_CODE_ERROR: u32 = 70001;

/// Equipment errors.
pub const QUEUE_ENQUEUE_ERROR: u32 = 80001;

/// System errors.
pub const DEBUG_MODE_ERROR: u32 = 90001;
pub const SEND_SMS_ERROR: u32 = 90002;
pub const SMS_NOT_ENABLED: u32 = 90003;
pub const EMAIL_NOT_ENABLED: u32 = 90004;
pub const GET_AUTHENTICATOR_ERROR: u32 = 90005;
pub const AUTHENTICATOR_NOT_SUPPORTED_ERROR: u32 = 90006;
pub const TELEPHONE_AREA_CODE_IS_EMPTY: u32 = 90007;
pub const TODAY_SEND_COUNT_EXCEEDS_LIMIT: u32 = 90015;
pub const PASSWORD_IS_EMPTY: u32 = 90008;
pub const AREA_CODE_IS_EMPTY: u32 = 90009;
pub const PASSWORD_OR_VERIFICATION_CODE_REQUIRED: u32 = 90010;
pub const EMAIL_EXIST: u32 = 90011;
pub const TELEPHONE_EXIST: u32 = 90012;
pub const DEVICE_EXIST: u32 = 90013;
pub const TELEPHONE_ERROR: u32 = 90014;
pub const DEVICE_NOT_EXIST: u32 = 90017;
pub const USERID_NOT_MATCH: u32 = 90018;

/// Mapping of error code -> default message.
static MESSAGES: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    // General
    m.insert(SUCCESS, "Success");
    m.insert(ERROR, "Internal Server Error");
    // Request / parameter
    m.insert(TOO_MANY_REQUESTS, "Too Many Requests");
    m.insert(INVALID_PARAMS, "Param Error");
    m.insert(ERROR_TOKEN_EMPTY, "User token is empty");
    m.insert(ERROR_TOKEN_INVALID, "User token is invalid");
    m.insert(ERROR_TOKEN_EXPIRE, "User token is expired");
    m.insert(SECRET_IS_EMPTY, "Secret is empty");
    m.insert(INVALID_ACCESS, "Invalid access");
    m.insert(INVALID_CIPHERTEXT, "Invalid ciphertext");
    // Database
    m.insert(DATABASE_QUERY_ERROR, "Database query error");
    m.insert(DATABASE_UPDATE_ERROR, "Database update error");
    m.insert(DATABASE_INSERT_ERROR, "Database insert error");
    m.insert(DATABASE_DELETED_ERROR, "Database deleted error");
    // User
    m.insert(USER_EXIST, "User already exists");
    m.insert(USER_NOT_EXIST, "User does not exist");
    m.insert(USER_PASSWORD_ERROR, "User password error");
    m.insert(USER_DISABLED, "User disabled");
    m.insert(INSUFFICIENT_BALANCE, "Insufficient balance");
    m.insert(STOP_REGISTER, "Stop register");
    m.insert(TELEGRAM_NOT_BOUND, "Telegram not bound ");
    m.insert(USER_NOT_BIND_OAUTH, "User not bind oauth method");
    m.insert(INVITE_CODE_ERROR, "Invite code error");
    // Node
    m.insert(NODE_EXIST, "Node already exists");
    m.insert(NODE_NOT_EXIST, "Node does not exist");
    m.insert(NODE_GROUP_EXIST, "Node group already exists");
    m.insert(NODE_GROUP_NOT_EXIST, "Node group does not exist");
    m.insert(NODE_GROUP_NOT_EMPTY, "Node group is not empty");
    // Coupon
    m.insert(COUPON_NOT_EXIST, "Coupon does not exist");
    m.insert(COUPON_ALREADY_USED, "Coupon has already been used");
    m.insert(COUPON_NOT_APPLICABLE, "Coupon does not match the order or conditions");
    m.insert(COUPON_INSUFFICIENT_USAGE, "Coupon has insufficient remaining uses");
    m.insert(COUPON_EXPIRED, "Coupon is expired");
    m.insert(COUPON_DISABLED, "Coupon is disabled");
    // Subscribe
    m.insert(SUBSCRIBE_EXPIRED, "Subscribe is expired");
    m.insert(SUBSCRIBE_NOT_AVAILABLE, "Subscribe is not available");
    m.insert(USER_SUBSCRIBE_EXIST, "User has subscription");
    m.insert(SUBSCRIBE_IS_USED_ERROR, "Subscribe is used");
    m.insert(
        SINGLE_SUBSCRIBE_MODE_EXCEEDS_LIMIT,
        "Single subscribe mode exceeds limit",
    );
    m.insert(SUBSCRIBE_QUOTA_LIMIT, "Subscribe quota limit");
    m.insert(SUBSCRIBE_OUT_OF_STOCK, "Subscribe out of stock");
    // Auth
    m.insert(VERIFY_CODE_ERROR, "Verify code error");
    // Equipment
    m.insert(QUEUE_ENQUEUE_ERROR, " Queue enqueue error");
    // System
    m.insert(DEBUG_MODE_ERROR, "Debug mode is enabled");
    m.insert(GET_AUTHENTICATOR_ERROR, "Unsupported login method");
    m.insert(
        AUTHENTICATOR_NOT_SUPPORTED_ERROR,
        "The authenticator does not support this method",
    );
    m.insert(TELEPHONE_AREA_CODE_IS_EMPTY, "Telephone area code is empty");
    m.insert(
        TODAY_SEND_COUNT_EXCEEDS_LIMIT,
        "This account has reached the limit of sending times today",
    );
    m.insert(SMS_NOT_ENABLED, "Telephone login is not enabled");
    m.insert(EMAIL_NOT_ENABLED, "Email function is not enabled yet");
    m.insert(
        PASSWORD_OR_VERIFICATION_CODE_REQUIRED,
        "Password or verification code required",
    );
    m.insert(EMAIL_EXIST, "Email already exists");
    m.insert(TELEPHONE_EXIST, "Telephone already exists");
    m.insert(DEVICE_EXIST, "device exists");
    m.insert(PASSWORD_IS_EMPTY, "password is empty");
    m.insert(TELEPHONE_ERROR, "telephone number error");
    m.insert(DEVICE_NOT_EXIST, "Device does not exist");
    m.insert(USERID_NOT_MATCH, "Userid not match");
    // Order
    m.insert(ORDER_NOT_EXIST, "Order does not exist");
    m.insert(PAYMENT_METHOD_NOT_FOUND, "Payment method not found");
    m.insert(ORDER_STATUS_ERROR, "Order status error");
    m.insert(INSUFFICIENT_OF_PERIOD, "Insufficient number of period");
    m
});

/// Returns the default message for an error code.
///
/// Mirrors Go `xerr.MapErrMsg`: falls back to `"Internal Server Error"` when the
/// code is unknown.
pub fn map_err_msg(err_code: u32) -> &'static str {
    MESSAGES
        .get(&err_code)
        .copied()
        .unwrap_or("Internal Server Error")
}

/// Returns `true` when the code is a known, registered error code.
///
/// Mirrors Go `xerr.IsCodeErr`.
pub fn is_code_err(err_code: u32) -> bool {
    MESSAGES.contains_key(&err_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_codes() {
        assert_eq!(map_err_msg(SUCCESS), "Success");
        assert_eq!(map_err_msg(ERROR), "Internal Server Error");
        assert_eq!(map_err_msg(INVALID_PARAMS), "Param Error");
        assert_eq!(map_err_msg(ORDER_NOT_EXIST), "Order does not exist");
    }

    #[test]
    fn unknown_code_falls_back() {
        assert_eq!(map_err_msg(999_999), "Internal Server Error");
    }

    #[test]
    fn is_code_err_recognizes_registered_codes() {
        assert!(is_code_err(SUCCESS));
        assert!(is_code_err(INVALID_PARAMS));
        assert!(!is_code_err(999_999));
    }
}
