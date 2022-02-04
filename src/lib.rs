//!  Rust library for printing human readable, relative time differences
//!
//! timediff is a Rust package for printing human readable, relative time differences.
//! Output is based on [ranges defined in the Day.js] Javascript library, and can be
//! customized if needed.
//!
//! [ranges defined in the Day.js]: https://day.js.org/docs/en/display/from-now

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod locale;
mod timediff;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
