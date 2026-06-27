# timediff

[![crates.io](https://img.shields.io/crates/v/timediff.svg)](https://crates.io/crates/timediff)
[![docs.rs](https://docs.rs/timediff/badge.svg)](https://docs.rs/timediff)
[![license](https://img.shields.io/crates/l/timediff.svg)](https://github.com/lryong/timediff/blob/main/LICENSE)

Human-friendly relative time for Rust, based on Day.js-inspired thresholds.

`timediff` turns duration strings and `std::time::Duration` values into UI-friendly copy such as `a minute ago`, `in 2 hours`, or `几秒前`.
It is intentionally small, focused, and easy to drop into applications that need readable relative-time text without pulling in a large date-time toolkit.

## Why timediff?

- Day.js-inspired threshold behavior for product copy that feels familiar to frontend teams
- Works with both duration strings and `std::time::Duration`
- Produces natural relative-time phrases for past and future timestamps
- Includes built-in locale support for common product-facing languages
- Keeps scope small and dependency surface light

## When timediff is a good fit

- You want relative time like `2 minutes ago` or `in a month`
- You already use Day.js semantics in the frontend and want similar wording on the backend
- You need a small crate focused on readable relative-time output
- You want simple locale switching without designing a formatting system from scratch

## When timediff is not the best fit

- You need a full date-time toolkit
- You need calendar-aware month and year arithmetic
- You need `chrono` or `time` integrations in the public API today
- You want a highly configurable formatter with many output modes right now

## Install

```toml
[dependencies]
timediff = "0.2.3"
```

Or use:

```bash
cargo add timediff
```

## Quick Start

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

## Why not just use `timeago`?

`timeago` is a strong crate and already has broader ecosystem recognition.
`timediff` is a better fit when you specifically want a smaller crate with Day.js-inspired threshold behavior and direct support for both duration strings and `std::time::Duration`.

| Need | Prefer `timediff` | Prefer `timeago` |
| --- | --- | --- |
| Day.js-inspired threshold ranges | Yes | Maybe |
| Direct human-readable duration-string input | Yes | Not the primary path |
| Small, focused relative-time crate | Yes | Yes |
| Broader ecosystem maturity and adoption | Maybe | Yes |
| More established formatting surface | Maybe | Yes |

## Current Public API

The current API is intentionally small:

- `TimeDiff::to_diff(String)` for human-readable duration strings such as `-10s` or `2h`
- `TimeDiff::to_diff_duration(Duration)` for `std::time::Duration`
- `.locale(String)` to switch locale
- `.parse()` to generate the final relative-time string

Example:

```rust
use timediff::TimeDiff;

let mut diff = TimeDiff::to_diff(String::from("-10m"));
diff.locale(String::from("en-US")).unwrap();

assert_eq!(diff.parse().unwrap(), "10 minutes ago");
```

## Supported Locales

The current public API supports these locales:

| Locale | Status |
| --- | --- |
| `en-US` | Supported |
| `zh-CN` | Supported |
| `de-DE` | Supported |
| `ru-RU` | Supported |
| `tr-TR` | Supported |

## Design Principles

- Friendly default output over excessive configuration
- Small surface area over feature sprawl
- Predictable threshold behavior over surprising edge cases
- Practical localization over one-language-only formatting
- A crate that feels good to use in product code

## Scope And Status

`timediff` is stable in scope and intentionally focused on one job: formatting relative time from duration inputs.

Current scope includes:

- human-readable relative-time output
- `std::time::Duration` support
- locale switching
- threshold behavior based on ranges defined by [Day.js](https://day.js.org/docs/en/display/from-now)

Current non-goals for the public API include:

- full date-time handling
- calendar-accurate month calculations
- `chrono` integration
- `time` crate integration
- custom threshold presets

## Reference Output

The following examples show default output behavior.
These values come from the current test suite and are useful as a quick threshold reference.

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


