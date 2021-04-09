# rbx_binary Changelog

## Unreleased
* Improve resiliency when encountering unknown property types ([#168][pr-168])

[pr-168]: https://github.com/rojo-rbx/rbx-dom/pull/168

## 0.5.0 (2019-12-18)
0.5.0 is intended to be mostly API-compatible with previous rbx_binary releases because it makes a lot of foundational changes. 0.6.0 will break the rbx_binary API significantly.

* Rewrote crate from ground-up using understanding gained from rbx_xml
* Added support for using reflection information, improving content compatibility
* Improved performance by up to 30% for some files
* Improved instrumentation using the `log` crate
* Improved the crate's error types ([#48](https://github.com/rojo-rbx/rbx-dom/issues/48))
* Fixed panics in many cases, instead returning an error ([#26](https://github.com/rojo-rbx/rbx-dom/issues/26))
* Fixed handling unknown `BinaryString` values ([#49](https://github.com/rojo-rbx/rbx-dom/issues/49))

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
