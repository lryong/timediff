use humantime::{self, DurationError};

pub fn format_duration(
    before_current_ts: bool,
    duration_str: &String,
) -> Result<String, DurationError> {
    if before_current_ts {
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x < 44 => {
                    return Ok("несколько секунд назад".to_string());
                }
                x if x < 89 => {
                    return Ok("минуту назад".to_string());
                }
                x if x < 40 * 60 => {
                    let m = x as f32 / 60 as f32;
                    return Ok(format!("{:.0} минуты назад", m.ceil()));
                }
                x if x < 44 * 60 => {
                    let m = x as f32 / 60 as f32;
                    return Ok(format!("{:.0} минут назад", m.ceil()));
                }
                x if x < 89 * 60 => {
                    return Ok("час назад".to_string());
                }
                x if x < 4 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} часа назад", h.ceil()));
                }
                x if x < 20 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}  часов назад", h.ceil()));
                }
                x if x < 21 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}  час назад", h.ceil()));
                }
                x if x < 35 * 60 * 60 => {
                    return Ok("день назад".to_string());
                }
                x if x < 4 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}  дня назад", d.ceil()));
                }
                x if x < 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0}  дней назад", d.ceil()));
                }
                x if x < 45 * 24 * 60 * 60 => {
                    return Ok("месяц назад".to_string());
                }
                x if x < 4 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} месяца назад", m));
                }
                x if x < 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} месяцев назад", m));
                }
                x if x < 17 * 30 * 24 * 60 * 60 => {
                    return Ok("год назад".to_string());
                }
                x if x < 34 * 30 * 24 * 60 * 60 => {
                    let y = x as f32 / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} года назад", y));
                }
                _ => {
                    let y =
                        v.as_secs_f32() / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} лет назад", y));
                }
            },
            Err(e) => return Err(e),
        }
    }
    match humantime::parse_duration(duration_str) {
        Ok(v) => match v.as_secs() {
            x if x < 44 => {
                return Ok("через несколько секунд".to_string());
            }
            x if x < 89 => {
                return Ok("через минуту".to_string());
            }
            x if x < 4 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("через {:.0} минуты", m.ceil()));
            }
            x if x < 21 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("через {:.0} минут", m.ceil()));
            }
            x if x < 44 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("через {:.0} минуты", m.ceil()));
            }
            x if x < 89 * 60 => {
                return Ok("через час".to_string());
            }
            x if x < 4 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} часа", h.ceil()));
            }
            x if x < 20 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} часов", h.ceil()));
            }
            x if x < 21 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} час", h.ceil()));
            }
            x if x < 35 * 60 * 60 => {
                return Ok("через день".to_string());
            }
            x if x < 4 * 24 * 60 * 60 => {
                let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} дня", d.ceil()));
            }
            x if x < 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} дней", d.ceil()));
            }
            x if x < 45 * 24 * 60 * 60 => {
                return Ok("через месяц".to_string());
            }
            x if x < 4 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} месяца", m));
            }
            x if x < 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} месяцев", m));
            }
            x if x < 17 * 30 * 24 * 60 * 60 => {
                return Ok("через год".to_string());
            }
            x if x < 50 * 30 * 24 * 60 * 60 => {
                let y = x as f32 / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} года", y));
            }
            _ => {
                let y = v.as_secs_f32() / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} лет", y));
            }
        },
        Err(e) => return Err(e),
    }
}
