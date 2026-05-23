use prometheus::{IntCounter, Histogram, register_int_counter, register_histogram};

pub struct Metrics {
    pub payment_initiated: IntCounter,
    pub payment_success: IntCounter,
    pub payment_failed: IntCounter,
    pub payment_latency: Histogram,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            payment_initiated: register_int_counter!(
                "payment_initiated_total",
                "Total number of initiated payments"
            ).unwrap(),

            payment_success: register_int_counter!(
                "payment_success_total",
                "Total successful payments"
            ).unwrap(),

            payment_failed: register_int_counter!(
                "payment_failed_total",
                "Total failed payments"
            ).unwrap(),

            payment_latency: register_histogram!(
                "payment_latency_seconds",
                "Histogram of payment processing latency in seconds"
            ).unwrap(),
        }
    }
}