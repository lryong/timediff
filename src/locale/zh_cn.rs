use humantime::{self, DurationError};

pub fn format_duration(
    before_current_ts: bool,
    duration_str: &String,
) -> Result<String, DurationError> {
    if before_current_ts {
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x <= 44 => {
                    return Ok("几秒前".to_string());
                }
                x if x <= 89 => {
                    return Ok("1分钟前".to_string());
                }
                x if x <= 44 * 60 => {
                    let m = x as f32 / 60 as f32;
                    return Ok(format!("{:.0} 分钟前", m.ceil()));
                }
                x if x <= 89 * 60 => {
                    return Ok("1小时前".to_string());
                }
                x if x <= 21 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} 小时前", h.ceil()));
                }
                x if x <= 35 * 60 * 60 => {
                    return Ok("1天前".to_string());
                }
                x if x <= 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} 天前", d.ceil()));
                }
                x if x <= 45 * 24 * 60 * 60 => {
                    return Ok("1个月前".to_string());
                }
                x if x <= 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} 个月前", m));
                }
                x if x <= 17 * 30 * 24 * 60 * 60 => {
                    return Ok("1年前".to_string());
                }
                _ => {
                    let y =
                        v.as_secs_f32() / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} 年前", y));
                }
            },
            Err(e) => return Err(e),
        }
    }
    match humantime::parse_duration(duration_str) {
        Ok(v) => match v.as_secs() {
            x if x <= 44 => {
                return Ok("几秒后".to_string());
            }
            x if x <= 89 => {
                return Ok("1分钟后".to_string());
            }
            x if x <= 44 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("{:.0} 分钟后", m.ceil()));
            }
            x if x <= 89 * 60 => {
                return Ok("1小时后".to_string());
            }
            x if x <= 21 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0} 小时后", h.ceil()));
            }
            x if x <= 35 * 60 * 60 => {
                return Ok("1天后".to_string());
            }
            x if x <= 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0} 天后", d.ceil()));
            }
            x if x <= 45 * 24 * 60 * 60 => {
                return Ok("1个月后".to_string());
            }
            x if x <= 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0} 个月后", m));
            }
            x if x <= 17 * 30 * 24 * 60 * 60 => {
                return Ok("1年后".to_string());
            }
            _ => {
                let y = v.as_secs_f32() / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0} 年后", y));
            }
        },
        Err(e) => return Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::TimeDiff;

    // use super::*;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_units() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s")).parse(),
            Ok(String::from("几秒前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44s")).parse(),
            Ok(String::from("几秒前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45s")).parse(),
            Ok(String::from("1分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89s")).parse(), // t: 1分钟前
            Ok(String::from("1分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90s")).parse(),
            Ok(String::from("2 分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-91s")).parse(),
            Ok(String::from("2 分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2m")).parse(),
            Ok(String::from("2 分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10m")).parse(),
            Ok(String::from("10 分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44m")).parse(),
            Ok(String::from("44 分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45m")).parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-60m")).parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-1h")).parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80m")).parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89m")).parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90m")).parse(),
            Ok(String::from("2 小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2h")).parse(),
            Ok(String::from("2 小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20h")).parse(),
            Ok(String::from("20 小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h")).parse(),
            Ok(String::from("21 小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h30m")).parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-22h")).parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h")).parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h30m")).parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-34h59m")).parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-36h")).parse(),
            Ok(String::from("2 天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10d")).parse(),
            Ok(String::from("10 天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-25d")).parse(),
            Ok(String::from("25 天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-26d")).parse(),
            Ok(String::from("1个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d")).parse(),
            Ok(String::from("1个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d2m")).parse(),
            Ok(String::from("2 个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-46d")).parse(),
            Ok(String::from("2 个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80d")).parse(),
            Ok(String::from("3 个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-9M")).parse(),
            Ok(String::from("9 个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M")).parse(),
            Ok(String::from("1年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M1m")).parse(),
            Ok(String::from("1年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-12M")).parse(),
            Ok(String::from("1年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-17M")).parse(),
            Ok(String::from("1 年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24M")).parse(),
            Ok(String::from("2 年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20y")).parse(),
            Ok(String::from("20 年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-100y")).parse(),
            Ok(String::from("101 年前"))
        );

        assert_eq!(
            TimeDiff::to_diff(String::from("10s")).parse(),
            Ok(String::from("几秒后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44s")).parse(),
            Ok(String::from("几秒后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45s")).parse(),
            Ok(String::from("1分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89s")).parse(),
            Ok(String::from("1分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90s")).parse(),
            Ok(String::from("2 分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2m")).parse(),
            Ok(String::from("2 分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10m")).parse(),
            Ok(String::from("10 分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44m")).parse(),
            Ok(String::from("44 分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45m")).parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("60m")).parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("1h")).parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80m")).parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m")).parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m10s")).parse(),
            Ok(String::from("2 小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90m")).parse(),
            Ok(String::from("2 小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2h")).parse(),
            Ok(String::from("2 小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20h")).parse(),
            Ok(String::from("20 小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h")).parse(),
            Ok(String::from("21 小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h30m")).parse(),
            Ok(String::from("1天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("22h")).parse(),
            Ok(String::from("1天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24h")).parse(),
            Ok(String::from("1天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("35h10m")).parse(),
            Ok(String::from("2 天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("36h")).parse(),
            Ok(String::from("2 天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10d")).parse(),
            Ok(String::from("10 天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("25d")).parse(),
            Ok(String::from("25 天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("26d")).parse(),
            Ok(String::from("1个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d")).parse(),
            Ok(String::from("1个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d1m")).parse(),
            Ok(String::from("2 个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("46d")).parse(),
            Ok(String::from("2 个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80d")).parse(),
            Ok(String::from("3 个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("9M")).parse(),
            Ok(String::from("9 个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M")).parse(),
            Ok(String::from("1年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M1m")).parse(),
            Ok(String::from("1年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("12M")).parse(),
            Ok(String::from("1年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24M")).parse(),
            Ok(String::from("2 年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20y")).parse(),
            Ok(String::from("20 年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("100y")).parse(),
            Ok(String::from("101 年后"))
        );
    }
}
