use crate::locale::*;
use humantime::{self, DurationError};
use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

/// Error loading locale
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// Invalid locate or locale currently not supported
    NotFoundLocale(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFoundLocale(input) => write!(f, "locale `{}` not found", input),
        }
    }
}

impl StdError for Error {}

#[derive(Debug, PartialEq)]
pub struct TimeDiff {
    // locale is the locale string used by time_diff function.
    locale: String,

    before_current_ts: bool,

    to: String,
}

impl TimeDiff {
    pub fn to_diff(to: String) -> Self {
        TimeDiff {
            locale: String::from("en-US"),
            before_current_ts: false,
            to,
        }
    }

    pub fn to_diff_duration(to: Duration) -> Self {
        TimeDiff {
            locale: String::from("en-US"),
            before_current_ts: false,
            to: humantime::format_duration(to).to_string(),
        }
    }

    pub fn locale(&mut self, l: String) -> Result<&mut Self, Error> {
        match l.as_str() {
            "zh-CN" | "ru-RU" | "tr-TR" | "en-US" => self.locale = l,
            _ => return Err(Error::NotFoundLocale(l)),
        }

        Ok(self)
    }

    pub fn parse(&mut self) -> Result<String, DurationError> {
        let c = self.to.find("-").unwrap_or(self.to.len());
        if c == 0 {
            self.before_current_ts = true;
            self.to = self.to[1..].to_string();
        }

        match self.locale.as_str() {
            "zh-CN" => return zh_cn::format_duration(self.before_current_ts, &self.to),
            "tr-TR" => return tr_tr::format_duration(self.before_current_ts, &self.to),
            "ru-RU" => return ru_ru::format_duration(self.before_current_ts, &self.to),
            "en-US" => return en_us::format_duration(self.before_current_ts, &self.to),

            _ => return en_us::format_duration(self.before_current_ts, &self.to),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::TimeDiff;
    use std::time::Duration;

    #[test]
    fn it_works_default_locale() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s")).parse(),
            Ok(String::from("a few seconds ago"))
        )
    }

    #[test]
    fn it_works_en_us() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s"))
                .locale(String::from("en-US"))
                .unwrap()
                .parse(),
            Ok(String::from("a few seconds ago"))
        )
    }

    #[test]
    fn it_works_zh_cn() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("几秒前"))
        );
    }

    #[test]
    fn it_works_ru_ru() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("несколько секунд назад"))
        )
    }

    #[test]
    fn it_works_tr_tr() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("birkaç saniye önce"))
        )
    }

    #[test]
    fn time_diff_invalid_locale() {
        assert_eq!(
            TimeDiff::to_diff(String::from("10day")).locale(String::from("unknown")),
            Err(Error::NotFoundLocale(String::from("unknown")))
        )
    }

    #[test]
    fn time_diff_with_duration() {
        assert_eq!(
            TimeDiff::to_diff_duration(Duration::new(30 * 60, 0)).parse(),
            Ok(String::from("in 30 minutes"))
        )
    }
}
