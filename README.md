timediff
========

**Status: stable**

[Documentation](https://docs.rs/timediff) |
[Github](https://github.com/lryong/timediff) |
[Crate](https://crates.io/crates/timediff)

Features:

* Parses and prints human readable, relative time differences. 
* Parses `std::time::Duration` to a human readable, relative time differences string.
* Parsed result is based on ranges defined in the [Day.js](https://day.js.org/docs/en/display/from-now)

Here are examples of durations and their corresponding string outputs (taken from test output), using default options:

```
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

License
=======

This package is licensed under MIT license. See [LICENSE](https://github.com/lryong/timediff/blob/main/LICENSE) for details.

Credits
=======

- Inspired by[mergestat/timediff](https://github.com/mergestat/timediff)


