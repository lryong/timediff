use std::time::Duration;

// Formatter is a function for formatting an output string for given time duration
/*
pub enum Formatter {
    Some(fn(String) -> String),
    None,
}

// Formatters is a collection of formatter functions mapped to the closest time duration
pub type Formatters = phf::Map<&'static str, Formatter>;
// pub type Formatters = phf::Map<&'static str, Option<&'static str>>;

static LOCALES_FORMATTERS: phf::Map<&'static str, &'static Formatters> = phf_map! {};
*/

// Locale represents an IETF BCP 47 formatted language tag.
#[derive(Debug, Clone)]
pub struct Locale(String);

impl Locale {
    // split splits the locale into language and territory components.
    fn split(self) -> (String, String) {
        // remove the encoding, if present
        let locales: Vec<&str> = self.0.split('.').collect();
        // split at the underscore
        let res: Vec<&str> = locales[0].split('_').collect();

        let language = res[0];
        let mut territory = "";
        if res.len() > 1 {
            territory = res[1];
        }

        (language.to_string(), territory.to_string())
    }
}

pub fn duration_to_string(d: Duration) -> String {
    d.as_secs_f64().to_string()
}

pub fn string_to_duration(s: String) -> Duration {
    Duration::new(s.parse::<u64>().unwrap(), 0)
}

pub trait Formatter {
    fn format_duration(&self, overflow: bool, duration_str: String) -> String;
}
