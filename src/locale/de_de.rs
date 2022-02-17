use humantime::{self, DurationError};

pub fn format_duration(
    before_current_ts: bool,
    duration_str: &String,
) -> Result<String, DurationError> {
    if before_current_ts {
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x <= 44 => {
                    return Ok("vor ein paar Sekunden".to_string());
                }
                x if x <= 89 => {
                    return Ok("vor einer Minute".to_string());
                }
                x if x <= 44 * 60 => {
                    let m = x as f32 / 60_f32;
                    return Ok(format!("vor {:.0} Minuten", m.ceil()));
                }
                x if x <= 89 * 60 => {
                    return Ok("vor einer Stunde".to_string());
                }
                x if x <= 21 * 60 * 60 => {
                    let h = x as f32 / 60_f32 / 60_f32;
                    return Ok(format!("vor {:.0} Stunden", h.ceil()));
                }
                x if x <= 35 * 60 * 60 => {
                    return Ok("vor einem Tag".to_string());
                }
                x if x <= 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("vor {:.0} Tagen", d.ceil()));
                }
                x if x <= 45 * 24 * 60 * 60 => {
                    return Ok("vor einem Monat".to_string());
                }
                x if x <= 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("vor {:.0} Monaten", m));
                }
                x if x <= 17 * 30 * 24 * 60 * 60 => {
                    return Ok("vor einem Jahr".to_string());
                }
                _ => {
                    let y = v.as_secs_f32() / 12_f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("vor {:.0} Jahren", y));
                }
            },
            Err(e) => return Err(e),
        }
    }
    match humantime::parse_duration(duration_str) {
        Ok(v) => match v.as_secs() {
            x if x <= 44 => Ok("in ein paar Sekunden".to_string()),
            x if x <= 89 => Ok("in einer Minute".to_string()),
            x if x <= 44 * 60 => {
                let m = x as f32 / 60_f32;
                return Ok(format!("in {:.0} Minuten", m.ceil()));
            }
            x if x <= 89 * 60 => Ok("in einer Stunde".to_string()),
            x if x <= 21 * 60 * 60 => {
                let h = x as f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} Stunden", h.ceil()));
            }
            x if x <= 35 * 60 * 60 => Ok("in einem Tag".to_string()),
            x if x <= 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} Tagen", d.ceil()));
            }
            x if x <= 45 * 24 * 60 * 60 => Ok("in einem Monat".to_string()),
            x if x <= 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} Monaten", m));
            }
            x if x <= 17 * 30 * 24 * 60 * 60 => Ok("in einem Jahr".to_string()),
            _ => {
                let y = v.as_secs_f32() / 12_f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("in {:.0} Jahren", y));
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
            TimeDiff::to_diff(String::from("-10s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor ein paar Sekunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor ein paar Sekunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einer Minute"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einer Minute"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-91s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 10 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 44 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-60m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-1h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 20 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 21 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h30m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-22h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h30m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-34h59m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-36h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Tagen"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 10 Tagen"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-25d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 25 Tagen"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-26d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Monat"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Monat"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d2m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-46d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 3 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-9M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 9 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Jahr"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M1m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Jahr"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-12M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor einem Jahr"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-17M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 1 Jahren"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 2 Jahren"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20y"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 20 Jahren"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-100y"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("vor 101 Jahren"))
        );

        assert_eq!(
            TimeDiff::to_diff(String::from("10s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in ein paar Sekunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in ein paar Sekunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einer Minute"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einer Minute"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 10 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 44 Minuten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("60m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("1h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einer Stunde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m10s"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 20 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 21 Stunden"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h30m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("22h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Tag"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("35h10m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Tagen"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("36h"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Tagen"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 10 Tagen"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("25d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 25 Tagen"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("26d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Monat"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Monat"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d1m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("46d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80d"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 3 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("9M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 9 Monaten"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Jahr"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M1m"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Jahr"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("12M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in einem Jahr"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24M"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 2 Jahren"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20y"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 20 Jahren"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("100y"))
                .locale(String::from("de-DE"))
                .unwrap()
                .parse(),
            Ok(String::from("in 101 Jahren"))
        );
    }
}
