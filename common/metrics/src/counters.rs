// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::ServiceMetrics;
use once_cell::sync::Lazy;
use prometheus::IntCounter;

pub static SVC_COUNTERS: Lazy<ServiceMetrics> = Lazy::new(ServiceMetrics::new_and_registered);

// Admission Control counters
pub static COUNTER_ADMISSION_CONTROL_CANNOT_SEND_REPLY: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "COUNTER_ADMISSION_CONTROL_CANNOT_SEND_REPLY",
        "Number of errors related to send reply in Admission Control"
    )
    .unwrap()
});

// Client counters
pub static COUNTER_CLIENT_ERRORS: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "COUNTER_CLIENT_ERRORS",
        "Number of errors encountered by Client"
    )
    .unwrap()
});
