# rbx_binary Changelog

## Unreleased
* Rewrote crate from ground-up using understanding gained from rbx_xml
* Improved deserialization performance by up to 30% for some files
* Fixed many cases where rbx_binary would previously panic
* Added support for using reflection information, improving content compatibility
* Improved instrumentation using the `log` crate
* Improved the crate's error types using the `snafu` crate

## 0.4.1 (2019-05-29)
* Fixed bad interaction with rbx_dom_weak 1.6.0 causing instances to go missing.

## 0.4.0 (2019-03-01)
* Updated to `rbx_dom_weak` 1.0

## 0.3.0 (2019-02-14)
* Updated `rbx_tree` dependency to `rbx_dom_weak` 0.3.0

## 0.2.0 (2019-01-25)
* Updated `rbx_tree` dependency to 0.2.0

## 0.1.0
* Initial release
* Supports `String` and `Bool` types