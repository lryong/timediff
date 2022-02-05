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
                    return Ok(format!("{:.0}分钟前", m.ceil()));
                }
                x if x <= 89 * 60 => {
                    return Ok("1小时前".to_string());
                }
                x if x <= 21 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}小时前", h.ceil()));
                }
                x if x <= 35 * 60 * 60 => {
                    return Ok("1天前".to_string());
                }
                x if x <= 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}天前", d.ceil()));
                }
                x if x <= 45 * 24 * 60 * 60 => {
                    return Ok("1个月前".to_string());
                }
                x if x <= 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}个月前", m));
                }
                x if x <= 17 * 30 * 24 * 60 * 60 => {
                    return Ok("1年前".to_string());
                }
                _ => {
                    let y =
                        v.as_secs_f32() / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}年前", y));
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
                return Ok(format!("{:.0}分钟后", m.ceil()));
            }
            x if x <= 89 * 60 => {
                return Ok("1小时后".to_string());
            }
            x if x <= 21 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0}小时后", h.ceil()));
            }
            x if x <= 35 * 60 * 60 => {
                return Ok("1天后".to_string());
            }
            x if x <= 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0}天后", d.ceil()));
            }
            x if x <= 45 * 24 * 60 * 60 => {
                return Ok("1个月后".to_string());
            }
            x if x <= 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0}个月后", m));
            }
            x if x <= 17 * 30 * 24 * 60 * 60 => {
                return Ok("1年后".to_string());
            }
            _ => {
                let y = v.as_secs_f32() / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0}年后", y));
            }
        },
        Err(e) => return Err(e),
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
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("几秒前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("几秒前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-91s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("10分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("44分钟前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-60m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-1h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("20小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("21小时前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h30m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-22h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h30m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-34h59m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-36h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("10天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-25d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("25天前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-26d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d2m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-46d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("3个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-9M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("9个月前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M1m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-12M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-17M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20y"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("20年前"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-100y"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("101年前"))
        );

        assert_eq!(
            TimeDiff::to_diff(String::from("10s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("几秒后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("几秒后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("10分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("44分钟后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("60m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("1h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m10s"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("20小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("21小时后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h30m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("22h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("35h10m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("36h"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("10天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("25d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("25天后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("26d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d1m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("46d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80d"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("3个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("9M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("9个月后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M1m"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("12M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("1年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24M"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("2年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20y"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("20年后"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("100y"))
                .locale(String::from("zh-CN"))
                .unwrap()
                .parse(),
            Ok(String::from("101年后"))
        );
    }
}
