pub fn run_latency(block_time_a : i64, block_time_b: i64) -> i64 {
    block_time_a - block_time_b 
}
pub fn time_difference(time_a: i64, time_b: i64) -> i64 {
    (time_a * 1000) - time_b
}