use humantime::{self, DurationError};

pub fn format_duration(
    before_current_ts: bool,
    duration_str: &String,
) -> Result<String, DurationError> {
    if before_current_ts {
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x <= 44 => {
                    return Ok("несколько секунд назад".to_string());
                }
                x if x <= 89 => {
                    return Ok("минуту назад".to_string());
                }
                x if x <= 40 * 60 => {
                    let m = x as f32 / 60 as f32;
                    return Ok(format!("{:.0} минуты назад", m.ceil()));
                }
                x if x <= 44 * 60 => {
                    let m = x as f32 / 60 as f32;
                    return Ok(format!("{:.0} минут назад", m.ceil()));
                }
                x if x <= 89 * 60 => {
                    return Ok("час назад".to_string());
                }
                x if x <= 4 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} часа назад", h.ceil()));
                }
                x if x <= 20 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} часов назад", h.ceil()));
                }
                x if x <= 21 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} час назад", h.ceil()));
                }
                x if x <= 35 * 60 * 60 => {
                    return Ok("день назад".to_string());
                }
                x if x <= 4 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} дня назад", d.ceil()));
                }
                x if x <= 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} дней назад", d.ceil()));
                }
                x if x <= 45 * 24 * 60 * 60 => {
                    return Ok("месяц назад".to_string());
                }
                x if x <= 4 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} месяца назад", m));
                }
                x if x <= 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} месяцев назад", m));
                }
                x if x <= 17 * 30 * 24 * 60 * 60 => {
                    return Ok("год назад".to_string());
                }
                x if x <= 34 * 30 * 24 * 60 * 60 => {
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
            x if x <= 44 => {
                return Ok("через несколько секунд".to_string());
            }
            x if x <= 89 => {
                return Ok("через минуту".to_string());
            }
            x if x <= 4 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("через {:.0} минуты", m.ceil()));
            }
            x if x <= 21 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("через {:.0} минут", m.ceil()));
            }
            x if x <= 44 * 60 => {
                let m = x as f32 / 60 as f32;
                return Ok(format!("через {:.0} минуты", m.ceil()));
            }
            x if x <= 89 * 60 => {
                return Ok("через час".to_string());
            }
            x if x <= 4 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} часа", h.ceil()));
            }
            x if x <= 20 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} часов", h.ceil()));
            }
            x if x <= 21 * 60 * 60 => {
                let h = x as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} час", h.ceil()));
            }
            x if x <= 35 * 60 * 60 => {
                return Ok("через день".to_string());
            }
            x if x <= 4 * 24 * 60 * 60 => {
                let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} дня", d.ceil()));
            }
            x if x <= 25 * 24 * 60 * 60 => {
                let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} дней", d.ceil()));
            }
            x if x <= 45 * 24 * 60 * 60 => {
                return Ok("через месяц".to_string());
            }
            x if x <= 4 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} месяца", m));
            }
            x if x <= 10 * 30 * 24 * 60 * 60 => {
                let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                return Ok(format!("через {:.0} месяцев", m));
            }
            x if x <= 17 * 30 * 24 * 60 * 60 => {
                return Ok("через год".to_string());
            }
            x if x <= 50 * 30 * 24 * 60 * 60 => {
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

#[cfg(test)]
mod tests {
    use crate::TimeDiff;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_units() {
        assert_eq!(
            TimeDiff::to_diff(String::from("-10s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("несколько секунд назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("несколько секунд назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("минуту назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("минуту назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 минуты назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-91s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 минуты назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 минуты назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("10 минуты назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-44m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("44 минут назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("час назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-60m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("час назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-1h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("час назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("час назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-89m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("час назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-90m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 часа назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-2h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 часа назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("20 часов назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("21 час назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-21h30m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("день назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-22h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("день назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("день назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24h30m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("день назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-34h59m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("день назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-36h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 дня назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("10 дней назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-25d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("25 дней назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-26d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("месяц назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("месяц назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-45d2m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 месяца назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-46d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 месяца назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-80d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("3 месяца назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-9M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("9 месяцев назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("год назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-10M1m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("год назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-12M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("год назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-17M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("1 года назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-24M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("2 года назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-20y"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("20 лет назад"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("-100y"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("101 лет назад"))
        );

        assert_eq!(
            TimeDiff::to_diff(String::from("10s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через несколько секунд"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через несколько секунд"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через минуту"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через минуту"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 минуты"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 минуты"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 10 минут"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("44m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 44 минуты"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через час"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("60m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через час"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("1h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через час"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через час"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через час"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("89m10s"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 часа"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("90m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 часа"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("2h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 часа"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 20 часов"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 21 час"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("21h30m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через день"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("22h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через день"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через день"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("35h10m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 дня"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("36h"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 дня"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 10 дней"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("25d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 25 дней"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("26d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через месяц"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через месяц"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("45d1m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 месяца"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("46d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 месяца"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("80d"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 3 месяца"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("9M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 9 месяцев"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через год"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("10M1m"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через год"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("12M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через год"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("24M"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 2 года"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("20y"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 20 лет"))
        );
        assert_eq!(
            TimeDiff::to_diff(String::from("100y"))
                .locale(String::from("ru-RU"))
                .unwrap()
                .parse(),
            Ok(String::from("через 101 лет"))
        );
    }
}
