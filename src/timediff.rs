use crate::locale::common::Formatter;
use crate::locale::*;
use humantime::{self, DurationError};

/// Error loading locale
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// Invalid locate or locale currently not supported
    NotFoundLocale(String),
}

#[derive(Debug)]
pub struct TimeDiff {
    // locale is the locale string used by time_diff function.
    locale: String,

    overflow: bool,

    to: String,
}

impl TimeDiff {
    pub fn to_diff(to: String) -> Self {
        TimeDiff {
            locale: String::from("zh-CN"),
            overflow: false,
            to,
        }
    }

    pub fn locale(&mut self, l: String) -> Result<(), Error> {
        match l.as_str() {
            "zh-CN" | "ru-RU" | "tr-TR" => self.locale = l,
            _ => return Err(Error::NotFoundLocale(l)),
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<String, DurationError> {
        let c = self.to.find("-").unwrap_or(self.to.len());
        if c == 0 {
            self.overflow = true;
            self.to = self.to[1..].to_string();
        }

        match self.locale.as_str() {
            "zh-CN" => return zh_cn::format_duration(self.overflow, &self.to),
            "tr-TR" => return tr_tr::format_duration(self.overflow, &self.to),
            "ru-RU" => return ru_ru::format_duration(self.overflow, &self.to),

            _ => return zh_cn::format_duration(self.overflow, &self.to),
        }
        // self.locale.format(self.overflow, &self.to)
    }
}
