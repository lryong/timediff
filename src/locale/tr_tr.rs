use crate::locale::common;
use humantime::{self, DurationError};
pub enum TRLocale {}

impl common::Formatter for TRLocale {
    fn format_duration(
        &self,
        overflow: bool,
        duration_str: &String,
    ) -> Result<String, DurationError> {
        if overflow {
            match humantime::parse_duration(duration_str) {
                Ok(v) => match v.as_secs() {
                    x if x < 44 => {
                        return Ok("birkaç saniye önce".to_string());
                    }
                    x if x < 89 => {
                        return Ok("bir dakika önce".to_string());
                    }
                    x if x < 44 * 60 => {
                        let m = x as f32 / 60 as f32;
                        return Ok(format!("{:.0} dakika önce", m.ceil()));
                    }
                    x if x < 89 * 60 => {
                        return Ok("bir saat önce".to_string());
                    }
                    x if x < 21 * 60 * 60 => {
                        let h = x as f32 / 60 as f32 / 60 as f32;
                        return Ok(format!("{:.0} saat önce", h.ceil()));
                    }
                    x if x < 35 * 60 * 60 => {
                        return Ok("bir gün önce".to_string());
                    }
                    x if x < 25 * 24 * 60 * 60 => {
                        let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                        return Ok(format!("{:.0}  gün önce", d.ceil()));
                    }
                    x if x < 45 * 24 * 60 * 60 => {
                        return Ok("bir ay önce".to_string());
                    }
                    x if x < 10 * 30 * 24 * 60 * 60 => {
                        let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                        return Ok(format!("{:.0} ay önce", m));
                    }
                    x if x < 17 * 30 * 24 * 60 * 60 => {
                        return Ok("bir yıl önce".to_string());
                    }
                    _ => {
                        let y = v.as_secs_f32()
                            / 12 as f32
                            / 30 as f32
                            / 24 as f32
                            / 60 as f32
                            / 60 as f32;
                        return Ok(format!("{:.0} yıl önce", y));
                    }
                },
                Err(e) => return Err(e),
            }
        }
        match humantime::parse_duration(duration_str) {
            Ok(v) => match v.as_secs() {
                x if x < 44 => {
                    return Ok("birkaç saniye içinde".to_string());
                }
                x if x < 89 => {
                    return Ok("bir dakika içinde".to_string());
                }
                x if x < 44 * 60 => {
                    let m = x as f32 / 60 as f32;
                    return Ok(format!("{:.0} dakika içinde", m.ceil()));
                }
                x if x < 89 * 60 => {
                    return Ok("bir saat içinde".to_string());
                }
                x if x < 21 * 60 * 60 => {
                    let h = x as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} saat içinde", h.ceil()));
                }
                x if x < 35 * 60 * 60 => {
                    return Ok("bir gün içinde".to_string());
                }
                x if x < 25 * 24 * 60 * 60 => {
                    let d = x as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} gün içinde", d.ceil()));
                }
                x if x < 45 * 24 * 60 * 60 => {
                    return Ok("bir ay içinde".to_string());
                }
                x if x < 10 * 30 * 24 * 60 * 60 => {
                    let m = x as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} ay içinde", m));
                }
                x if x < 17 * 30 * 24 * 60 * 60 => {
                    return Ok("bir yıl içinde".to_string());
                }
                _ => {
                    let y =
                        v.as_secs_f32() / 12 as f32 / 30 as f32 / 24 as f32 / 60 as f32 / 60 as f32;
                    return Ok(format!("{:.0} yıl içinde", y));
                }
            },
            Err(e) => return Err(e),
        }
    }
}
