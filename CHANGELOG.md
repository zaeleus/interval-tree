# Changelog

## Unreleased

## Changed

  * Keys changed from `Range` to `RangeInclusive`. The ranges used as keys were
    always treated as closed intervals, so using `RangeInclusive` makes that
    more obvious. Change usages of `..` to `..=`.

### Deprecated

  * `Entry` fields `key` and `value` are no longer public. Use `entry.key()`
    and `entry.get()`, respectively, instead.

## 0.1.0 - 2019-11-13

  * Initial release
