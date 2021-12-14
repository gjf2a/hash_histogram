# Overview

`HashHistogram` creates histograms with keys of any hashable data type. Features include:

* Check histogram count for a key.
* Check total histogram counts across all keys.
* Provide all keys in descending ranked order.
* Find the mode of the histogram (i.e., an item with the largest number of counts)
* Find the mode of any `IntoIterator` type, bulding a `HashHistogram` as an intermediate step.

# Updates
* **0.6.1**: Added `bump_by()`.
* **0.6.0**: Refactored `mode()` and `mode_values()` so that they only return the mode, rather than both mode and count.
* **0.5.2**: Added `mode_values()`
* **0.5.1**: Fixed some documentation.
* **0.5**: Initial public release.

# License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.