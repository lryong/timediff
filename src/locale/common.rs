use humantime::DurationError;
use std::time::Duration;

pub enum Locales {
    ZHLocale,
    RULocale,
    TRLocale,
}

pub trait Formatter {
    fn format_duration(
        &self,
        overflow: bool,
        duration_str: &String,
    ) -> Result<String, DurationError>;
}

impl Formatter for Locales {
    fn format_duration(
        &self,
        _overflow: bool,
        _duration_str: &String,
    ) -> Result<String, DurationError> {
        Ok(String::new())
    }
}
