//! Day.js-inspired relative time for Rust with locale-aware UI copy.
//!
//! `timediff` formats duration strings and `std::time::Duration` values into
//! human-friendly output such as `a minute ago`, `in 2 hours`, and `几秒前`.
//! The crate stays intentionally small so services can reuse product-facing
//! relative-time wording without adopting a broader date-time toolkit.
//!
//! [ranges defined in the Day.js]: https://day.js.org/docs/en/display/from-now

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod locale;
mod timediff;

pub use self::timediff::{Error as TimeDiffError, TimeDiff};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
