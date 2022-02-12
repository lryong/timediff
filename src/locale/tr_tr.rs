use humantime::{self, DurationError};

pub fn format_duration(
    before_current_ts: bool,
    duration_str: &String,
) -> Result<String, DurationError> {
    if before_current_ts {
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x <= 44 => {
                    return Ok("birkaç saniye önce".to_string());
                }
                x if x <= 89 => {
                    return Ok("bir dakika önce".to_string());
                }
                x if x <= 44 * 60 => {
                    let m = x as f32 / 60_f32;
                    return Ok(format!("{:.0} dakika önce", m.ceil()));
                }
                x if x <= 89 * 60 => {
                    return Ok("bir saat önce".to_string());
                }
                x if x <= 21 * 60 * 60 => {
                    let h = x as f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} saat önce", h.ceil()));
                }
                x if x <= 35 * 60 * 60 => {
                    return Ok("bir gün önce".to_string());
                }
                x if x <= 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} gün önce", d.ceil()));
                }
                x if x <= 45 * 24 * 60 * 60 => {
                    return Ok("bir ay önce".to_string());
                }
                x if x <= 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} ay önce", m));
                }
                x if x <= 17 * 30 * 24 * 60 * 60 => {
                    return Ok("bir yıl önce".to_string());
                }
                _ => {
                    let y =
                        v.as_secs_f32() / 12_f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                    return Ok(format!("{:.0} yıl önce", y));
                }
            },
            Err(e) => return Err(e),
        }
    }
    match humantime::parse_duration(duration_str) {
        Ok(v) => match v.as_secs() {
            x if x <= 44 => {
                Ok("birkaç saniye içinde".to_string())
            }
            x if x <= 89 => {
                Ok("bir dakika içinde".to_string())
            }
            x if x <= 44 * 60 => {
                let m = x as f32 / 60_f32;
                return Ok(format!("{:.0} dakika içinde", m.ceil()));
            }
            x if x <= 89 * 60 => {
                Ok("bir saat içinde".to_string())
            }
            x if x <= 21 * 60 * 60 => {
                let h = x as f32 / 60_f32 / 60_f32;
                return Ok(format!("{:.0} saat içinde", h.ceil()));
            }
            x if x <= 35 * 60 * 60 => {
                Ok("bir gün içinde".to_string())
            }
            x if x <= 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("{:.0} gün içinde", d.ceil()));
            }
            x if x <= 45 * 24 * 60 * 60 => {
                Ok("bir ay içinde".to_string())
            }
            x if x <= 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("{:.0} ay içinde", m));
            }
            x if x <= 17 * 30 * 24 * 60 * 60 => {
                Ok("bir yıl içinde".to_string())
            }
            _ => {
                let y = v.as_secs_f32() / 12_f32 / 30_f32 / 24_f32 / 60_f32 / 60_f32;
                return Ok(format!("{:.0} yıl içinde", y));
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
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("birkaç saniye önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("birkaç saniye önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir dakika önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir dakika önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 dakika önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-91s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 dakika önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 dakika önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("10 dakika önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("44 dakika önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-60m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-1h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("20 saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("21 saat önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h30m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-22h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h30m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-34h59m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-36h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("10 gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-25d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("25 gün önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-26d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir ay önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir ay önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d2m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 ay önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-46d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 ay önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("3 ay önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-9M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("9 ay önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir yıl önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M1m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir yıl önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-12M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir yıl önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-17M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("1 yıl önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 yıl önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20y"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("20 yıl önce"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-100y"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("101 yıl önce"))
        );

        assert_eq!(
            TimeDiff::to_diff(String::from("10s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("birkaç saniye içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("birkaç saniye içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir dakika içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir dakika içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 dakika içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 dakika içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("10 dakika içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("44 dakika içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("60m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("1h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m10s"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("20 saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("21 saat içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h30m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("22h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir gün içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("35h10m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 gün içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("36h"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 gün içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("10 gün içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("25d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("25 gün içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("26d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir ay içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir ay içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d1m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 ay içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("46d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 ay içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80d"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("3 ay içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("9M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("9 ay içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir yıl içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M1m"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir yıl içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("12M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("bir yıl içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24M"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("2 yıl içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20y"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("20 yıl içinde"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("100y"))
                .locale(String::from("tr-TR"))
                .unwrap()
                .parse(),
            Ok(String::from("101 yıl içinde"))
        );
    }
}
