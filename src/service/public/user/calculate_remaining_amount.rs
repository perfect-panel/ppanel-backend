//! Helper for computing the deductible remainder of a subscription at
//! unsubscribe time. Ported from the Go logic in
//! `server/internal/logic/public/user/calculateRemainingAmount.go`.

pub fn calculate_remaining_amount(
    total_amount: i64,
    start_time: i64,
    expire_time: i64,
    now: i64,
) -> i64 {
    if expire_time <= start_time {
        return 0;
    }
    let total_secs = expire_time - start_time;
    let remaining_secs = (expire_time - now).max(0);
    let ratio = (remaining_secs as f64) / (total_secs as f64);
    let amount = (total_amount as f64) * ratio;
    if amount < 0.0 {
        0
    } else {
        amount.round() as i64
    }
}
