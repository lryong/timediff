use humantime::{self, DurationError};

pub fn format_duration(
    before_current_ts: bool,
    duration_str: &String,
) -> Result<String, DurationError> {
    if before_current_ts {
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x <= 44 => {
                    return Ok("a few seconds ago".to_string());
                }
                x if x <= 89 => {
                    return Ok("a minute ago".to_string());
                }
                x if x <= 44 * 60 => {
                    let m = x as f32 / 60_f32;
                    return Ok(format!("{:.0} minutes ago", m.ceil()));
                }
                x if x <= 89 * 60 => {
                    return Ok("an hour ago".to_string());
                }
                x if x <= 21 * 60 * 60 => {
                    let h = x as f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} hours ago", h.ceil()));
                }
                x if x <= 35 * 60 * 60 => {
                    return Ok("a day ago".to_string());
                }
                x if x <= 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} days ago", d.ceil()));
                }
                x if x <= 45 * 24 * 60 * 60 => {
                    return Ok("a month ago".to_string());
                }
                x if x <= 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} months ago", m));
                }
                x if x <= 17 * 30 * 24 * 60 * 60 => {
                    return Ok("a year ago".to_string());
                }
                _ => {
                    let y =
                        v.as_secs_f32() / 12_f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} years ago", y));
                }
            },
            Err(e) => return Err(e),
        }
    }
    match humantime::parse_duration(duration_str) {
        Ok(v) => match v.as_secs() {
            x if x <= 44 => {
                Ok("in a few seconds".to_string())
            }
            x if x <= 89 => {
                Ok("in a minute".to_string())
            }
            x if x <= 44 * 60 => {
                let m = x as f32 / 60_f32;
                return Ok(format!("in {:.0} minutes", m.ceil()));
            }
            x if x <= 89 * 60 => {
                Ok("in an hour".to_string())
            }
            x if x <= 21 * 60 * 60 => {
                let h = x as f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} hours", h.ceil()));
            }
            x if x <= 35 * 60 * 60 => {
                Ok("in a day".to_string())
            }
            x if x <= 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} days", d.ceil()));
            }
            x if x <= 45 * 24 * 60 * 60 => {
                Ok("in a month".to_string())
            }
            x if x <= 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} months", m));
            }
            x if x <= 17 * 30 * 24 * 60 * 60 => {
                Ok("in a year".to_string())
            }
            _ => {
                let y = v.as_secs_f32() / 12_f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} years", y));
            }
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::TimeDiff;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_units() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s")).parse(),
            Ok(String::from("a few seconds ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44s")).parse(),
            Ok(String::from("a few seconds ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45s")).parse(),
            Ok(String::from("a minute ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89s")).parse(), // t: 1分钟前
            Ok(String::from("a minute ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90s")).parse(),
            Ok(String::from("2 minutes ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-91s")).parse(),
            Ok(String::from("2 minutes ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2m")).parse(),
            Ok(String::from("2 minutes ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10m")).parse(),
            Ok(String::from("10 minutes ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44m")).parse(),
            Ok(String::from("44 minutes ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45m")).parse(),
            Ok(String::from("an hour ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-60m")).parse(),
            Ok(String::from("an hour ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-1h")).parse(),
            Ok(String::from("an hour ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80m")).parse(),
            Ok(String::from("an hour ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89m")).parse(),
            Ok(String::from("an hour ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90m")).parse(),
            Ok(String::from("2 hours ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2h")).parse(),
            Ok(String::from("2 hours ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20h")).parse(),
            Ok(String::from("20 hours ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h")).parse(),
            Ok(String::from("21 hours ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h30m")).parse(),
            Ok(String::from("a day ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-22h")).parse(),
            Ok(String::from("a day ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h")).parse(),
            Ok(String::from("a day ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h30m")).parse(),
            Ok(String::from("a day ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-34h59m")).parse(),
            Ok(String::from("a day ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-36h")).parse(),
            Ok(String::from("2 days ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10d")).parse(),
            Ok(String::from("10 days ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-25d")).parse(),
            Ok(String::from("25 days ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-26d")).parse(),
            Ok(String::from("a month ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d")).parse(),
            Ok(String::from("a month ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d2m")).parse(),
            Ok(String::from("2 months ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-46d")).parse(),
            Ok(String::from("2 months ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80d")).parse(),
            Ok(String::from("3 months ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-9M")).parse(),
            Ok(String::from("9 months ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M")).parse(),
            Ok(String::from("a year ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M1m")).parse(),
            Ok(String::from("a year ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-12M")).parse(),
            Ok(String::from("a year ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-17M")).parse(),
            Ok(String::from("1 years ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24M")).parse(),
            Ok(String::from("2 years ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20y")).parse(),
            Ok(String::from("20 years ago"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-100y")).parse(),
            Ok(String::from("101 years ago"))
        );

        assert_eq!(
            TimeDiff::to_diff(String::from("10s")).parse(),
            Ok(String::from("in a few seconds"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44s")).parse(),
            Ok(String::from("in a few seconds"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45s")).parse(),
            Ok(String::from("in a minute"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89s")).parse(),
            Ok(String::from("in a minute"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90s")).parse(),
            Ok(String::from("in 2 minutes"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2m")).parse(),
            Ok(String::from("in 2 minutes"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10m")).parse(),
            Ok(String::from("in 10 minutes"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44m")).parse(),
            Ok(String::from("in 44 minutes"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45m")).parse(),
            Ok(String::from("in an hour"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("60m")).parse(),
            Ok(String::from("in an hour"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("1h")).parse(),
            Ok(String::from("in an hour"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80m")).parse(),
            Ok(String::from("in an hour"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m")).parse(),
            Ok(String::from("in an hour"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m10s")).parse(),
            Ok(String::from("in 2 hours"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90m")).parse(),
            Ok(String::from("in 2 hours"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2h")).parse(),
            Ok(String::from("in 2 hours"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20h")).parse(),
            Ok(String::from("in 20 hours"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h")).parse(),
            Ok(String::from("in 21 hours"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h30m")).parse(),
            Ok(String::from("in a day"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("22h")).parse(),
            Ok(String::from("in a day"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24h")).parse(),
            Ok(String::from("in a day"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("35h10m")).parse(),
            Ok(String::from("in 2 days"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("36h")).parse(),
            Ok(String::from("in 2 days"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10d")).parse(),
            Ok(String::from("in 10 days"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("25d")).parse(),
            Ok(String::from("in 25 days"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("26d")).parse(),
            Ok(String::from("in a month"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d")).parse(),
            Ok(String::from("in a month"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d1m")).parse(),
            Ok(String::from("in 2 months"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("46d")).parse(),
            Ok(String::from("in 2 months"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80d")).parse(),
            Ok(String::from("in 3 months"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("9M")).parse(),
            Ok(String::from("in 9 months"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M")).parse(),
            Ok(String::from("in a year"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M1m")).parse(),
            Ok(String::from("in a year"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("12M")).parse(),
            Ok(String::from("in a year"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24M")).parse(),
            Ok(String::from("in 2 years"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20y")).parse(),
            Ok(String::from("in 20 years"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("100y")).parse(),
            Ok(String::from("in 101 years"))
        );
    }
}
