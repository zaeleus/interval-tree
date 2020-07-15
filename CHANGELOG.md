# Changelog

## Unreleased

### Changed

  * Intersections include the endpoints, e.g., [0, 1] and [3, 5] now intersect
    [1, 3].

  * `Entry` fields `key` and `value` are now private. Use `entry.key()` and
    `entry.get()`, respectively, instead.

## 0.2.0 - 2020-05-08

### Changed

  * Keys changed from `Range` to `RangeInclusive`. The ranges used as keys were
    always treated as closed intervals, so using `RangeInclusive` makes that
    more obvious. Change usages of `..` to `..=`.

### Deprecated

  * `Entry` fields `key` and `value` are no longer public. Use `entry.key()`
    and `entry.get()`, respectively, instead.

### Fixed

  * The tree balance factor is calculated using the same primitive data type as
    the height. This avoids any casts that may possibly wrap.

## 0.1.0 - 2019-11-13

  * Initial release
