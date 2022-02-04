use std::time::SystemTime;

/// TimeDiffOption is an option used to customize a call to TimeDiff
fn time_diff_option(time_diff_opts: *mut TimeDiffOption) {}

struct TimeDiffOption {
    // start is the time to calculate the time from.
    start: SystemTime,
    // locale is the locale string used by time_diff function.
}
