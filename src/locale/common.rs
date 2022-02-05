use humantime::DurationError;

/*
#[derive(Debug)]
pub enum Locales {
    ZHLocale,
    RULocale,
    TRLocale,
}
*/

pub trait Formatter {
    fn format_duration(
        &self,
        overflow: bool,
        duration_str: &String,
    ) -> Result<String, DurationError>;
}

/*
impl Locales {
    pub fn format(&self, overflow: bool, duration_str: &String) -> Result<String, DurationError> {
        match self {
            &ZHLocale => return ZHLocale.format_duration(overflow, duration_str),
            &RULocale => return RULocale.format_duration(overflow, duration_str),
            &TRLocale => return TRLocale.format_duration(overflow, duration_str),

            _ => Ok(String::from("")),
        }
        // self.format_duration(overflow, duration_str)
        // Ok(String::new())
    }
}
*/
