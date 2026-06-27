# timediff

[![crates.io](https://img.shields.io/crates/v/timediff.svg)](https://crates.io/crates/timediff)
[![docs.rs](https://docs.rs/timediff/badge.svg)](https://docs.rs/timediff)
[![license](https://img.shields.io/crates/l/timediff.svg)](https://github.com/lryong/timediff/blob/main/LICENSE)

Human-friendly relative time for Rust, with Day.js-inspired thresholds.

`timediff` turns duration strings and `std::time::Duration` values into UI-friendly copy such as `a minute ago`, `in 2 hours`, and `几秒前`.
It stays intentionally small so Rust services can match product-facing relative-time wording without adopting a full date-time toolkit.

## At A Glance

- Day.js-inspired threshold behavior that feels familiar to frontend teams
- Accepts both duration strings like `-90s` and `std::time::Duration`
- Formats past and future values with natural product copy
- Includes built-in locale support for common UI-facing languages
- Keeps the dependency surface light and the API intentionally small

## Quick Start

### Install

```toml
[dependencies]
timediff = "0.2.3"
```

```bash
cargo add timediff
```

### Format a duration string

```rust
use timediff::TimeDiff;

let mut diff = TimeDiff::to_diff(String::from("-90s"));

assert_eq!(diff.parse().unwrap(), "2 minutes ago");
```

### Format `std::time::Duration`

```rust
use std::time::Duration;
use timediff::TimeDiff;

let mut diff = TimeDiff::to_diff_duration(Duration::from_secs(30 * 60));

assert_eq!(diff.parse().unwrap(), "in 30 minutes");
```

### Switch locale

```rust
use timediff::TimeDiff;

let mut diff = TimeDiff::to_diff(String::from("-10s"));
diff.locale(String::from("zh-CN")).unwrap();

assert_eq!(diff.parse().unwrap(), "几秒前");
```

## Why timediff

`timediff` is designed for the narrow but common case where you already know the duration and only need clean relative-time text.
It is a good fit when you want backend output to align more closely with Day.js-style product copy instead of exposing raw durations or adopting a broader time stack.

## Choose timediff when

- You want relative time like `2 minutes ago` or `in a month`
- You already use Day.js semantics in the frontend and want similar wording on the backend
- You need a focused crate for readable relative-time output
- You want simple locale switching without designing a formatter from scratch

## Skip timediff when

- You need a full date-time toolkit
- You need calendar-aware month or year arithmetic
- You need `chrono` or `time` integration in the public API today
- You want many formatting modes or highly configurable output right now

## Supported Locales

The current public API supports these locales:

| Locale | Example for `-10s` |
| --- | --- |
| `en-US` | `a few seconds ago` |
| `zh-CN` | `几秒前` |
| `de-DE` | `vor ein paar Sekunden` |
| `ru-RU` | `несколько секунд назад` |
| `tr-TR` | `birkaç saniye önce` |

## Why not just use `timeago`?

`timeago` is a solid crate with broader recognition.
`timediff` is the better fit when you specifically want Day.js-inspired threshold behavior, direct support for duration-string input, and a smaller product-copy-focused API.

| Need | Prefer `timediff` | Prefer `timeago` |
| --- | --- | --- |
| Day.js-inspired threshold ranges | Yes | Maybe |
| Direct human-readable duration-string input | Yes | Not the primary path |
| Small, focused relative-time crate | Yes | Yes |
| Broader ecosystem maturity and adoption | Maybe | Yes |
| More established formatting surface | Maybe | Yes |

## Current Public API

The current API is intentionally small:

| API | Purpose |
| --- | --- |
| `TimeDiff::to_diff(String)` | Accept a human-readable duration string such as `-10s` or `2h` |
| `TimeDiff::to_diff_duration(Duration)` | Accept `std::time::Duration` directly |
| `.locale(String)` | Switch locale before formatting |
| `.parse()` | Generate the final relative-time string |

```rust
use timediff::TimeDiff;

let mut diff = TimeDiff::to_diff(String::from("-10m"));
diff.locale(String::from("en-US")).unwrap();

assert_eq!(diff.parse().unwrap(), "10 minutes ago");
```

## Design Principles

- Friendly default output over excessive configuration
- Small surface area over feature sprawl
- Predictable threshold behavior over surprising edge cases
- Practical localization over one-language-only formatting
- A crate that feels good to use in product code

## Scope And Status

`timediff` is intentionally focused on one job: formatting relative time from duration inputs.

Current scope:

- Human-readable relative-time output
- `std::time::Duration` support
- Locale switching
- Threshold behavior based on [Day.js](https://day.js.org/docs/en/display/from-now)

Not in scope yet:

- Full date-time handling
- Calendar-accurate month calculations
- `chrono` integration
- `time` crate integration
- Custom threshold presets

## Reference Output

The following examples reflect the current default output behavior from the test suite.
Use them as a quick threshold reference.

```text
-864000h   100 years ago
-172800h   20 years ago
-17280h    2 years ago
-8640h     a year ago
-7200h1m   a year ago
-7200h     10 months ago
-6480h     9 months ago
-1920h     3 months ago
-1104h     2 months ago
-1080h     a month ago
-624h      a month ago
-600h      25 days ago
-240h      10 days ago
-36h       2 days ago
-24h       a day ago
-22h       a day ago
-21h30m    a day ago
-21h       21 hours ago
-20h       20 hours ago
-2h        2 hours ago
-90m       2 hours ago
-89m       an hour ago
-80m       an hour ago
-1h        an hour ago
-60m       an hour ago
-45m       an hour ago
-44m       44 minutes ago
-10m       10 minutes ago
-2m        2 minutes ago
-90s       2 minutes ago
-89s       a minute ago
-45s       a minute ago
-44s       a few seconds ago
-10s       a few seconds ago
10s        in a few seconds
44s        in a few seconds
45s        in a minute
89s        in a minute
90s        in 2 minutes
2m         in 2 minutes
10m        in 10 minutes
44m        in 44 minutes
45m        in an hour
60m        in an hour
1h         in an hour
80m        in an hour
89m        in an hour
90m        in 2 hours
2h         in 2 hours
20h        in 20 hours
21h        in 21 hours
21h30m     in a day
22h        in a day
24h        in a day
36h        in 2 days
240h       in 10 days
600h       in 25 days
624h       in a month
1080h      in a month
1104h      in 2 months
1920h      in 3 months
6480h      in 9 months
7200h      in 10 months
7200h1m    in a year
8640h      in a year
17280h     in 2 years
172800h    in 20 years
864000h    in 100 years
```

## License

This package is licensed under the MIT license.
See [LICENSE](https://github.com/lryong/timediff/blob/main/LICENSE) for details.

## Credits

- Inspired by [mergestat/timediff](https://github.com/mergestat/timediff)


