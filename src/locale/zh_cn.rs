use humantime::{self, DurationError};

pub fn format_duration(overflow: bool, duration_str: &String) -> Result<String, DurationError> {
    if overflow {
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x < 44 => {
                    return Ok("1秒前".to_string());
                }
                x if x < 89 => {
                    return Ok("1分钟前".to_string());
                }
                x if x < 44 * 60 => {
                    let m = x as f32 / 60 as f32;
                    return Ok(format!("{:.0} 分钟前", m.ceil()));
                }
                x if x < 89 * 60 => {
                    return Ok("1小时前".to_string());
                }
                x if x < 21 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} 小时前", h.ceil()));
                }
                x if x < 35 * 60 * 60 => {
                    return Ok("1天前".to_string());
                }
                x if x < 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} 天前", d.ceil()));
                }
                x if x < 45 * 24 * 60 * 60 => {
                    return Ok("1个月前".to_string());
                }
                x if x < 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} 个月前", m));
                }
                x if x < 17 * 30 * 24 * 60 * 60 => {
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
            x if x < 44 => {
                return Ok("1秒后".to_string());
            }
            x if x < 89 => {
                return Ok("1分钟后".to_string());
            }
            x if x < 44 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("{:.0} 分钟后", m.ceil()));
            }
            x if x < 89 * 60 => {
                return Ok("1小时后".to_string());
            }
            x if x < 21 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0} 小时后", h.ceil()));
            }
            x if x < 35 * 60 * 60 => {
                return Ok("1天后".to_string());
            }
            x if x < 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0} 天后", d.ceil()));
            }
            x if x < 45 * 24 * 60 * 60 => {
                return Ok("1个月后".to_string());
            }
            x if x < 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("{:.0} 个月后", m));
            }
            x if x < 17 * 30 * 24 * 60 * 60 => {
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
            Ok(String::from("1秒前"))
        );
    }
}
